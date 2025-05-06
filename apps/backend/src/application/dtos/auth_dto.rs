use crate::presentation::graphql::types::auth_type::SignUpInput;

#[derive(Debug, Clone)]
pub struct SignUpDto {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl From<SignUpInput> for SignUpDto {
    fn from(input: SignUpInput) -> Self {
        Self {
            name: input.name,
            email: input.email,
            password: input.password,
        }
    }
}
