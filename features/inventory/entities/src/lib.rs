

pub mod seat;
pub mod reservation;

/*
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeatEntityDto {
    pub id: Option<Uuid>,
    pub event_id: Option<Uuid>,
    pub seat_number: Option<String>,
    pub section: Option<String>,
    pub row_number: Option<String>,
    pub seat_type: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub status: Option<String>,
    pub version: Option<i64>,
    pub reserved_by: Option<String>,
    pub reserved_until: Option<DateTime>,
    pub booking_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReservationEntityDto {
    pub id: Option<Uuid>,
    pub seat_id: Option<Uuid>,
    pub event_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub expires_at: Option<DateTime>,
    pub status: Option<String>,
    pub created_at: Option<DateTime>,
}
     */
