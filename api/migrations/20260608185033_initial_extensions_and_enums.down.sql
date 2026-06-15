-- Add down migration script here
DROP SCHEMA IF EXISTS content CASCADE;
DROP SCHEMA IF EXISTS users CASCADE;

DROP TYPE IF EXISTS tag_category;
DROP TYPE IF EXISTS unit_type;
DROP TYPE IF EXISTS log_entity;
DROP TYPE IF EXISTS log_action;
DROP TYPE IF EXISTS friendship_status;
DROP TYPE IF EXISTS meal_type;
DROP TYPE IF EXISTS restriction_category;
