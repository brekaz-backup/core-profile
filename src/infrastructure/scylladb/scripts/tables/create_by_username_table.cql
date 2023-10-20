CREATE TABLE IF NOT EXISTS profile.by_username (
    username text,
    user_id uuid,
    primary key (username)
)