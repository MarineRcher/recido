#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "restriction_category", rename_all = "snake_case")]
pub enum RestrictionCategory {
    Allergy,
    Diet,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "meal_type", rename_all = "snake_case")]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "friendship_status", rename_all = "snake_case")]
pub enum FriendshipStatus {
    Pending,
    Accepted,
    Blocked,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "log_action", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogAction {
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

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "tag_category", rename_all = "snake_case")]
pub enum TagCategory {
    Diet,
    Season,
    Difficulty,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "unit_type", rename_all = "snake_case")]
pub enum UnitType {
    Piece,
    G,
    Ml,
    Tbsp,
    Tsp,
    Cup,
    Kg,
    L,
}
