use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// USER Permission
const USER_RESOURCE: &str = "AUTH_USER";

define_resource_perms! {
    CanReadUser => (READ, USER_RESOURCE),
    CanDeleteUser => (DELETE, USER_RESOURCE)
}

// ROLE Permission
const ROLE_RESOURCE: &str = "AUTH_ROLE";

define_resource_perms! {
    CanCreateRole => (CREATE, ROLE_RESOURCE),
    CanReadRole => (READ, ROLE_RESOURCE),
    CanUpdateRole => (UPDATE, ROLE_RESOURCE),
    CanDeleteRole => (DELETE, ROLE_RESOURCE)
}

// PERMISSION Permission
const PERMISSION_RESOURCE: &str = "AUTH_PERMISSION";

define_resource_perms! {
    CanCreatePermission => (CREATE, PERMISSION_RESOURCE),
    CanReadPermission => (READ, PERMISSION_RESOURCE),
    CanUpdatePermission => (UPDATE, PERMISSION_RESOURCE),
    CanDeletePermission => (DELETE, PERMISSION_RESOURCE)
}

// SCOPE Permission
const SCOPE_RESOURCE: &str = "AUTH_SCOPE";

define_resource_perms! {
    CanCreateScope => (CREATE, SCOPE_RESOURCE),
    CanReadScope => (READ, SCOPE_RESOURCE),
    CanUpdateScope => (UPDATE, SCOPE_RESOURCE),
    CanDeleteScope => (DELETE, SCOPE_RESOURCE)
}

// CLIENT Permission
const CLIENT_RESOURCE: &str = "AUTH_CLIENT";

define_resource_perms! {
    CanCreateClient => (CREATE, CLIENT_RESOURCE),
    CanReadClient => (READ, CLIENT_RESOURCE),
    CanUpdateClient => (UPDATE, CLIENT_RESOURCE),
    CanDeleteClient => (DELETE, CLIENT_RESOURCE)
}

// TOKEN Permission
const TOKEN_RESOURCE: &str = "AUTH_TOKEN";

define_resource_perms! {
    CanReadToken => (READ, TOKEN_RESOURCE)
}

// AUTH_CODE Permission
const AUTH_CODE_RESOURCE: &str = "AUTH_AUTH_CODE";

define_resource_perms! {
    CanCreateAuthCode => (CREATE, AUTH_CODE_RESOURCE),
    CanReadAuthCode => (READ, AUTH_CODE_RESOURCE),
    CanDeleteAuthCode => (DELETE, AUTH_CODE_RESOURCE)
}
