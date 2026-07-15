use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

const URL_RESOURCE: &str = "URL_SHORTENER:URL";
const ANALYTICS_RESOURCE: &str = "URL_SHORTENER:ANALYTICS";
const API_KEY_RESOURCE: &str = "URL_SHORTENER:API_KEY";

define_resource_perms! {
    CanCreateUrl => (CREATE, URL_RESOURCE),
    CanUpdateUrl => (UPDATE, URL_RESOURCE),
    CanDeleteUrl => (DELETE, URL_RESOURCE),
    CanViewAnalytics => (READ, ANALYTICS_RESOURCE),
    CanManageApiKeys => (CREATE, API_KEY_RESOURCE),
    CanDeleteApiKey => (DELETE, API_KEY_RESOURCE)
}
