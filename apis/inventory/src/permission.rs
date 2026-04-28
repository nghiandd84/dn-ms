use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// SEAT Permission
const SEAT_RESOURCE: &str = "INVENTORY_SEAT";

define_resource_perms! {
    CanCreateSeat => (CREATE, SEAT_RESOURCE),
    CanReadSeat => (READ, SEAT_RESOURCE),
    CanUpdateSeat => (UPDATE, SEAT_RESOURCE),
    CanDeleteSeat => (DELETE, SEAT_RESOURCE)
}

// RESERVATION Permission
const RESERVATION_RESOURCE: &str = "INVENTORY_RESERVATION";

define_resource_perms! {
    CanCreateReservation => (CREATE, RESERVATION_RESOURCE),
    CanReadReservation => (READ, RESERVATION_RESOURCE),
    CanUpdateReservation => (UPDATE, RESERVATION_RESOURCE),
    CanDeleteReservation => (DELETE, RESERVATION_RESOURCE)
}
