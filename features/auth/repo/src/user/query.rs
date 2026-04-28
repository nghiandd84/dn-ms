use sea_orm::LoaderTrait;
use std::vec;
use tracing::debug;
use uuid::Uuid;

use shared_shared_auth::claim::UserAccessData;
use shared_shared_data_app::password::compare;
use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::{app::AppError, auth::AuthError};
use shared_shared_macro::Query;

use features_auth_entities::access::{Column as AccessColumn, Entity as AccessEntity};
use features_auth_entities::role::{Column as RoleColumn, Entity as RoleEntity};
use features_auth_entities::user::{ActiveModel, Column, Entity, ModelOptionDto};
use features_auth_model::user::UserData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(AccessColumn))]
#[query_filter(column_name(RoleColumn))]
#[query_filter(column_name(Column))]
struct UserQueryManager;

impl UserQueryManager {
    fn build_filter_condition(filter_condition: &FilterCondition) -> Condition {
        match filter_condition {
            FilterCondition::And(conditions) => {
                let mut condition = Condition::all();
                for c in conditions {
                    condition = condition.add(Self::build_filter_condition(c));
                }
                condition
            }
            FilterCondition::Or(conditions) => {
                let mut condition = Condition::any();
                for c in conditions {
                    condition = condition.add(Self::build_filter_condition(c));
                }
                condition
            }
            FilterCondition::Leaf(filter_enum) => {
                let name = filter_enum.get_name();
                if name.starts_with("role.`") {
                    if let Ok(column) = RoleColumn::from_str(name.as_str()) {
                        return Self::filter_condition_rolecolumn(column, filter_enum);
                    }
                } else if let Ok(column) = Column::from_str(name.as_str()) {
                    return Self::filter_condition_column(column, filter_enum);
                }
                Condition::all()
            }
        }
    }
}

pub struct UserQuery;

impl UserQuery {
    pub async fn get_user_by_email_and_password<'a>(
        email: String,
        password: String,
    ) -> Result<UserData, AppError> {
        let pagination = Pagination::new(1, 1);
        let order = Order::default();

        let param: FilterParam<String> = FilterParam {
            name: Column::Email.to_string(),
            operator: FilterOperator::Equal,
            value: Some(email.clone()),
            raw_value: email,
        };
        let email_filter = FilterEnum::String(param);
        let filters: Vec<FilterEnum> = vec![email_filter];

        let result =
            UserQueryManager::filter(&pagination, &order, &FilterCondition::from(filters)).await?;

        let dto = result.result.into_iter().next();
        if dto.is_none() {
            return Err(AppError::Auth(AuthError::NotFoundUser));
        }
        let dto = dto.unwrap();
        let current_password = dto.password.clone().unwrap();
        let compare_password = compare(&password, &current_password);

        if compare_password.is_err() {
            return Err(AppError::Auth(AuthError::WrongPassword));
        }
        let compare_password = compare_password.unwrap();
        if !compare_password {
            return Err(AppError::Auth(AuthError::WrongPassword));
        }
        Ok(dto.into())
    }

    pub async fn get_access_data_by_user_id<'a>(id: Uuid) -> Result<Vec<UserAccessData>, DbErr> {
        let models = Entity::find()
            .filter(Column::Id.eq(id))
            .find_with_related(AccessEntity)
            .all(UserQueryManager::get_db())
            .await?;

        if let Some(data) = models.first() {
            let user_model = data.0.clone();
            let accesses = data.1.clone();
            let users = vec![user_model.clone()];
            let roles = users
                .load_many_to_many(RoleEntity, AccessEntity, UserQueryManager::get_db())
                .await?;

            let roles = roles.first().unwrap();
            let results: Vec<UserAccessData> = accesses
                .iter()
                .map(|item| {
                    let find_role = roles.iter().find(|role| role.id == item.role_id);
                    debug!("find_role {:?}", find_role);
                    let find_role_name = find_role.map_or("".to_string(), |role| role.name.clone());
                    let user_access_data = UserAccessData {
                        key: Some(item.key.clone()),
                        role_name: find_role_name.to_string(),
                    };
                    user_access_data
                })
                .collect();
            Ok(results)
        } else {
            return Err(DbErr::RecordNotFound("Not found".to_string()));
        }
    }
    pub async fn get<'a>(user_id: Uuid) -> Result<UserData, DbErr> {
        let model = UserQueryManager::get_by_id_uuid(user_id).await?;
        let user_data: UserData = model.into();
        Ok(user_data)
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<UserData>, DbErr> {
        let result = UserQueryManager::filter(pagination, order, &filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
