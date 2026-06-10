use sqlx::FromRow;
use uuid::Uuid;
use crate::models::users::enums::RestrictionCategory;

#[derive(Debug, FromRow)]
pub struct DietaryRestriction {
    pub id_dietary_restriction: Uuid,
    pub category: RestrictionCategory,
    pub name: String,
}

#[derive(Debug, FromRow)]
pub struct UserDietaryRestriction {
    pub id_user_dietary_restriction: Uuid,
    pub user_id: Uuid,
    pub id_dietary_restriction: Uuid,
}
