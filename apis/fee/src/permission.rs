use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// CONFIGURATION Permission
const CONFIGURATION_RESOURCE: &str = "FEE_CONFIGURATION";

define_resource_perms! {
    CanCreateConfiguration => (CREATE, CONFIGURATION_RESOURCE),
    CanReadConfiguration => (READ, CONFIGURATION_RESOURCE),
    CanUpdateConfiguration => (UPDATE, CONFIGURATION_RESOURCE),
    CanDeleteConfiguration => (DELETE, CONFIGURATION_RESOURCE)
}
