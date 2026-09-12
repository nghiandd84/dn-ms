use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use features_booking_entities::booking_capacity::{
    BookingCapacityForCreateDto, Model, ModelOptionDto,
};

/// CAPACITY-mode input block supplied inside a create-booking request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BookingCapacityInput {
    pub container_id: Uuid,
    #[serde(default = "default_quantity")]
    pub quantity: i32,
}

fn default_quantity() -> i32 {
    1
}

impl BookingCapacityInput {
    pub fn to_dto(&self, booking_id: Uuid) -> BookingCapacityForCreateDto {
        BookingCapacityForCreateDto {
            booking_id,
            container_id: self.container_id,
            quantity: self.quantity,
        }
    }
}

#[derive(Serialize, Debug, ToSchema, Default)]
pub struct BookingCapacityData {
    pub booking_id: Option<Uuid>,
    pub container_id: Option<Uuid>,
    pub quantity: Option<i32>,
}

impl Into<BookingCapacityData> for ModelOptionDto {
    fn into(self) -> BookingCapacityData {
        BookingCapacityData {
            booking_id: self.booking_id,
            container_id: self.container_id,
            quantity: self.quantity,
        }
    }
}

impl From<Model> for BookingCapacityData {
    fn from(m: Model) -> Self {
        BookingCapacityData {
            booking_id: Some(m.booking_id),
            container_id: Some(m.container_id),
            quantity: Some(m.quantity),
        }
    }
}
