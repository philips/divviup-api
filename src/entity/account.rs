use sea_orm::{entity::prelude::*, ActiveValue::Set, IntoActiveModel};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use validator::{Validate, ValidationErrors};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "account")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub id: Uuid,

    pub name: String,

    #[serde(with = "::time::serde::iso8601")]
    pub created_at: OffsetDateTime,

    #[serde(with = "::time::serde::iso8601")]
    pub updated_at: OffsetDateTime,

    #[serde(skip)]
    pub admin: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::membership::Entity")]
    Membership,
    #[sea_orm(has_many = "super::task::Entity")]
    Task,
}

impl Related<super::membership::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Membership.def()
    }
}

impl Related<super::task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct NewAccount {
    #[validate(required, length(min = 3, max = 100))]
    name: Option<String>,
}

impl Model {
    pub fn build(name: String) -> Result<ActiveModel, ValidationErrors> {
        NewAccount { name: Some(name) }.build()
    }
}

impl NewAccount {
    pub fn build(self) -> Result<ActiveModel, ValidationErrors> {
        self.validate()?;
        Ok(ActiveModel {
            name: Set(self.name.unwrap()),
            id: Set(Uuid::new_v4()),
            created_at: Set(TimeDateTimeWithTimeZone::now_utc()),
            updated_at: Set(TimeDateTimeWithTimeZone::now_utc()),
            ..Default::default()
        })
    }
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct UpdateAccount {
    #[validate(required, length(min = 3, max = 100))]
    name: Option<String>,
}

impl UpdateAccount {
    pub fn build(self, account: Model) -> Result<ActiveModel, ValidationErrors> {
        self.validate()?;
        let mut am = account.into_active_model();
        am.name = Set(self.name.unwrap());
        am.updated_at = Set(TimeDateTimeWithTimeZone::now_utc());
        Ok(am)
    }
}
