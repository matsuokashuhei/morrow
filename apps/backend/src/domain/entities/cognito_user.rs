use aws_sdk_cognitoidentityprovider::operation::sign_up::SignUpOutput;

#[derive(Debug, Clone)]
pub struct CognitoUser {
    pub user_confirmed: bool,
    pub user_sub: String,
}

impl From<SignUpOutput> for CognitoUser {
    fn from(output: SignUpOutput) -> Self {
        Self {
            user_confirmed: output.user_confirmed,
            user_sub: output.user_sub,
        }
    }
}
