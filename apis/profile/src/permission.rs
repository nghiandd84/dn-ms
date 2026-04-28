use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// PROFILE Permission
const PROFILE_RESOURCE: &str = "PROFILE_PROFILE";

define_resource_perms! {
    CanCreateProfile => (CREATE, PROFILE_RESOURCE),
    CanReadProfile => (READ, PROFILE_RESOURCE),
    CanUpdateProfile => (UPDATE, PROFILE_RESOURCE),
    CanDeleteProfile => (DELETE, PROFILE_RESOURCE)
}

// PREFERENCE Permission
const PREFERENCE_RESOURCE: &str = "PROFILE_PREFERENCE";

define_resource_perms! {
    CanCreatePreference => (CREATE, PREFERENCE_RESOURCE),
    CanReadPreference => (READ, PREFERENCE_RESOURCE),
    CanUpdatePreference => (UPDATE, PREFERENCE_RESOURCE),
    CanDeletePreference => (DELETE, PREFERENCE_RESOURCE)
}

// SOCIAL_LINK Permission
const SOCIAL_LINK_RESOURCE: &str = "PROFILE_SOCIAL_LINK";

define_resource_perms! {
    CanCreateSocialLink => (CREATE, SOCIAL_LINK_RESOURCE),
    CanReadSocialLink => (READ, SOCIAL_LINK_RESOURCE),
    CanUpdateSocialLink => (UPDATE, SOCIAL_LINK_RESOURCE),
    CanDeleteSocialLink => (DELETE, SOCIAL_LINK_RESOURCE)
}
