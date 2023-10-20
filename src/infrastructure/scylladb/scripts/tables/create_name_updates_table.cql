CREATE TABLE IF NOT EXISTS profile.name_updates
(
    user_id    uuid,
    updated_at timestamp,
    primary key (user_id)
)