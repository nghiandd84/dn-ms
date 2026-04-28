use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// PAYMENT Permission
const PAYMENT_RESOURCE: &str = "PAYMENT_PAYMENT";

define_resource_perms! {
    CanCreatePayment => (CREATE, PAYMENT_RESOURCE),
    CanReadPayment => (READ, PAYMENT_RESOURCE),
    CanUpdatePayment => (UPDATE, PAYMENT_RESOURCE),
    CanDeletePayment => (DELETE, PAYMENT_RESOURCE)
}

// METHOD Permission
const METHOD_RESOURCE: &str = "PAYMENT_METHOD";

define_resource_perms! {
    CanCreateMethod => (CREATE, METHOD_RESOURCE),
    CanReadMethod => (READ, METHOD_RESOURCE),
    CanUpdateMethod => (UPDATE, METHOD_RESOURCE),
    CanDeleteMethod => (DELETE, METHOD_RESOURCE)
}

// ATTEMPT Permission
const ATTEMPT_RESOURCE: &str = "PAYMENT_ATTEMPT";

define_resource_perms! {
    CanCreateAttempt => (CREATE, ATTEMPT_RESOURCE),
    CanReadAttempt => (READ, ATTEMPT_RESOURCE),
    CanUpdateAttempt => (UPDATE, ATTEMPT_RESOURCE),
    CanDeleteAttempt => (DELETE, ATTEMPT_RESOURCE)
}

// METHOD_LIMIT Permission
const METHOD_LIMIT_RESOURCE: &str = "PAYMENT_METHOD_LIMIT";

define_resource_perms! {
    CanCreateMethodLimit => (CREATE, METHOD_LIMIT_RESOURCE),
    CanReadMethodLimit => (READ, METHOD_LIMIT_RESOURCE),
    CanUpdateMethodLimit => (UPDATE, METHOD_LIMIT_RESOURCE),
    CanDeleteMethodLimit => (DELETE, METHOD_LIMIT_RESOURCE)
}
