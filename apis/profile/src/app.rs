use std::time::Duration;

use axum::Router;
use features_auth_remote::PermissionService;
use features_profiles_model::state::{ProfileAppState, ProfileCacheState};
use tokio::{spawn, time::interval};
use tracing::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use shared_shared_app::{
    config::AppConfig, discovery::get_consul_client, start_app::StartApp, state::AppState,
};
use shared_shared_config::db::Database;

use features_profiles_migrations::{Migrator, MigratorTrait};

use crate::{
    doc::ApiDoc,
    routes::{
        profile::routes as profile_routes, social_link::routes as social_link_routes,
        user_preference::routes as user_preference_routes,
    },
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<ProfileAppState, ProfileCacheState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn custom_handler(
        &self,
        app_state: &mut AppState<ProfileAppState, ProfileCacheState>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        let mut clone_app_state = app_state.clone();
        async move {
            spawn(async move {
                let service_key = "PROFILE".to_string();
                let mut interval = interval(Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    let consul_client = get_consul_client().unwrap();
                    PermissionService::update_remote(&consul_client).await;
                    let all_permissions =
                        PermissionService::get_roles_by_service_name(service_key.clone()).await;
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
                        clone_app_state.set_permission_map(role_name, mask_permissions);
                    }
                }
            });
            Ok(())
        }
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

    fn routes(&self, app_state: &AppState<ProfileAppState, ProfileCacheState>) -> Router {
        let all_routes = Router::new()
            .merge(profile_routes(app_state))
            .merge(user_preference_routes(app_state))
            .merge(social_link_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "PROFILE".to_string(),
        Some("profile".to_string()),
        true,
        true,
    );

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app(None).await?;

    debug!("Profile app stopped");

    Ok(())
}
