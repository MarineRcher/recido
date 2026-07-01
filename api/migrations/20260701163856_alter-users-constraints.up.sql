
ALTER TABLE users.sessions
    DROP CONSTRAINT IF EXISTS sessions_user_id_fkey,
    ADD CONSTRAINT sessions_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users.users(user_id) ON DELETE CASCADE;

ALTER TABLE users.user_roles
    DROP CONSTRAINT IF EXISTS user_roles_user_id_fkey,
    ADD CONSTRAINT user_roles_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users.users(user_id) ON DELETE CASCADE;

ALTER TABLE users.logs
    DROP CONSTRAINT IF EXISTS logs_user_id_fkey,
    ADD CONSTRAINT logs_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users.users(user_id) ON DELETE SET NULL;
