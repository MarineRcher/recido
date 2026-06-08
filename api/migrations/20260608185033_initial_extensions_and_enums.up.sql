-- Add up migration script here
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
