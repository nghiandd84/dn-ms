pub mod process;

use crate::metrics::process::ProcessMetrics;

#[derive(Clone)]
pub struct StateMetrics {
    pub process_metrics: ProcessMetrics,
}

impl StateMetrics {
    pub fn new(service_name: String) -> Self {
        Self {
            process_metrics: ProcessMetrics::new(service_name),
        }
    }
}
