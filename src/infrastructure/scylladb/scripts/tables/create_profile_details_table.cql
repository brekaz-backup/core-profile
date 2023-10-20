CREATE TABLE IF NOT EXISTS profile.profile_details (
    user_id uuid,
    lat float,
    lng float,
    birthday date,
    created_at timestamp,
    primary key (user_id)
)