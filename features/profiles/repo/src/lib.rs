pub mod profile;
pub mod social_link;
pub mod user_preference;

pub use profile::{ProfileMutation, ProfileQuery};
pub use social_link::{SocialLinkMutation, SocialLinkQuery};
pub use user_preference::{UserPreferenceMutation, UserPreferenceQuery};
