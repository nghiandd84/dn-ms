use rs_consul::{
    Config, Consul, DeregisterEntityPayload, RegisterEntityCheck, RegisterEntityPayload,
    RegisterEntityService,
};

use std::collections::HashMap;
use tracing::info;

static NODE_ID: &str = "dn-ms";

pub fn get_consul_client() -> Result<Consul, Box<dyn std::error::Error>> {
    let consul_config = Config::from_env();

    let consul = Consul::new(consul_config);

    info!("Consul client initialized successfully",);

    Ok(consul)
}

pub async fn register_service(
    consul: &Consul,
    service_name: &str,
    instance_id: &str,
    service_ip: &str,
    service_port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let service_address = format!("http://{}:{}", service_ip, service_port);

    let mut definition: HashMap<String, String> = HashMap::new();
    definition.insert(
        "http".to_string(),
        format!("{}/healthchecker", service_address),
    );
    definition.insert("method".to_string(), "GET".to_string());
    definition.insert("tls_skip_verify".to_string(), "false".to_string());
    definition.insert("interval".to_string(), "10s".to_string());
    definition.insert("timeout".to_string(), "1s".to_string());
    definition.insert(
        "deregister_critical_service_after".to_string(),
        "1m".to_string(),
    );
    let check = RegisterEntityCheck {
        CheckID: Some(format!("service:{}:health", instance_id)),
        Node: None,
        Name: "HTTP Health Check".to_string(),
        Notes: Some("Checks if the service is healthy".to_string()),
        Status: Some("passing".to_string()),
        ServiceID: Some(instance_id.to_string()),
        Definition: definition,
    };

    let payload = RegisterEntityPayload {
        ID: None,
        Node: NODE_ID.to_string(),
        Address: service_ip.to_string(), //server address
        Datacenter: None,
        TaggedAddresses: Default::default(),
        NodeMeta: Default::default(),
        Service: Some(RegisterEntityService {
            ID: Some(instance_id.to_string()),
            Service: service_name.to_string(),
            Tags: vec![service_name.to_string(), NODE_ID.to_string()],
            TaggedAddresses: Default::default(),
            Meta: Default::default(),
            Port: Some(service_port),
            Namespace: None,
        }),
        Checks: vec![check],
        SkipNodeUpdate: None,
    };

    consul.register_entity(&payload).await.unwrap();

    info!(
        "Service '{}'  registered with instance '{}' successfully!",
        service_name, instance_id
    );
    Ok(())
}

pub async fn deregister_service(
    consul: &Consul,
    service_name: &str,
    instance_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Deregistering service '{}' with instance '{}'...",
        service_name, instance_id
    );

    let payload = DeregisterEntityPayload {
        Node: Some(NODE_ID.to_string()),
        Datacenter: None,
        CheckID: Some(format!("service:{}:health", instance_id)),
        ServiceID: Some(instance_id.to_string()),
        Namespace: None,
    };
    consul.deregister_entity(&payload).await.unwrap();
    Ok(())
}
