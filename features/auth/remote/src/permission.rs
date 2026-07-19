use features_auth_model::permission::PermissionData;
use shared_shared_app::state::FieldPermissionEntry;
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
            value: Some(format!("{}:", service_key)),
            raw_value: format!("{}:", service_key),
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

    /// Fetch field-level permissions for all roles that have entries for this service.
    /// Calls GET /field-permissions?resource=sw|SERVICE_KEY: and groups by role name.
    pub async fn get_field_permissions_by_service_name(
        service_key: String,
    ) -> HashMap<String, Vec<FieldPermissionEntry>> {
        let field_permission_endpoint = std::env::var("AUTH_FIELD_PERMISSIONS_ENDPOINT")
            .unwrap_or_else(|_| {
                // Fallback: derive from AUTH_ROLES_ENDPOINT by replacing /roles with /field-permissions
                let roles_endpoint =
                    std::env::var("AUTH_ROLES_ENDPOINT").unwrap_or_default();
                roles_endpoint.replace("/roles", "/field-permissions")
            });
        let permission_baggage_header = std::env::var("AUTH_ROLES_BAGGAGE_HEADER")
            .expect("AUTH_ROLES_BAGGAGE_HEADER must be set");

        let mut headers = HashMap::new();
        headers.insert("baggage".to_string(), permission_baggage_header);

        let condition = FilterCondition::leaf(FilterEnum::String(FilterParam {
            name: "resource".to_string(),
            operator: FilterOperator::StartWith,
            value: Some(format!("{}:", service_key)),
            raw_value: format!("{}:", service_key),
        }));

        let query_string = condition.to_query_string();
        let page_size = 20;
        let mut page = 1u64;
        let mut role_field_permissions: HashMap<String, Vec<FieldPermissionEntry>> = HashMap::new();

        loop {
            let url = format!(
                "{}?{}&includes=role&page={}&page_size={}",
                field_permission_endpoint, query_string, page, page_size
            );

            let res = Self::call_api(url, reqwest::Method::GET, None, headers.clone()).await;
            if let Err(err_msg) = res {
                debug!("Error calling field permission service: {}", err_msg);
                return role_field_permissions;
            }
            let res = res.unwrap();
            debug!("Field permission service response page {}: {:?}", page, res);

            let total_page = res.get("total_page").and_then(|v| v.as_u64()).unwrap_or(0);
            let results = match res.get("result") {
                Some(r) => r,
                None => break,
            };

            if let Some(arr) = results.as_array() {
                for entry in arr {
                    // Extract role_name from the included role relation or role_id lookup
                    let role_name = entry
                        .get("role")
                        .and_then(|r| r.get("name"))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();

                    if role_name.is_empty() {
                        continue;
                    }

                    let resource = entry
                        .get("resource")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string();
                    let action = entry
                        .get("action")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as u32;
                    let fields: Vec<String> = entry
                        .get("fields")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default();

                    if resource.is_empty() || fields.is_empty() {
                        continue;
                    }

                    role_field_permissions
                        .entry(role_name.to_string())
                        .or_default()
                        .push(FieldPermissionEntry {
                            resource,
                            action,
                            fields,
                        });
                }
            }

            if page >= total_page {
                break;
            }
            page += 1;
        }

        role_field_permissions
    }
}
