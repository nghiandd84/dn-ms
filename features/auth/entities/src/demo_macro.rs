use serde::Serialize;

use shared_shared_macro::Dto;

#[derive(Dto, Default)]
#[dto(name(ForChangeAgeName), columns(name, age), option)]
// #[dto(name(ForChangeDobNameOption), columns(name, age), option)]
pub struct Model {
    pub name: String,
    pub age: u32,
    pub dob: String,
}

/*
use std::fmt::Debug;

use serde::Serialize;
use shared_shared_macro::Dto;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum StringOrNumber {
    Str(String),
    Num(i32),
}
impl Default for StringOrNumber {
    fn default() -> Self {
        StringOrNumber::Str(String::new())
    }
}
impl Debug for StringOrNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringOrNumber::Str(s) => write!(f, "String({})", s),
            StringOrNumber::Num(n) => write!(f, "Number({})", n),
        }
    }
}

#[derive(Default, Debug, Serialize)]
pub struct SubModel {
    pub address: String,
    pub phone: String,
}
// Implement the trait for the original struct
#[derive(Dto, Default)]
#[dto(name(ForChangeAgeDob), columns(name, age, dob))]
#[dto(name(ForField1Field2), columns(name, field1, field2, field3))]
#[dto(name(ForSubModel), columns(sub_model, age))]
pub struct Model {
    pub name: String,
    pub age: u32,
    pub dob: String,
    pub field1: String,
    pub field2: String,
    pub field3: StringOrNumber,
    pub sub_model: SubModel,
}
 */
