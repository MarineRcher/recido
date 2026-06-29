#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "restriction_category", rename_all = "snake_case")]
pub enum RestrictionCategory {
    Allergy,
    Diet,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "log_action", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogAction {
    Create,
    Delete,
    Login,
    Logout,
    Error,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "log_entity", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogEntity {
    User,
    Recipe,
    Ingredient,
    Comment,
    Post,
    Friendship,
    Session,
}

