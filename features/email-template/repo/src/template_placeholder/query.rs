use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum},
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_macro::Query;

use features_email_template_entities::email_templates::Column as EmailTemplateColumn;
use features_email_template_entities::email_templates::Entity as EmailTemplateEntity;
use features_email_template_entities::template_placeholders::{
    ActiveModel, Column, Entity, ModelOptionDto,
};
use features_email_template_model::template_placeholder::TemplatePlaceholderData;

const RELATED_PREFIX: &str = "email_template.";

#[derive(Query)]
#[query(key_type(i32))]
#[query_filter(column_name(Column))]
#[query_related(entity(EmailTemplateEntity), column(EmailTemplateColumn), field(email_templates), name("email_template"))]
struct TemplatePlaceholderQueryManager;

/// Separate filters into parent filters and related entity filters.
/// Related filters have names starting with "email_template." prefix.
fn separate_filters(filters: &FilterCondition) -> (FilterCondition, Vec<FilterEnum>) {
    let leaves = filters.collect_leaves();
    let mut parent_leaves: Vec<FilterEnum> = vec![];
    let mut related_leaves: Vec<FilterEnum> = vec![];

    for leaf in leaves {
        let name = leaf.get_name();
        if name.starts_with(RELATED_PREFIX) {
            related_leaves.push(leaf);
        } else {
            parent_leaves.push(leaf);
        }
    }

    (parent_leaves.into(), related_leaves)
}

pub struct TemplatePlaceholderQuery {}

impl TemplatePlaceholderQuery {
    pub async fn get<'a>(id: i32) -> Result<TemplatePlaceholderData, DbErr> {
        let model = TemplatePlaceholderQueryManager::get_by_id_i32(id).await?;
        let user_data: TemplatePlaceholderData = model.into();
        Ok(user_data)
    }

    pub async fn get_with_related<'a>(
        id: i32,
        query_params: &QueryParams,
    ) -> Result<TemplatePlaceholderData, DbErr> {
        let includes = query_params.includes();
        if !includes.is_empty() {
            let model = TemplatePlaceholderQueryManager::get_by_id_i32_with_related_entities(
                id,
                &includes,
                &vec![],
            )
            .await?;
            Ok(model.into())
        } else {
            let model = TemplatePlaceholderQueryManager::get_by_id_i32(id).await?;
            Ok(model.into())
        }
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
        query_params: &QueryParams,
    ) -> Result<QueryResult<TemplatePlaceholderData>, DbErr> {
        let (parent_filters, related_filters) = separate_filters(filters);
        let has_related = !related_filters.is_empty();

        let mut includes = query_params.includes();
        // Auto-include email_template if related filters are present
        if has_related && !includes.contains(&"email_template".to_string()) {
            includes.push("email_template".to_string());
        }

        let result = if !includes.is_empty() {
            TemplatePlaceholderQueryManager::filter_with_related_entities(
                pagination,
                order,
                &parent_filters,
                &includes,
                &related_filters,
            )
            .await?
        } else {
            TemplatePlaceholderQueryManager::filter(pagination, order, &parent_filters).await?
        };
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
