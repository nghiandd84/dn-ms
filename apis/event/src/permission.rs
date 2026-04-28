use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// EVENT Permission
const EVENT_RESOURCE: &str = "EVENT_EVENT";

define_resource_perms! {
    CanCreateEvent => (CREATE, EVENT_RESOURCE),
    CanReadEvent => (READ, EVENT_RESOURCE),
    CanUpdateEvent => (UPDATE, EVENT_RESOURCE),
    CanDeleteEvent => (DELETE, EVENT_RESOURCE)
}
