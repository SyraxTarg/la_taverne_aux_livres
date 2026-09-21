use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub role_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::role::Entity",
        from = "Column::RoleId",
        to = "super::role::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Role,

    #[sea_orm(has_many = "super::user_reading::Entity")]
    UserReadBooks,
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Related<super::user_reading::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserReadBooks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}