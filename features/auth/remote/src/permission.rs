use std::vec;

use features_auth_model::{permission::PermissionData};
use shared_shared_macro::RemoteService;

#[derive(Debug, RemoteService)]
#[remote(name(auth_service))]
pub struct PermissionService {}

impl PermissionService {
    pub async fn get_roles_by_service_name<'a>(service_key: String) -> HashMap<String, Vec<PermissionData>> {
        let permission_endpoint = std::env::var("AUTH_ROLES_ENDPOINT")
            .expect("AUTH_ROLES_ENDPOINT must be set");
        let permission_baggage_header = std::env::var("AUTH_ROLES_BAGGAGE_HEADER")
            .expect("AUTH_ROLES_BAGGAGE_HEADER must be set");
        let mut headers = HashMap::new();
        headers.insert("baggage".to_string(), permission_baggage_header);
        debug!(
            "Calling permission service at {} with headers: {:?}",
            permission_endpoint, headers
        );
        let page = 1;
        let page_size = 20;
        let permission_endpoint = format!(
            "{}?resource=sw|{}&page={}&page_size={}",
            permission_endpoint, service_key, page, page_size
        );
        let res = Self::call_api(permission_endpoint, reqwest::Method::GET, None, headers).await;
        if res.is_err() {
            let err_msg = res.err().unwrap();
            debug!("Error calling permission service: {}", err_msg);
            return HashMap::new();
        }
        let res = res.unwrap();
        debug!("Permission service response: {:?}", res);

        let permissions = res.get("result");
        if permissions.is_none() {
            println!("Response body does not contain permissions");
            return HashMap::new();
        }
        let permissions = permissions.unwrap();
        let perms: Vec<PermissionData> =
            serde_json::from_value(permissions.clone()).unwrap_or_else(|_| vec![]);

        let mut role_permissions: HashMap<String, Vec<PermissionData>> = HashMap::new();
        // for perm in perms {
        //     role_permissions.entry(perm.role.clone()).or_insert_with(Vec::new).push(perm);
        // }

        role_permissions

    }
}
