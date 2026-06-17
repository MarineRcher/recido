-- Add up migration script here

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
INSERT INTO users.roles (level) VALUES ('user'), ('admin');
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
    user_agent    TEXT,
    error_message TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_logs_user_created    ON users.logs (user_id, created_at);
CREATE INDEX idx_logs_entity          ON users.logs (entity_type, entity_id, created_at);
CREATE INDEX idx_logs_errors          ON users.logs (created_at) WHERE action_type = 'ERROR';
