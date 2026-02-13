use std::future::Future;

use features_event_stream::ChangeEventMessage;

use crate::consumers::event_consumer::error::EventError;

pub fn handle_change_event(
    message: ChangeEventMessage,
) -> impl Future<Output = Result<(), EventError>> {
    async move {
        // Process the change event
        Ok(())
    }
}
