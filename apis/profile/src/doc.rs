use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Profile API",
        version = "0.1.0",
        description = "Complete Profile Restful API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
        crate::routes::profile::create_profile,
        crate::routes::profile::get_profile,
        crate::routes::profile::get_profile_by_user_id,
        crate::routes::profile::filter_profiles,
        crate::routes::profile::update_profile,
        crate::routes::profile::delete_profile,
        crate::routes::user_preference::create_user_preference,
        crate::routes::user_preference::get_user_preference,
        crate::routes::user_preference::get_user_preference_by_profile_id,
        crate::routes::user_preference::filter_user_preferences,
        crate::routes::user_preference::update_user_preference,
        crate::routes::user_preference::delete_user_preference,
        crate::routes::social_link::create_social_link,
        crate::routes::social_link::get_social_link,
        crate::routes::social_link::get_social_links_by_profile_id,
        crate::routes::social_link::filter_social_links,
        crate::routes::social_link::update_social_link,
        crate::routes::social_link::delete_social_link,
    ),
    components(
        schemas(
            features_profiles_model::ProfileData,
            features_profiles_model::ProfileForCreateRequest,
            features_profiles_model::ProfileForUpdateRequest,
            features_profiles_model::UserPreferenceData,
            features_profiles_model::UserPreferenceForCreateRequest,
            features_profiles_model::UserPreferenceForUpdateRequest,
            features_profiles_model::SocialLinkData,
            features_profiles_model::SocialLinkForCreateRequest,
            features_profiles_model::SocialLinkForUpdateRequest,
        )
    ),
    tags(
        (name = "profile", description = "Profile management operations"),
        (name = "user-preference", description = "User preference management operations"),
        (name = "social-link", description = "Social link management operations"),
    ),
    modifiers(&JwtSecurityAddon)
    
)]
pub struct ApiDoc;
