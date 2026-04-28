use shared_shared_auth::{
    define_resource_perms,
    permission::{ADMIN, CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// BAKER Permission
const BAKER_RESOURCE: &str = "BAKERY_BAKER";

define_resource_perms! {
    CanDeleteBaker  => (DELETE, BAKER_RESOURCE),
    CanUpdateBaker  => (UPDATE, BAKER_RESOURCE),
    CanReadBaker => (READ, BAKER_RESOURCE),
    CanCreateBaker => (CREATE, BAKER_RESOURCE),
    CanManageBaker => (ADMIN, BAKER_RESOURCE)
}

// BAKERY Permission
const BAKERY_RESOURCE: &str = "BAKERY_BAKERY";

define_resource_perms! {
    CanCreateBakery => (CREATE, BAKERY_RESOURCE),
    CanUpdateBakery => (UPDATE, BAKERY_RESOURCE),
    CanDeleteBakery => (DELETE, BAKERY_RESOURCE)
}

// CAKE Permission
const CAKE_RESOURCE: &str = "BAKERY_CAKE";

define_resource_perms! {
    CanCreateCake => (CREATE, CAKE_RESOURCE),
    CanUpdateCake => (UPDATE, CAKE_RESOURCE),
    CanDeleteCake => (DELETE, CAKE_RESOURCE)
}

// CAKE_BAKER Permission
const CAKE_BAKER_RESOURCE: &str = "BAKERY_CAKE_BAKER";

define_resource_perms! {
    CanCreateCakeBaker => (CREATE, CAKE_BAKER_RESOURCE),
    CanUpdateCakeBaker => (UPDATE, CAKE_BAKER_RESOURCE),
    CanDeleteCakeBaker => (DELETE, CAKE_BAKER_RESOURCE)
}

// CUSTOMER Permission
const CUSTOMER_RESOURCE: &str = "BAKERY_CUSTOMER";

define_resource_perms! {
    CanCreateCustomer => (CREATE, CUSTOMER_RESOURCE),
    CanUpdateCustomer => (UPDATE, CUSTOMER_RESOURCE),
    CanDeleteCustomer => (DELETE, CUSTOMER_RESOURCE)
}

// ORDER Permission
const ORDER_RESOURCE: &str = "BAKERY_ORDER";

define_resource_perms! {
    CanCreateOrder => (CREATE, ORDER_RESOURCE),
    CanUpdateOrder => (UPDATE, ORDER_RESOURCE),
    CanDeleteOrder => (DELETE, ORDER_RESOURCE)
}

// LINEITEM Permission
const LINEITEM_RESOURCE: &str = "BAKERY_LINEITEM";

define_resource_perms! {
    CanCreateLineitem => (CREATE, LINEITEM_RESOURCE),
    CanUpdateLineitem => (UPDATE, LINEITEM_RESOURCE),
    CanDeleteLineitem => (DELETE, LINEITEM_RESOURCE)
}
