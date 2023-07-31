use super::prelude::*;

pub trait JwtTokenFactory<T>: Clone + Send + Sync
where
    T: TokenCacheService,
{
    fn create_token(&self, user: User, valid_until: Option<chrono::NaiveDateTime>) -> String;
    fn revoke_token(&self, token: String) -> Result<()>;
    fn is_token_valid(&self, token: String) -> Result<bool>;

    fn new(cache_service: T) -> Self;
}

#[derive(Clone)]
pub struct DefaultJwtTokenFactory<T>
where
    T: TokenCacheService,
{
    cache_service: T,
}

impl JwtTokenFactory<RedisTokenCache> for DefaultJwtTokenFactory<RedisTokenCache> {
    fn create_token(&self, user: User, valid_until: Option<chrono::NaiveDateTime>) -> String {
        todo!("Create new one token")
    }

    fn revoke_token(&self, token: String) -> Result<()> {
        todo!("Sends token to redis")
    }

    fn is_token_valid(&self, token: String) -> Result<bool> {
        todo!("Validate token")
    }

    fn new(cache_service: RedisTokenCache) -> Self {
        Self {
            cache_service: cache_service,
        }
    }
}

pub trait TokenCacheService: Clone + Send + Sync {
    fn add_revoked_token(&self, token: String) -> Result<()>;
}

#[derive(Clone)]
pub struct RedisTokenCache {
    redis_connection: String,
}

impl TokenCacheService for RedisTokenCache {
    fn add_revoked_token(&self, token: String) -> Result<()> {
        todo!()
    }
}
