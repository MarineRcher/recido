-- Add down migration script here

DROP TABLE IF EXISTS users.logs;
DROP TABLE IF EXISTS users.user_dietary_restrictions;
DROP TABLE IF EXISTS users.dietary_restrictions;
DROP TABLE IF EXISTS users.sessions;
DROP TABLE IF EXISTS users.user_roles;
DROP TABLE IF EXISTS users.roles;
DROP TRIGGER IF EXISTS trg_users_updated_at ON users.users;
DROP FUNCTION IF EXISTS users.set_updated_at();
DROP TABLE IF EXISTS users.users;
