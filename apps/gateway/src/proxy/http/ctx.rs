use std::sync::Arc;

use crate::gateway::state::GatewayState;

use super::HeaderBuffer;

pub struct DakiaHttpGatewayCtx {
    pub gateway_state: Arc<GatewayState>,
    pub ds_res_header_buffer: HeaderBuffer,
    pub us_req_header_buffer: HeaderBuffer,
}

impl DakiaHttpGatewayCtx {
    pub fn new(gateway_state: Arc<GatewayState>) -> DakiaHttpGatewayCtx {
        DakiaHttpGatewayCtx {
            gateway_state,
            ds_res_header_buffer: HeaderBuffer::new(),
            us_req_header_buffer: HeaderBuffer::new(),
        }
    }
}
