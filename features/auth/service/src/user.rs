use tracing::debug;
use uuid::Uuid;

use shared_shared_data_app::result::Result;
use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::Pagination,
};

use features_auth_entities::access::AccessForCreateDto;
use features_auth_repo::access::{AccessMutation, AccessQuery};

pub struct UserService {}

impl UserService {
    pub async fn assign_roles(user_id: Uuid, role_ids: Vec<Uuid>, key: Option<String>) -> Result<bool> {
        debug!("Assign roles {:?} to user {:?}", role_ids, user_id);
        let key = key.unwrap_or_default();
        for role_id in &role_ids {
            let create_dto = AccessForCreateDto {
                user_id,
                role_id: *role_id,
                key: key.clone(),
            };
            let insert = AccessMutation::create(create_dto).await;
            if insert.is_err() {
                debug!(
                    "Failed to assign role {:?} to user {:?}: {:?}",
                    role_id,
                    user_id,
                    insert.err()
                );
            }
        }
        Ok(true)
    }

    pub async fn unassign_roles(user_id: Uuid, role_ids: Vec<Uuid>) -> Result<bool> {
        let param: FilterParam<Uuid> = FilterParam {
            name: "user_id".to_string(),
            operator: FilterOperator::Equal,
            value: Some(user_id),
            raw_value: user_id.to_string(),
        };
        let user_filter = FilterEnum::Uuid(param);
        let filters: Vec<FilterEnum> = vec![user_filter];
        let pagination = Pagination::new(1, 200);
        let order = Order::default();
        let search =
            AccessQuery::search(&pagination, &order, &FilterCondition::from(filters)).await?;
        for dto in search.result {
            if let Some(role_id) = dto.role_id {
                debug!(
                    "Current role id {:?} and unassign roles {:?}",
                    role_id, role_ids
                );
                if role_ids.contains(&role_id) {
                    debug!(
                        "Unassign role id {:?} from user id {:?}",
                        role_id, user_id
                    );
                    if let Some(id) = dto.id {
                        let _ = AccessMutation::delete(id).await?;
                    }
                }
            }
        }
        Ok(true)
    }
}
