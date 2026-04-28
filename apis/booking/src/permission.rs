use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// BOOKING Permission
const BOOKING_RESOURCE: &str = "BOOKING_BOOKING";

define_resource_perms! {
    CanCreateBooking => (CREATE, BOOKING_RESOURCE),
    CanReadBooking => (READ, BOOKING_RESOURCE),
    CanUpdateBooking => (UPDATE, BOOKING_RESOURCE),
    CanDeleteBooking => (DELETE, BOOKING_RESOURCE)
}

// SEAT Permission
const SEAT_RESOURCE: &str = "BOOKING_SEAT";

define_resource_perms! {
    CanCreateSeat => (CREATE, SEAT_RESOURCE),
    CanReadSeat => (READ, SEAT_RESOURCE),
    CanUpdateSeat => (UPDATE, SEAT_RESOURCE),
    CanDeleteSeat => (DELETE, SEAT_RESOURCE)
}
