use async_graphql::InputObject;

#[derive(InputObject, Clone)]
pub struct SignUpInput {
    #[graphql(validator(min_length = 1))]
    pub name: String,
    #[graphql(validator(email))]
    pub email: String,
    #[graphql(validator(min_length = 6))]
    pub password: String,
}
