use axum::Router;
use tracing::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use shared_shared_app::{config::AppConfig, start_app::StartApp, state::AppState};
use shared_shared_config::db::Database;

use features_booking_migrations::{Migrator, MigratorTrait};
use features_booking_model::state::{BookingAppState, BookingCacheState};

use crate::{
    doc::ApiDoc,
    routes::{booking::routes as booking_routes, booking_seat::routes as booking_seat_routes},
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<BookingAppState, BookingCacheState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn migrate(
        &self,
        db: &Database,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        async {
            Migrator::up((db).get_connection().as_ref(), None).await?;
            Ok(())
        }
    }

    fn routes(&self, app_state: &AppState<BookingAppState, BookingCacheState>) -> Router {
        let all_routes = Router::new()
            .merge(booking_routes(app_state))
            .merge(booking_seat_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "BOOKING".to_string(),
        Some("booking".to_string()),
        true,
        true,
    );

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app(None).await?;

    debug!("Booking app stopped");

    Ok(())
}
