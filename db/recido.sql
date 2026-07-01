-- ============================================================
-- Recipe App V2 — Script PostgreSQL
-- Schémas séparés : users / content
-- ============================================================

-- Extension UUID
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================
-- SCHEMAS
-- ============================================================

CREATE SCHEMA IF NOT EXISTS users;
CREATE SCHEMA IF NOT EXISTS content;

-- ============================================================
-- ENUMS (globaux, pas attachés à un schéma)
-- ============================================================

CREATE TYPE restriction_category AS ENUM ('allergy', 'diet');

CREATE TYPE meal_type AS ENUM ('breakfast', 'lunch', 'dinner', 'snack');

CREATE TYPE friendship_status AS ENUM ('pending', 'accepted', 'blocked');

CREATE TYPE log_action AS ENUM ('DELETE', 'LOGIN', 'LOGOUT', 'ERROR');

CREATE TYPE log_entity AS ENUM (
    'USER', 'RECIPE', 'INGREDIENT', 'COMMENT',
    'POST', 'FRIENDSHIP', 'SESSION'
);

CREATE TYPE tag_category AS ENUM ('diet', 'season', 'difficulty');

CREATE TYPE unit_type AS ENUM ('piece', 'g', 'ml', 'tbsp', 'tsp', 'cup', 'kg', 'l');

-- ============================================================
-- SCHEMA : users
-- ============================================================

-- ------------------------------------------------------------
-- users.users
-- ------------------------------------------------------------
CREATE TABLE users.users (
    user_id       UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    email         VARCHAR     UNIQUE NOT NULL,
    password VARCHAR     NOT NULL,
    login        VARCHAR     UNIQUE NOT NULL,
    avatar_url    TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ
);

CREATE INDEX idx_users_email  ON users.users (email);
CREATE INDEX idx_users_login ON users.users (login);

CREATE TABLE users.sessions (
    session_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users.users(user_id) ON DELETE CASCADE,
    refresh_token_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL,
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    ip_address INET,
    user_agent TEXT,
    revoked_at TIMESTAMPTZ
);

CREATE INDEX idx_sessions_user_id ON users.sessions(user_id);
CREATE UNIQUE INDEX idx_sessions_refresh_token_hash ON users.sessions(refresh_token_hash);

-- Trigger updated_at
CREATE OR REPLACE FUNCTION users.set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_users_updated_at
    BEFORE UPDATE ON users.users
    FOR EACH ROW EXECUTE FUNCTION users.set_updated_at();

