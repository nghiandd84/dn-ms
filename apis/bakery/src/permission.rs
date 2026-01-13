// use shared_shared_macro_rule::define_resource_perms;

use shared_shared_auth::{
    define_resource_perms,
    permission::{DELETE, READ, UPDATE},
    ResourcePermission,
};

// BAKER Permission
define_resource_perms! {
    CanDeleteBaker  => (DELETE, "BAKER"),
    CanUpdateBaker  => (UPDATE, "BAKER"),
    CanReadBaker => (READ, "BAKER")
}
