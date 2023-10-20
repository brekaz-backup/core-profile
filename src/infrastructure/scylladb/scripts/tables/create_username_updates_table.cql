CREATE TABLE IF NOT EXISTS profile.username_updates (
    user_id uuid,
    updated_at  timestamp,
    primary key (user_id)
)