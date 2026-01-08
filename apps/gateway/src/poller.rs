use async_trait::async_trait;
use pingora::server::ShutdownWatch;
use pingora::services::background::BackgroundService;
use std::time::Duration;
use tokio::time::interval;
use tracing::debug;

use shared_shared_app::discovery::get_consul_client;

use features_auth_remote::TokenService;

pub struct ApiPoller {
    pub interval_duration: Duration,
}

#[async_trait]
impl BackgroundService for ApiPoller {
    async fn start(&self, mut shutdown: ShutdownWatch) {
        let mut ticker = interval(self.interval_duration);

        loop {
            tokio::select! {
                // Wait for the next interval tick
                _ = ticker.tick() => {
                    self.call_external_api().await;
                }
                // Stop the loop if the server is shutting down
                _ = shutdown.changed() => {
                    debug!("Shutting down background API poller");
                    break;
                }
            }
        }
    }
}

impl ApiPoller {
    async fn call_external_api(&self) {
        // Your logic to call an API (e.g., using reqwest or Pingora's internal HTTP client)
        debug!("Calling API to refresh data...");
        let consul_client = get_consul_client().unwrap();
        TokenService::update_remote(&consul_client).await;
    }
}
