pub mod prelude {
    pub(super) use super::super::prelude::*;
    pub(super) use diesel::dsl::*;

    pub use super::PasswordSalter;
    pub use super::User;
    pub use super::UserService;
}
use prelude::*;

#[derive(Serialize, Deserialize, Debug, Selectable, Queryable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,

    #[diesel(column_name = "salt")]
    pub(crate) salt_b64: String,
    #[diesel(column_name = "salted_password")]
    pub(crate) salted_password_b64: String,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub name: String,
    pub email: String,

    #[diesel(column_name = "salt")]
    salt_b64: String,
    #[diesel(column_name = "salted_password")]
    salted_password_b64: String,
}

pub trait PasswordSalter: Sync + Send {
    fn salt_password(&self, salt_b64: &String, password_b64: &String) -> Result<String>;
    fn gerenate_salt(&self) -> String;
}

#[derive(Clone)]
pub struct UserService {
    db_adapter: Arc<DbAdapter>,
    salter: Arc<dyn PasswordSalter>,
}
impl UserService {
    pub fn new(db: &Arc<DbAdapter>, salter: &Arc<dyn PasswordSalter>) -> Self {
        UserService {
            db_adapter: db.clone(),
            salter: salter.clone(),
        }
    }

    pub fn as_arc(self) -> Arc<Self> {
        Arc::from(self)
    }

    pub async fn add_new_user(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<User> {
        let existed_user = self.find_by_email(&email).await?;

        if existed_user.is_some() {
            return Err(RumboError::UserError(
                "User alredy existed error".to_string(),
            ));
        }

        let salt = self.salter.gerenate_salt();
        let salted_password = self.salter.salt_password(&salt, &password)?;
        let user = NewUser {
            name: name,
            email: email,
            salt_b64: salt,
            salted_password_b64: salted_password,
        };
        let user = self.create(user).await?;

        Ok(user)
    }

    pub async fn authenticate(&self, email: &String, password: &String) -> Result<User> {
        let existed_user = self.find_by_email(&email).await?;

        if existed_user.is_none() {
            return Err(RumboError::UserError("User is not registered".to_string()));
        }

        let existed_user = existed_user.unwrap();

        let salt = &existed_user.salt_b64;
        let salted_password = self.salter.salt_password(&salt, &password)?;

        if !salted_password.eq(&existed_user.salted_password_b64) {
            return Err(RumboError::UserError("Incorrect password".to_string()));
        }

        Ok(existed_user)
    }

    pub async fn update_user(&self, user: User) -> Result<User> {
        let user = self.find_by_email(&user.email).await?;

        if user.is_none() {
            return Err(RumboError::UserError(
                "Update error, user wasn't found in DB".to_string(),
            ));
        }

        self.update(&user.unwrap()).await
    }

    pub async fn delete_user(&self, email: &str) -> Result<()> {
        let user = self.find_by_email(email).await?;

        if user.is_none() {
            return Err(RumboError::UserError(
                "Can't delete user becaue it wasn't found in DB".to_string(),
            ));
        }

        self.delete(user.unwrap().id).await
    }

    pub async fn with_email(&self, user_email: &str) -> Result<Option<User>> {
        self.find_by_email(user_email).await
    }

    pub async fn with_id(&self, user_id: i64) -> Result<Option<User>> {
        self.get(user_id).await
    }

    async fn create(&self, user: NewUser) -> Result<User> {
        let mut connection = self.db_adapter.get_connection()?;

        let result = diesel::insert_into(crate::schema::users::table)
            .values(user)
            .returning(User::as_returning())
            .get_result(&mut connection)?;

        Ok(result)
    }

    async fn find_by_email(&self, user_email: &str) -> Result<Option<User>> {
        use crate::schema::users::dsl::*;

        let mut connection = self.db_adapter.get_connection()?;
        let result = users
            .filter(with_email(user_email))
            .first::<User>(&mut connection)
            .optional()?;

        Ok(result)
    }

    async fn get(&self, user_id: i64) -> Result<Option<User>> {
        use crate::schema::users::dsl::*;
        let mut connection = self.db_adapter.get_connection()?;

        let result = users
            .find(user_id)
            .first::<User>(&mut connection)
            .optional()?;

        Ok(result)
    }

    async fn delete(&self, user_id: i64) -> Result<()> {
        use crate::schema::users::dsl::*;
        let mut connection = self.db_adapter.get_connection()?;

        diesel::delete(users.find(user_id)).execute(&mut connection)?;

        Ok(())
    }

    async fn update(&self, user: &User) -> Result<User> {
        use crate::schema::users::dsl::*;
        let mut connection = self.db_adapter.get_connection()?;

        let result = diesel::update(users.find(user.id))
            .set(user)
            .returning(User::as_returning())
            .get_result(&mut connection)?;

        Ok(result)
    }
}

type WithEmail<'a> = Eq<crate::schema::users::email, &'a str>;

fn with_email(value: &str) -> WithEmail {
    crate::schema::users::email.eq(value)
}
