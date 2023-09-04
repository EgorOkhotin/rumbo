pub mod prelude {
    pub use super::super::prelude::*;

    pub use super::JwtTokenFactory;
    pub use super::RedisTokenCache;

    pub use jwt_compact::{prelude::*, alg::{Hs256, Hs256Key}};
}
use prelude::*;
use redis::Commands;
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct JwtTokenFactory<T>
where
    T: TokenCacheService,
{
    cache_service: T,
}

impl JwtTokenFactory<RedisTokenCache> {
    pub fn create_token<T: CustomClaims>(&self, claims: T, valid_until: Option<chrono::Duration>) -> String {
            // Choose time-related options for token creation / validation.
            let time_options = match valid_until {
                None => self.get_time_options(),
                Some(value) => TimeOptions::from_leeway(value)
            };

            // Create a symmetric HMAC key, which will be used both to create and verify tokens.
            let key = self.get_secrect_key();
            // Create a token.
            let header = Header::default().with_key_id("my-key");
            let claims = Claims::new(claims)
                .set_duration_and_issuance(&time_options, Duration::hours(1))
                .set_not_before(Utc::now());
            let token_string = Hs256.token(header, &claims, &key).unwrap();

            token_string
    }

    pub fn revoke_token(&self, token: String) -> Result<()> {
        self.cache_service.add_revoked_token(token)
    }

    pub fn get_user_data_if_token_valid<T: CustomClaims>(&self, token: String) -> Result<Option<T>> {

        if self.cache_service.is_token_revoked(&token)? {
            return Ok(None);
        }

        let time_options = self.get_time_options();
        let key = self.get_secrect_key();

        // Parse the token.
        let token = UntrustedToken::new(&token).unwrap();
        // Before verifying the token, we might find the key which has signed the token
        // using the `Header.key_id` field.
        // assert_eq!(token.header().key_id.as_deref(), Some("my-key"));
        // Validate the token integrity.
        let token: Token<T> = Hs256.validate_integrity(&token, &key)?;
        // Validate additional conditions.
        token.claims()
            .validate_expiration(&time_options)?
            .validate_maturity(&time_options)?;

        Ok(Some(token.claims().custom))
    }

    pub fn new(cache_service: RedisTokenCache) -> Self {
        Self {
            cache_service: cache_service,
        }
    }

    pub fn as_arc(self) -> Arc<Self> {
        Arc::from(self)
    }

    fn get_time_options(&self) -> TimeOptions {
        TimeOptions::default()
    }

    fn get_secrect_key(&self) -> Hs256Key {
        Hs256Key::new(b"super_secret_key_donut_steel")
    }
}

pub trait TokenCacheService: Clone + Send + Sync {
    fn add_revoked_token(&self, token: String) -> Result<()>;
    fn is_token_revoked(&self, token: &str) -> Result<bool>;
}

#[derive(Clone)]
pub struct RedisTokenCache {
    redis_connection: String,
}

impl TokenCacheService for RedisTokenCache {
    fn add_revoked_token(&self, token: String) -> Result<()> {
        let client = self.get_client()?;
        let mut connection = client.get_connection()?;

        connection.set(token, String::new())?;
        Ok(())
    }

    fn is_token_revoked(&self, token: &str) -> Result<bool> {
        let client = self.get_client()?;
        let mut connection = client.get_connection()?;

        Ok(connection.exists::<&str, String>(token).is_ok())
    }
}

impl RedisTokenCache {
    fn get_client(&self) -> Result<redis::Client> {
        let connection:&str = &self.redis_connection;
        Ok(redis::Client::open(connection)?)
    }

    pub fn new(connection: &str) -> Self {
        Self {
            redis_connection: connection.to_string()
        }
    }
}

pub trait CustomClaims: Copy + Serialize + DeserializeOwned {}
