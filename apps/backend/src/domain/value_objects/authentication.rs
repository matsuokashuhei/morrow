pub struct SignUpOutput {
    pub user_sub: String,
    pub user_confirmed: bool,
    pub session: Option<String>,
}

pub struct SignInOutput {
    pub id_token: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i32,
}
