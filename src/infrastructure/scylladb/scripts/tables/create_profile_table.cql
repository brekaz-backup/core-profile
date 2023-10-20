CREATE TABLE IF NOT EXISTS profile.profiles (
    user_id uuid,
    username text,
    names text,
    photo text,
    photo_hash text,
    portrait text,
    portrait_hash text,
    description text,
    privacy boolean,
    verified boolean,
    is_active boolean,
    primary key (user_id)
)