use std::vec;

use sea_orm::{FromQueryResult, LoaderTrait};
use shared_shared_auth::claim::UserAccessData;
use shared_shared_data_app::error::AppError;
use shared_shared_data_auth::{error::AuthError, password::compare};
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
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
    fn build_filter_condition(filters: &Vec<FilterEnum>) -> Condition {
        let mut condition = Condition::all();
        for filter_enum in filters {
            let name = filter_enum.get_name();
            if name.starts_with("role.`") {
                if let Ok(column) = RoleColumn::from_str(filter_enum.get_name().as_str()) {
                    condition =
                        condition.add(Self::filter_condition_rolecolumn(column, filter_enum));
                }
            } else {
                if let Ok(column) = Column::from_str(filter_enum.get_name().as_str()) {
                    condition = condition.add(Self::filter_condition_column(column, filter_enum));
                }
            }
        }
        condition
    }
}

pub struct UserQuery;

impl UserQuery {
    pub async fn get_user_by_email_and_password<'a>(
        db: &'a DbConn,
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

        let result = UserQueryManager::filter(db, &pagination, &order, &filters).await?;

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

    pub async fn get_access_data_by_user_id<'a>(
        db: &'a DbConn,
        id: Uuid,
    ) -> Result<Vec<UserAccessData>, DbErr> {
        let models = Entity::find()
            .filter(Column::Id.eq(id))
            .find_with_related(AccessEntity)
            .all(db)
            .await?;

        if let Some(data) = models.first() {
            let user_model = data.0.clone();
            let accesses = data.1.clone();
            let users = vec![user_model.clone()];
            let roles = users
                .load_many_to_many(RoleEntity, AccessEntity, db)
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
    pub async fn get<'a>(db: &'a DbConn, user_id: Uuid) -> Result<UserData, DbErr> {
        let model = UserQueryManager::get_by_id_uuid(db, user_id).await?;
        let user_data: UserData = model.into();
        Ok(user_data)
    }

    pub async fn search<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<UserData>, DbErr> {
        let result = UserQueryManager::filter(db, pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn advance_search<'a>(
        db: &'a DbConn,
        _pagination: &Pagination,
        _order: &Order,
        _filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<UserData>, DbErr> {
        // let result = UserQueryManager::filter(db, pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: 3,
            result: vec![],
        };
        debug!("Start test search");
        // TODO understand how to use Select, SelectTwo, and SelectTwoMany
        // Only one find_with_related. Hope for new version of sea_orm
        // let users = Entity::find().find_with_related(RoleEntity);
        // let x = Entity::find().into_tuple()
        let users = Entity::find()
            .filter(Column::Email.contains("nghia"))
            // .find_also_related(AccessEntity)
            // .find_also_linked(l)
            // .find_with_linked(l)
            .find_with_related(AccessEntity)
            .all(db)
            .await?;
        // .join(JoinType::LeftJoin, features_auth_entities::role::Relation::Access);
        // .join(JoinType::LeftJoin, rel);

        // Filter by role.name
        // let users = users
        //     // .filter(RoleColumn::Name.contains("2"))
        //     // .filter(
        //     //     AccessColumn::Id
        //     //         .ne(Uuid::parse_str("885d3245-17cd-4d16-a424-6158cb59693e").unwrap()),
        //     // )
        //     // .filter(Column::Email.contains("2"))
        //     // .into_model::<AdvanceUser>()
        //     .all(db)
        //     .await?;
        // let users = users.filter(Column::Email.contains("n")).all(db).await?;
        debug!("users: {:?}", users);

        let user1s = Entity::find()
            .filter(Column::Email.contains("n"))
            .all(db)
            .await?;
        debug!("user1s: {:?}", user1s);
        let user2s = user1s
            .load_many_to_many(RoleEntity, AccessEntity, db)
            .await?;
        debug!("user2s: {:?}", user2s);
        // let rs = users
        //     .load_many_to_many(RoleEntity, AccessEntity, db)
        //     .await?;

        Ok(mapped_result)
    }
}

#[derive(FromQueryResult, Debug)]
struct AdvanceUser {
    id: Uuid,
    key: String,
}
