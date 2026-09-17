use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// BOOKING Permission
const BOOKING_RESOURCE: &str = "BOOKING:BOOKING";

define_resource_perms! {
    CanCreateBooking => (CREATE, BOOKING_RESOURCE),
    CanReadBooking => (READ, BOOKING_RESOURCE),
    CanUpdateBooking => (UPDATE, BOOKING_RESOURCE),
    CanDeleteBooking => (DELETE, BOOKING_RESOURCE)
}

// ITEM Permission (booking line items: seats, tables, rooms, cars, slots, ...)
const ITEM_RESOURCE: &str = "BOOKING:ITEM";

define_resource_perms! {
    CanCreateItem => (CREATE, ITEM_RESOURCE),
    CanReadItem => (READ, ITEM_RESOURCE),
    CanUpdateItem => (UPDATE, ITEM_RESOURCE),
    CanDeleteItem => (DELETE, ITEM_RESOURCE)
}

// GUEST BOOKING Permission (admin management of guest bookings)
const GUEST_BOOKING_RESOURCE: &str = "BOOKING:GUEST_BOOKING";

define_resource_perms! {
    CanCreateGuestBooking => (CREATE, GUEST_BOOKING_RESOURCE),
    CanReadGuestBooking => (READ, GUEST_BOOKING_RESOURCE),
    CanUpdateGuestBooking => (UPDATE, GUEST_BOOKING_RESOURCE),
    CanDeleteGuestBooking => (DELETE, GUEST_BOOKING_RESOURCE)
}