-- ------------------------------------------------------------
-- users.roles
-- ------------------------------------------------------------
CREATE TABLE users.roles (
    id_role    UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    level      VARCHAR     UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ------------------------------------------------------------
-- users.user_roles
-- ------------------------------------------------------------
CREATE TABLE users.user_roles (
    id_user_roles UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id       UUID        NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    role_id       UUID        NOT NULL REFERENCES users.roles (id_role) ON DELETE CASCADE,
    granted_at    TIMESTAMPTZ DEFAULT NOW(),
    granted_by    UUID        REFERENCES users.users (user_id) ON DELETE SET NULL,

    CONSTRAINT uq_user_roles UNIQUE (user_id, role_id)
);

-- ------------------------------------------------------------
-- users.sessions
-- ------------------------------------------------------------
CREATE TABLE users.sessions (
    id_session   UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id      UUID        NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    token_hash   VARCHAR     UNIQUE NOT NULL,
    device_info  VARCHAR,
    ip_address   INET,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at   TIMESTAMPTZ NOT NULL,
    last_used_at TIMESTAMPTZ DEFAULT NOW(),
    is_revoked   BOOLEAN     NOT NULL DEFAULT FALSE
);

CREATE INDEX idx_sessions_token_hash ON users.sessions (token_hash);
CREATE INDEX idx_sessions_user_id    ON users.sessions (user_id);
CREATE INDEX idx_sessions_expires_at ON users.sessions (expires_at);

-- ------------------------------------------------------------
-- users.dietary_restrictions
-- ------------------------------------------------------------
CREATE TABLE users.dietary_restrictions (
    id_dietary_restriction UUID               PRIMARY KEY DEFAULT uuid_generate_v4(),
    category               restriction_category NOT NULL,
    name                   VARCHAR            UNIQUE NOT NULL
);

-- ------------------------------------------------------------
-- users.user_dietary_restrictions
-- ------------------------------------------------------------
CREATE TABLE users.user_dietary_restrictions (
    id_user_dietary_restriction UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id                     UUID NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    id_dietary_restriction      UUID NOT NULL REFERENCES users.dietary_restrictions (id_dietary_restriction) ON DELETE CASCADE,

    CONSTRAINT uq_user_dietary UNIQUE (user_id, id_dietary_restriction)
);

-- ------------------------------------------------------------
-- users.logs
-- ------------------------------------------------------------
CREATE TABLE users.logs (
    id            UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id       UUID        REFERENCES users.users (user_id) ON DELETE SET NULL,
    action_type   log_action  NOT NULL,
    entity_type   log_entity  NOT NULL,
    entity_id     UUID,
    old_values    JSONB,
    new_values    JSONB,
    ip_address    INET,
    error_message TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_logs_user_created    ON users.logs (user_id, created_at);
CREATE INDEX idx_logs_entity          ON users.logs (entity_type, entity_id, created_at);
CREATE INDEX idx_logs_errors          ON users.logs (created_at) WHERE action_type = 'ERROR';

-- ============================================================
-- SCHEMA : content
-- ============================================================

-- ------------------------------------------------------------
-- content.ingredient_categories
-- ------------------------------------------------------------
CREATE TABLE content.ingredient_categories (
    id   UUID    PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR UNIQUE NOT NULL
);

-- ------------------------------------------------------------
-- content.ingredients
-- ------------------------------------------------------------
CREATE TABLE content.ingredients (
    id_ingredient     UUID      PRIMARY KEY DEFAULT uuid_generate_v4(),
    name              VARCHAR   UNIQUE NOT NULL,
    url_ingredient_img TEXT,
    category_id       UUID      REFERENCES content.ingredient_categories (id) ON DELETE SET NULL,
    default_unit      unit_type DEFAULT 'g'
);

CREATE INDEX idx_ingredients_name ON content.ingredients (name);

-- ------------------------------------------------------------
-- content.ingredient_nutrients
-- ------------------------------------------------------------
CREATE TABLE content.ingredient_nutrients (
    ingredient_id            UUID PRIMARY KEY REFERENCES content.ingredients (id_ingredient) ON DELETE CASCADE,
    energy_kcal              REAL,
    protein_g                REAL,
    carbohydrate_g           REAL,
    fat_g                    REAL,
    fiber_g                  REAL,
    sodium_mg                REAL,
    fatty_acids_saturated_g  REAL,
    sugar_total_g            REAL,
    sucrose_g                REAL,
    glucose_g                REAL,
    fructose_g               REAL,
    lactose_g                REAL,
    maltose_g                REAL,
    alcohol_g                REAL,
    caffeine_mg              REAL
);

-- ------------------------------------------------------------
-- content.recipes
-- ------------------------------------------------------------
CREATE TABLE content.recipes (
    id_recipe          UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id            UUID        NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    name_recipe        VARCHAR     NOT NULL,
    description        TEXT,
    difficulty         VARCHAR,
    prep_time_minutes  INT,
    cook_time_minutes  INT,
    servings           INT         DEFAULT 4,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_recipes_user_id    ON content.recipes (user_id);
CREATE INDEX idx_recipes_created_at ON content.recipes (created_at);
CREATE INDEX idx_recipes_name       ON content.recipes (name_recipe);

CREATE OR REPLACE FUNCTION content.set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_recipes_updated_at
    BEFORE UPDATE ON content.recipes
    FOR EACH ROW EXECUTE FUNCTION content.set_updated_at();

-- ------------------------------------------------------------
-- content.steps
-- ------------------------------------------------------------
CREATE TABLE content.steps (
    id_step           UUID    PRIMARY KEY DEFAULT uuid_generate_v4(),
    id_recipe         UUID    NOT NULL REFERENCES content.recipes (id_recipe) ON DELETE CASCADE,
    number_step       INT     NOT NULL,
    description       TEXT    NOT NULL,
    duration_minutes  INT
);

CREATE INDEX idx_steps_recipe ON content.steps (id_recipe);

-- ------------------------------------------------------------
-- content.recipe_ingredients
-- ------------------------------------------------------------
CREATE TABLE content.recipe_ingredients (
    id_recipe_ingredient UUID      PRIMARY KEY DEFAULT uuid_generate_v4(),
    id_recipe            UUID      NOT NULL REFERENCES content.recipes (id_recipe) ON DELETE CASCADE,
    id_ingredient        UUID      NOT NULL REFERENCES content.ingredients (id_ingredient) ON DELETE RESTRICT,
    quantity             DECIMAL   NOT NULL,
    unit                 unit_type NOT NULL,
    step_order           INT       DEFAULT 0
);

CREATE INDEX idx_recipe_ingredients_recipe ON content.recipe_ingredients (id_recipe);

-- ------------------------------------------------------------
-- content.equipment
-- ------------------------------------------------------------
CREATE TABLE content.equipment (
    id_equipment        UUID    PRIMARY KEY DEFAULT uuid_generate_v4(),
    name                VARCHAR UNIQUE NOT NULL,
    equipment_image_url TEXT
);

CREATE INDEX idx_equipment_name ON content.equipment (name);

-- ------------------------------------------------------------
-- content.recipe_equipment
-- ------------------------------------------------------------
CREATE TABLE content.recipe_equipment (
    id_recipe_equipment UUID    PRIMARY KEY DEFAULT uuid_generate_v4(),
    id_recipe           UUID    NOT NULL REFERENCES content.recipes (id_recipe) ON DELETE CASCADE,
    id_equipment        UUID    NOT NULL REFERENCES content.equipment (id_equipment) ON DELETE RESTRICT,
    quantity            INT     DEFAULT 1,
    notes               TEXT
);

CREATE INDEX idx_recipe_equipment_recipe ON content.recipe_equipment (id_recipe);

-- ------------------------------------------------------------
-- content.images_recipes
-- ------------------------------------------------------------
CREATE TABLE content.images_recipes (
    id_image_recipe  UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    id_recipe        UUID        NOT NULL REFERENCES content.recipes (id_recipe) ON DELETE CASCADE,
    url_image_recipe TEXT        NOT NULL,
    user_id          UUID        NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    is_cover         BOOLEAN     DEFAULT FALSE,
    created_at       TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_images_recipe ON content.images_recipes (id_recipe);

-- ------------------------------------------------------------
-- content.tags
-- ------------------------------------------------------------
CREATE TABLE content.tags (
    id       UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    name     VARCHAR      UNIQUE NOT NULL,
    category tag_category
);

-- ------------------------------------------------------------
-- content.recipe_tags
-- ------------------------------------------------------------
CREATE TABLE content.recipe_tags (
    recipe_id UUID NOT NULL REFERENCES content.recipes (id_recipe) ON DELETE CASCADE,
    tag_id    UUID NOT NULL REFERENCES content.tags (id) ON DELETE CASCADE,

    PRIMARY KEY (recipe_id, tag_id)
);

CREATE INDEX idx_recipe_tags_tag ON content.recipe_tags (tag_id);

-- ------------------------------------------------------------
-- content.meal_plan
-- ------------------------------------------------------------
CREATE TABLE content.meal_plan (
    id_meal_plan     UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id          UUID        NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    id_recipe        UUID        NOT NULL REFERENCES content.recipes (id_recipe) ON DELETE CASCADE,
    plan_date        DATE        NOT NULL,
    meal_type        meal_type   NOT NULL,
    servings_planned INT         DEFAULT 4,
    created_at       TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_meal_plan_user_date ON content.meal_plan (user_id, plan_date);

-- ------------------------------------------------------------
-- content.grocery_list
-- ------------------------------------------------------------
CREATE TABLE content.grocery_list (
    id_grocery_list UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID        NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    id_ingredient   UUID        REFERENCES content.ingredients (id_ingredient) ON DELETE SET NULL,
    custom_name     VARCHAR,
    quantity        DECIMAL     NOT NULL DEFAULT 0,
    unit            unit_type   DEFAULT 'piece',
    category        VARCHAR,
    is_checked      BOOLEAN     NOT NULL DEFAULT FALSE,
    id_meal_plan    UUID        REFERENCES content.meal_plan (id_meal_plan) ON DELETE SET NULL,
    created_at      TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT chk_grocery_item CHECK (id_ingredient IS NOT NULL OR custom_name IS NOT NULL)
);

CREATE INDEX idx_grocery_user_checked ON content.grocery_list (user_id, is_checked);
CREATE INDEX idx_grocery_meal_plan    ON content.grocery_list (id_meal_plan);

-- ------------------------------------------------------------
-- content.user_save
-- ------------------------------------------------------------
CREATE TABLE content.user_save (
    id_user_save    UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID        NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    id_recipe       UUID        NOT NULL REFERENCES content.recipes (id_recipe) ON DELETE CASCADE,
    collection_name VARCHAR     DEFAULT 'Favoris',
    added_at        TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT uq_user_save UNIQUE (user_id, id_recipe, collection_name)
);

-- ------------------------------------------------------------
-- content.recipes_feedback
-- ------------------------------------------------------------
CREATE TABLE content.recipes_feedback (
    id_recipe_feedback UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id            UUID        NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    id_recipe          UUID        NOT NULL REFERENCES content.recipes (id_recipe) ON DELETE CASCADE,
    is_cooked          BOOLEAN     NOT NULL DEFAULT FALSE,
    cooked_at          TIMESTAMPTZ,
    rating             INT         CHECK (rating BETWEEN 1 AND 5),
    feedback           TEXT,
    created_at         TIMESTAMPTZ DEFAULT NOW(),
    updated_at         TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT uq_feedback UNIQUE (user_id, id_recipe)
);

CREATE INDEX idx_feedback_recipe_rating ON content.recipes_feedback (id_recipe, rating);

-- ------------------------------------------------------------
-- content.recipe_comments
-- ------------------------------------------------------------
CREATE TABLE content.recipe_comments (
    id_recipe_comment UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    id_recipe         UUID        NOT NULL REFERENCES content.recipes (id_recipe) ON DELETE CASCADE,
    user_id           UUID        NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    description       TEXT        NOT NULL,
    parent_comment_id UUID        REFERENCES content.recipe_comments (id_recipe_comment) ON DELETE CASCADE,
    created_at        TIMESTAMPTZ DEFAULT NOW(),
    updated_at        TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_recipe_comments_recipe  ON content.recipe_comments (id_recipe, created_at);
CREATE INDEX idx_recipe_comments_parent  ON content.recipe_comments (parent_comment_id);

-- ------------------------------------------------------------
-- content.posts
-- ------------------------------------------------------------
CREATE TABLE content.posts (
    id_post     UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id     UUID        NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    id_recipe   UUID        REFERENCES content.recipes (id_recipe) ON DELETE SET NULL,
    post_image  TEXT        NOT NULL,
    description TEXT,
    created_at  TIMESTAMPTZ DEFAULT NOW(),
    updated_at  TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_posts_user_created ON content.posts (user_id, created_at);
CREATE INDEX idx_posts_recipe       ON content.posts (id_recipe);

-- ------------------------------------------------------------
-- content.post_like
-- ------------------------------------------------------------
CREATE TABLE content.post_like (
    id_post_like UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    id_post      UUID        NOT NULL REFERENCES content.posts (id_post) ON DELETE CASCADE,
    user_id      UUID        NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    created_at   TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT uq_post_like UNIQUE (id_post, user_id)
);

CREATE INDEX idx_post_like_post ON content.post_like (id_post);

-- ------------------------------------------------------------
-- content.post_comments
-- ------------------------------------------------------------
CREATE TABLE content.post_comments (
    id_post_comment   UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    id_post           UUID        NOT NULL REFERENCES content.posts (id_post) ON DELETE CASCADE,
    user_id           UUID        NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    comment           TEXT        NOT NULL,
    parent_comment_id UUID        REFERENCES content.post_comments (id_post_comment) ON DELETE CASCADE,
    created_at        TIMESTAMPTZ DEFAULT NOW(),
    updated_at        TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_post_comments_post   ON content.post_comments (id_post, created_at);
CREATE INDEX idx_post_comments_parent ON content.post_comments (parent_comment_id);

-- ------------------------------------------------------------
-- content.friendships
-- ------------------------------------------------------------
CREATE TABLE content.friendships (
    id_friendship    UUID              PRIMARY KEY DEFAULT uuid_generate_v4(),
    id_request_user  UUID              NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    id_address_user  UUID              NOT NULL REFERENCES users.users (user_id) ON DELETE CASCADE,
    state            friendship_status NOT NULL DEFAULT 'pending',
    created_at       TIMESTAMPTZ       DEFAULT NOW(),

    CONSTRAINT uq_friendship UNIQUE (id_request_user, id_address_user),
    CONSTRAINT chk_no_self_friend CHECK (id_request_user <> id_address_user)
);

CREATE INDEX idx_friendships_request ON content.friendships (id_request_user, state);
CREATE INDEX idx_friendships_address ON content.friendships (id_address_user, state);

-- ============================================================
-- FIN DU SCRIPT
-- ============================================================
