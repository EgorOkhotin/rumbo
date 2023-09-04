pub mod prelude {
    pub use super::super::prelude::*;

    pub use super::AuthenticationService;
    pub use super::TokenValidationResult;
    pub use super::LoginData;
}
use prelude::*;

use super::jwt::CustomClaims;



pub struct AuthenticationService {
    user_service: Arc<UserService>,
    salter: Arc<dyn PasswordSalter>,
    token_service: Arc<JwtTokenFactory<RedisTokenCache>>
}

pub struct LoginData {
    email: String,
    password: String,
}

pub enum TokenValidationResult {
    Invalid,
    Valid(AuthenticationClaims),
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct AuthenticationClaims {
    pub id: i64
}
impl CustomClaims for AuthenticationClaims {}

pub struct Token(String);

impl AuthenticationService {
    pub fn new(user_service: &Arc<UserService>, salter: &Arc<dyn PasswordSalter>, token_service: &Arc<JwtTokenFactory<RedisTokenCache>>) -> Self {
        Self {
            user_service: user_service.clone(),
            salter: salter.clone(),
            token_service: token_service.clone()
        }
    }

    pub fn as_arc(self) -> Arc<Self> {
        Arc::from(self)
    }

    pub async fn with_login(&self, data: LoginData) -> Result<Option<Token>> {
        let user = self.user_service.with_email(&data.email).await?;
        if user.is_none() {
            return Ok(None);
        };

        let user = user.unwrap();
        let salted_password = self.salter.salt_password(&user.salt_b64, &data.password)?;
        let auth_claims = AuthenticationClaims { id: user.id };

        if salted_password.eq(&user.salted_password_b64) {
            let token = self.token_service.create_token(auth_claims, None);
            Ok(Some(Token(token)))
        }
        else {
            Ok(None)
        }
    }

    pub async fn with_token(&self, data: String) -> Result<TokenValidationResult> {
        let is_valid = self.token_service.get_user_data_if_token_valid(data)?;

        let validation_result = match is_valid {
            Some(user) => TokenValidationResult::Valid(user),
            None => TokenValidationResult::Invalid
        };

        Ok(validation_result)
    }
}
