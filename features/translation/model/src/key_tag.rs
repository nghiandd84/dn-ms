use serde::Serialize;
use shared_shared_macro::Response;
use utoipa::ToSchema;
use uuid::Uuid;

use features_translation_entities::key_tag::ModelOptionDto;

#[derive(Serialize, Debug, ToSchema, Default, Response)]
pub struct KeyTagData {
    pub id: Option<Uuid>,
    pub key_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
}
impl Into<KeyTagData> for ModelOptionDto {
    fn into(self) -> KeyTagData {
        KeyTagData {
            id: self.id,
            key_id: self.key_id,
            tag_id: self.tag_id,
        }
    }
}
