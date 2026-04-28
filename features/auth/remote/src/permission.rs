use features_auth_model::permission::PermissionData;
use shared_shared_data_core::filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam};
use shared_shared_macro::RemoteService;

#[derive(Debug, RemoteService)]
#[remote(name(auth_service))]
pub struct PermissionService {}

impl PermissionService {
    pub async fn get_roles_by_service_name<'a>(
        service_key: String,
    ) -> HashMap<String, Vec<PermissionData>> {
        let permission_endpoint =
            std::env::var("AUTH_ROLES_ENDPOINT").expect("AUTH_ROLES_ENDPOINT must be set");
        let permission_baggage_header = std::env::var("AUTH_ROLES_BAGGAGE_HEADER")
            .expect("AUTH_ROLES_BAGGAGE_HEADER must be set");
        let mut headers = HashMap::new();
        headers.insert("baggage".to_string(), permission_baggage_header);
        debug!(
            "Calling permission service at {} with headers: {:?}",
            permission_endpoint, headers
        );

        let condition = FilterCondition::leaf(FilterEnum::String(FilterParam {
            name: "permissions[resource]".to_string(),
            operator: FilterOperator::StartWith,
            value: Some(service_key.clone()),
            raw_value: service_key,
        }));

        let query_string = condition.to_query_string();
        let page_size = 20;
        let mut page = 1u64;
        let mut role_permissions: HashMap<String, Vec<PermissionData>> = HashMap::new();

        loop {
            let url = format!(
                "{}?{}&includes=permissions&page={}&page_size={}",
                permission_endpoint, query_string, page, page_size
            );
            let res = Self::call_api(url, reqwest::Method::GET, None, headers.clone()).await;
            if let Err(err_msg) = res {
                debug!("Error calling permission service: {}", err_msg);
                return role_permissions;
            }
            let res = res.unwrap();
            debug!("Permission service response page {}: {:?}", page, res);

            let total_page = res.get("total_page").and_then(|v| v.as_u64()).unwrap_or(0);
            let roles = match res.get("result") {
                Some(r) => r,
                None => break,
            };

            if let Some(arr) = roles.as_array() {
                for role in arr {
                    let name = role.get("name").and_then(|v| v.as_str()).unwrap_or_default();
                    if name.is_empty() {
                        continue;
                    }
                    let perms: Vec<PermissionData> = role
                        .get("permissions")
                        .cloned()
                        .and_then(|v| serde_json::from_value(v).ok())
                        .unwrap_or_default();

                    role_permissions
                        .entry(name.to_string())
                        .or_default()
                        .extend(perms);
                }
            }

            if page >= total_page {
                break;
            }
            page += 1;
        }

        role_permissions
    }
}
