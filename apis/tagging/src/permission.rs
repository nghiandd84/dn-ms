use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

const TAG_GROUP_RESOURCE: &str = "TAGGING:TAG_GROUP";
const TAG_RESOURCE: &str = "TAGGING:TAG";
const ENTITY_TAG_RESOURCE: &str = "TAGGING:ENTITY_TAG";

define_resource_perms! {
    CanCreateTagGroup => (CREATE, TAG_GROUP_RESOURCE),
    CanReadTagGroup => (READ, TAG_GROUP_RESOURCE),
    CanUpdateTagGroup => (UPDATE, TAG_GROUP_RESOURCE),
    CanDeleteTagGroup => (DELETE, TAG_GROUP_RESOURCE),

    CanCreateTag => (CREATE, TAG_RESOURCE),
    CanReadTag => (READ, TAG_RESOURCE),
    CanUpdateTag => (UPDATE, TAG_RESOURCE),
    CanDeleteTag => (DELETE, TAG_RESOURCE),

    CanCreateEntityTag => (CREATE, ENTITY_TAG_RESOURCE),
    CanReadEntityTag => (READ, ENTITY_TAG_RESOURCE),
    CanDeleteEntityTag => (DELETE, ENTITY_TAG_RESOURCE)
}
