pub mod prelude {
    pub use super::Argon2PasswordSalter;

    pub(super) use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, Salt, SaltString},
        Argon2,
    };
    // pub(super) use

    pub use super::super::prelude::*;
}
use prelude::*;

mod jwt;

pub struct Argon2PasswordSalter;
impl PasswordSalter for Argon2PasswordSalter {
    fn salt_password(&self, salt: &String, password: &String) -> Result<String> {
        let argon2 = Argon2::default();

        let salt = Salt::from_b64(salt)?;
        let password = password.as_bytes();

        let result = argon2.hash_password(password, salt)?;

        Ok(result.to_string())
    }

    fn gerenate_salt(&self) -> String {
        let result = SaltString::generate(&mut OsRng);
        result.as_str().to_string()
    }
}
