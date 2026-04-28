use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// PROJECT Permission
const PROJECT_RESOURCE: &str = "TRANSLATION_PROJECT";

define_resource_perms! {
    CanCreateProject => (CREATE, PROJECT_RESOURCE),
    CanReadProject => (READ, PROJECT_RESOURCE),
    CanUpdateProject => (UPDATE, PROJECT_RESOURCE),
    CanDeleteProject => (DELETE, PROJECT_RESOURCE)
}

// KEY Permission
const KEY_RESOURCE: &str = "TRANSLATION_KEY";

define_resource_perms! {
    CanCreateKey => (CREATE, KEY_RESOURCE),
    CanReadKey => (READ, KEY_RESOURCE),
    CanUpdateKey => (UPDATE, KEY_RESOURCE),
    CanDeleteKey => (DELETE, KEY_RESOURCE)
}

// TAG Permission
const TAG_RESOURCE: &str = "TRANSLATION_TAG";

define_resource_perms! {
    CanCreateTag => (CREATE, TAG_RESOURCE),
    CanReadTag => (READ, TAG_RESOURCE),
    CanUpdateTag => (UPDATE, TAG_RESOURCE),
    CanDeleteTag => (DELETE, TAG_RESOURCE)
}

// VERSION Permission
const VERSION_RESOURCE: &str = "TRANSLATION_VERSION";

define_resource_perms! {
    CanCreateVersion => (CREATE, VERSION_RESOURCE),
    CanReadVersion => (READ, VERSION_RESOURCE),
    CanUpdateVersion => (UPDATE, VERSION_RESOURCE),
    CanDeleteVersion => (DELETE, VERSION_RESOURCE)
}
