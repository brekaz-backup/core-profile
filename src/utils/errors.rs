use blumer_lib_errors::AppError;
use strum_macros::Display;

#[derive(Debug, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ProfileError {
    CantUpdateUsernameYet,
    CantUpdateNamesYet,
    UsernameNotAvailable,
    NotValidUsername,
    CantViewThisUser,
}

impl  ProfileError {
    pub fn into(self) -> AppError {
        let reason = match self {
            ProfileError::CantUpdateUsernameYet => "User cant update the username yet, try again in {:?} days",
            ProfileError::CantUpdateNamesYet => "User cant update the name yet, try again in {:?} days",
            ProfileError::UsernameNotAvailable => "Username not available",
            ProfileError::NotValidUsername => "Your username only allowed to contain letters, numbers, dots and underscore",
            ProfileError::CantViewThisUser => "Profile can´t view this profile (Blocked)",
        };

        AppError::ValidationError { reason: reason.to_string(), code: self.to_string() }
    }
}