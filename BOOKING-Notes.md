Task lists

- When create or update event. We need to some task
    - Trigger messate update-event(id, total_seats)
        - API inventory will receive message and create multiple seat of event


- Create api POST booking in microservice booking 
    - Lock seat_id to not allow other request can booking on this seat. Timeout 10 minutes
    - Provide seat_id and price
    - Call API inventory to check seat_available. API invetory must check table seats and reservations 
    - Create new record in booking_seats table
    - Trigger message booking (id, user_id, seat_id, event_id)
        - API invetory will receive message to create reservations and update seats.reserved_by , seats.reserved_until , seats.status = PENDING



