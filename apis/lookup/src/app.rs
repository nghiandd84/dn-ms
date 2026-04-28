use std::time::Duration;

use axum::Router;
use features_auth_remote::PermissionService;
use tokio::{spawn, time::interval};
use tracing::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use shared_shared_app::{
    config::AppConfig, discovery::get_consul_client, start_app::StartApp, state::AppState,
};
use shared_shared_config::db::Database;

use features_lookup_migrations::{Migrator, MigratorTrait};
use features_lookup_model::state::{LookupAppState, LookupCacheState};

use crate::{
    doc::ApiDoc,
    routes::{
        lookup_item::routes as lookup_item_routes,
        lookup_item_translation::routes as lookup_item_translation_routes,
        lookup_type::routes as lookup_type_routes,
    },
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<LookupAppState, LookupCacheState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn public_paths(&self) -> &'static [&'static str] {
        &["/lookup-types"]
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

    fn routes(&self, app_state: &AppState<LookupAppState, LookupCacheState>) -> Router {
        let all_routes = Router::new()
            .merge(lookup_type_routes(app_state))
            .merge(lookup_item_routes(app_state))
            .merge(lookup_item_translation_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }

    fn custom_handler(
        &self,
        app_state: &mut AppState<LookupAppState, LookupCacheState>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        let mut clone_app_state = app_state.clone();
        async move {
            spawn(async move {
                let service_key = "LOOKUP".to_string();

                let mut interval = interval(Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    debug!("Interval task running ...");
                    debug!(
                        "Call API Permission to get permission by service name: {}",
                        service_key
                    );
                    let consul_client = get_consul_client().unwrap();
                    PermissionService::update_remote(&consul_client).await;
                    let all_permissions =
                        PermissionService::get_roles_by_service_name(service_key.clone()).await;
                    debug!(
                        "Permissions for service {}: {:?}",
                        service_key, all_permissions
                    );

                    for (role_name, permissions) in all_permissions {
                        let mask_permissions = permissions
                            .iter()
                            .map(|perm| {
                                (
                                    perm.resource.clone().unwrap_or_default(),
                                    perm.mask.unwrap_or(0) as u32,
                                )
                            })
                            .collect();
                        debug!(
                            "Role: {}, Permissions mask: {:?}",
                            role_name, mask_permissions
                        );
                        clone_app_state.set_permission_map(role_name, mask_permissions);
                    }
                }
            });
            Ok(())
        }
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let service_key = "LOOKUP".to_string();
    let app_config = AppConfig::new(
        service_key.to_string(),
        Some("lookup".to_string()),
        true,
        true,
    );

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app(None).await?;

    debug!("Lookup app stopped");

    Ok(())
}
