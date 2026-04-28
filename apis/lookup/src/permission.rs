use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, UPDATE},
    ResourcePermission,
};

const LOOKUP_TYPE_RESOURCE: &str = "LOOKUP_TYPE";
const LOOKUP_ITEM_RESOURCE: &str = "LOOKUP_ITEM";
const LOOKUP_ITEM_TRANSLATION_RESOURCE: &str = "LOOKUP_ITEM_TRANSLATION";

define_resource_perms! {
    CanCreateLookupType => (CREATE, LOOKUP_TYPE_RESOURCE),
    CanUpdateLookupType => (UPDATE, LOOKUP_TYPE_RESOURCE),
    CanDeleteLookupType => (DELETE, LOOKUP_TYPE_RESOURCE),

    CanCreateLookupItem => (CREATE, LOOKUP_ITEM_RESOURCE),
    CanUpdateLookupItem => (UPDATE, LOOKUP_ITEM_RESOURCE),
    CanDeleteLookupItem => (DELETE, LOOKUP_ITEM_RESOURCE),

    CanCreateLookupItemTranslation => (CREATE, LOOKUP_ITEM_TRANSLATION_RESOURCE),
    CanUpdateLookupItemTranslation => (UPDATE, LOOKUP_ITEM_TRANSLATION_RESOURCE),
    CanDeleteLookupItemTranslation => (DELETE, LOOKUP_ITEM_TRANSLATION_RESOURCE)
}
