// use shared_shared_macro_rule::define_resource_perms;

use shared_shared_auth::{
    define_resource_perms,
    permission::{ADMIN, CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// BAKER Permission
const BAKER_RESOURCE: &str = "BAKERY_BAKER";

define_resource_perms! {
    CanDeleteBaker  => (DELETE, BAKER_RESOURCE),
    CanUpdateBaker  => (UPDATE, BAKER_RESOURCE),
    CanReadBaker => (READ, BAKER_RESOURCE),
    CanCreateBaker => (CREATE, BAKER_RESOURCE),
    CanManageBaker => (ADMIN, BAKER_RESOURCE)
}
