use std::sync::Arc;

use crate::{
    config::source_config::find_router_config_or_err,
    error::{DakiaError, DakiaResult},
    gateway::{interceptor::Phase, state::GatewayStateStore},
    proxy::http::helpers::get_inet_addr_from_backend,
};

use super::{
    helpers::is_valid_ds_host,
    session::{self},
    DakiaHttpGatewayCtx,
};
use async_trait::async_trait;
use http::StatusCode;
use pingora::{
    prelude::HttpPeer,
    proxy::{ProxyHttp, Session},
    Error, ErrorSource,
    ErrorType::HTTPStatus,
};
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::FailToProxy;

#[derive(Clone)]
pub struct Proxy {
    gateway_state_store: Arc<GatewayStateStore>,
}

impl Proxy {
    pub async fn build(gateway_state_store: Arc<GatewayStateStore>) -> DakiaResult<Proxy> {
        let proxy = Proxy {
            gateway_state_store,
        };

        Ok(proxy)
    }
}

#[async_trait]
impl ProxyHttp for Proxy {
    type CTX = DakiaHttpGatewayCtx;
    fn new_ctx(&self) -> Self::CTX {
        let gateway_state = self.gateway_state_store.get_state();
        DakiaHttpGatewayCtx::new(gateway_state)
    }

    async fn early_request_filter(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<(), Box<Error>> {
        let mut session = session::Session::build(Phase::Init, _session, _ctx);
        session.execute_interceptors_phase().await?;
        Ok(())
    }

    async fn request_filter(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<bool, Box<Error>> {
        let mut session = session::Session::build(Phase::RequestFilter, _session, _ctx);
        let host = session.ds_req_header("host")?;

        match host {
            Some(host) => {
                let is_valid_ds_host = is_valid_ds_host(
                    &session.ctx().gateway_state.gateway_config(),
                    &self.gateway_state_store.get_state().pattern_registry(),
                    host,
                )
                .await?;

                if !is_valid_ds_host {
                    session.set_res_status(StatusCode::FORBIDDEN);
                    session.flush_ds_res_header().await?;
                    return Ok(true);
                }
            }

            None => {
                // host is required header
                session.set_res_status(StatusCode::BAD_REQUEST);
                session.flush_ds_res_header().await?;
                return Ok(true);
            }
        };

        Ok(session.execute_interceptors_phase().await?)
    }

    async fn proxy_upstream_filter(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<bool, Box<Error>>
    where
        Self::CTX: Send + Sync,
    {
        let mut session = session::Session::build(Phase::UpstreamProxyFilter, _session, _ctx);
        Ok(!session.execute_interceptors_phase().await?)
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        let session = session::Session::build(Phase::UpstreamProxyFilter, _session, _ctx);

        let router_config = find_router_config_or_err(&session)?;
        let upstream_name = &router_config.upstream;

        let gateway_state = self.gateway_state_store.get_state();
        let lb_registry = gateway_state.lb_registry();

        let mut lb = lb_registry.get(&upstream_name).await?;
        lb = match lb {
            None => lb_registry.get("default").await?,
            Some(lb) => Some(lb),
        };

        let lb = lb.ok_or(DakiaError::i_explain(format!(
            "load balacer not found for upstream {upstream_name}"
        )))?;

        let backend = lb.select(b"", 256).unwrap(); // hash doesn't matter

        let inet_address = get_inet_addr_from_backend(&backend);

        let upstream_node_config = gateway_state
            .gateway_config()
            .find_upstream_config_or_err(upstream_name, true)
            .map(|a| a.find_upstream_node_config_or_err(inet_address))??;

        let tls = upstream_node_config.tls;
        let sni = upstream_node_config.clone().sni.unwrap_or("".to_string());

        let peer = Box::new(HttpPeer::new(backend.addr, tls, sni));

        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        _upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<(), pingora_core::BError>
    where
        Self::CTX: Send + Sync,
    {
        let mut session = session::Session::build(Phase::PreUpstreamRequest, _session, _ctx);
        session.upstream_request(_upstream_request);
        session.execute_interceptors_phase().await?;
        session.flush_us_req_header()?;

        Ok(())
    }

    async fn fail_to_proxy(
        &self,
        _session: &mut Session,
        e: &Error,
        _ctx: &mut Self::CTX,
    ) -> FailToProxy
    where
        Self::CTX: Send + Sync,
    {
        let code = match e.etype() {
            HTTPStatus(code) => *code,
            _ => {
                match e.esource() {
                    ErrorSource::Upstream => 502,
                    ErrorSource::Downstream => {
                        match e.etype() {
                            pingora::ErrorType::WriteError
                            | pingora::ErrorType::ReadError
                            | pingora::ErrorType::ConnectionClosed => {
                                /* conn already dead */
                                0
                            }
                            _ => 400,
                        }
                    }
                    ErrorSource::Internal | ErrorSource::Unset => 500,
                }
            }
        };

        if code > 0 {
            let mut session = session::Session::build(Phase::PreDownstreamResponse, _session, _ctx);
            let status_code = StatusCode::from_u16(code).unwrap();
            session.set_res_status(status_code);
            session.flush_ds_res_header().await.unwrap();
        }

        // code

        FailToProxy {
            can_reuse_downstream: false,
            error_code: code,
        }
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        _upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<(), Box<Error>>
    where
        Self::CTX: Send + Sync,
    {
        let mut session = session::Session::build(Phase::PostUpstreamResponse, _session, _ctx);
        session.upstream_response(_upstream_response);
        session.execute_interceptors_phase().await?;
        session.flush_ds_res_header().await?;
        Ok(())
    }
}
