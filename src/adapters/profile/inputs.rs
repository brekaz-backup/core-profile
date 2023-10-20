

// #[derive(Default)]
// pub struct SignUpValidator;
//
// impl CustomValidator<SignUpRegisterInput> for SignUpValidator {
//     fn check(&self, value: &SignUpRegisterInput) -> Result<(), String> {
//         if value.phone.is_some() && value.email.is_some() {
//             return Err(String::from("Invalid request you can't send Phone and Email at the same time"));
//         }
//         if value.phone.is_none() && value.email.is_none() {
//             return Err(String::from("Invalid request at least Phone or Email are required in this request"));
//         }
//
//         Ok(())
//     }
// }