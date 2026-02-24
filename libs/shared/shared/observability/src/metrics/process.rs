use opentelemetry::{global, metrics::ObservableCounter, KeyValue};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;

#[derive(Clone)]
pub struct ProcessMetrics {
    _counter_registration: ObservableCounter<u64>,
    process_stats: Arc<Mutex<ProcessStats>>,
    name: String,
}

#[derive(Clone)]
pub struct ProcessStats {
    user_cpu_time: u64,
    system_cpu_time: u64,
    gc_runs: u64,
    allocated_objects: u64,
    loaded_features: u64,
}

impl ProcessMetrics {
    pub fn new(service_name: String) -> Self {
        let meter = global::meter("process-metrics");
        let process_stats = Arc::new(Mutex::new(ProcessStats {
            user_cpu_time: 0,
            system_cpu_time: 0,
            gc_runs: 0,
            allocated_objects: 0,
            loaded_features: 100, // Initial libraries loaded
        }));

        let stats_for_callback = process_stats.clone();

        let process_counter = meter
            .u64_observable_counter("process_resource_usage")
            .with_description("Process resource usage counters")
            .with_callback(move |observer| {
                if let Ok(stats) = stats_for_callback.try_lock() {
                    debug!("Observing process metrics: user_cpu_time={}, system_cpu_time={}, gc_runs={}, allocated_objects={}, loaded_features={}",
                        stats.user_cpu_time, stats.system_cpu_time, stats.gc_runs, stats.allocated_objects, stats.loaded_features);
                    observer.observe(
                        stats.user_cpu_time,
                        &[
                            KeyValue::new("cpu_type", "user"),
                            KeyValue::new("unit", "microseconds"),
                        ],
                    );

                    observer.observe(
                        stats.system_cpu_time,
                        &[
                            KeyValue::new("cpu_type", "system"),
                            KeyValue::new("unit", "microseconds"),
                        ],
                    );

                    observer.observe(
                        stats.gc_runs,
                        &[
                            KeyValue::new("resource", "gc_runs"),
                            KeyValue::new("unit", "count"),
                        ],
                    );

                    observer.observe(
                        stats.allocated_objects,
                        &[
                            KeyValue::new("resource", "allocated_objects"),
                            KeyValue::new("unit", "count"),
                        ],
                    );

                    observer.observe(
                        stats.loaded_features,
                        &[
                            KeyValue::new("resource", "loaded_features"),
                            KeyValue::new("unit", "count"),
                        ],
                    );
                }
            })
            .build();

        Self {
            _counter_registration: process_counter,
            process_stats,
            name: service_name,
        }
    }

    pub async fn update_stats(&self) {
        debug!("Updating process metrics for {}", self.name);
        let mut stats = self.process_stats.lock().await;

        // Simulate accumulating process metrics
        stats.user_cpu_time += rand::random::<u64>() % 1000 + 100;
        stats.system_cpu_time += rand::random::<u64>() % 500 + 50;
        stats.gc_runs += if rand::random::<u8>() % 10 == 0 { 1 } else { 0 };
        stats.allocated_objects += rand::random::<u64>() % 10000 + 1000;
        stats.loaded_features += if rand::random::<u8>() % 20 == 0 { 1 } else { 0 };
    }
}
