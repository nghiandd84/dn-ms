use chrono::Utc;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveValue::NotSet, EntityTrait, QueryFilter},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use features_lookup_entities::{lookup_item, lookup_type};

#[derive(Deserialize, Serialize)]
struct CountryData {
    code: String,
    name: String,
    sort_order: i32,
    iso3: String,
    phone_code: String,
    region: String,
    capital: String,
    currency: String,
    flag: String,
}
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260407_000004_seed_countries"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // First, insert the lookup_type for countries
        let lookup_type_model = lookup_type::ActiveModel {
            id: Set(Uuid::new_v4()),
            code: Set("COUNTRY".to_string()),
            name: Set("Country".to_string()),
            description: Set("List of countries and territories".to_string()),
            is_active: Set(true),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            tenant_id: NotSet,
        };

        let inserted_lookup_type = lookup_type::Entity::insert(lookup_type_model)
            .exec(db)
            .await?;

        let lookup_type_id = inserted_lookup_type.last_insert_id;

        // Read the countries JSON file (embedded at compile time)
        let countries_json = include_str!("countries.json");

        let countries: Vec<CountryData> = serde_json::from_str(countries_json)
            .map_err(|e| DbErr::Custom(format!("Failed to parse countries.json: {}", e)))?;

        // Insert country data
        for country in countries {
            let lookup_item_model = lookup_item::ActiveModel {
                id: Set(Uuid::new_v4()),
                lookup_type_id: Set(lookup_type_id),
                code: Set(country.code),
                name: Set(country.name),
                url: NotSet,
                query_param_one: NotSet,
                query_param_two: NotSet,
                meta: Set(json!({
                    "iso3": country.iso3,
                    "phone_code": country.phone_code,
                    "region": country.region,
                    "capital": country.capital,
                    "currency": country.currency,
                    "flag": country.flag,
                })),
                tenants: Set(vec![]),
                is_active: Set(true),
                sort_order: Set(country.sort_order),
                created_at: Set(Utc::now().naive_utc()),
                updated_at: Set(Utc::now().naive_utc()),
            };

            lookup_item::Entity::insert(lookup_item_model)
                .exec(db)
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        // First, find the COUNTRY lookup type
        let lookup_type_result = lookup_type::Entity::find()
            .filter(lookup_type::Column::Code.eq("COUNTRY"))
            .one(db)
            .await?;

        if let Some(lookup_type_record) = lookup_type_result {
            let lookup_type_id = lookup_type_record.id;

            // Delete all country lookup items
            lookup_item::Entity::delete_many()
                .filter(lookup_item::Column::LookupTypeId.eq(lookup_type_id))
                .exec(db)
                .await?;

            // Delete the lookup_type
            lookup_type::Entity::delete_by_id(lookup_type_id)
                .exec(db)
                .await?;
        }

        Ok(())
    }
}
