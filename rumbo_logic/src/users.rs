pub mod prelude {
    pub(super) use super::super::prelude::*;

    pub use super::PasswordSalter;
    pub use super::User;
    pub use super::UserService;
}

use prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: i64,
    pub name: String,
    pub email: String,
    salt_b64: String,
    salted_password_b64: String,
    // instances: Vec<ObjectId>,
}

impl User {
    fn new(name: String, email: String, salt: String, salted_password: String) -> Self {
        User {
            id: 0,
            name,
            email,
            salt_b64: salt,
            salted_password_b64: salted_password,
            // instances: vec![],
        }
    }
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
        // let existed_user = self.find_by_email(&email).await?;

        // if existed_user.is_some() {
        //     return Err(RumboError::UserError(
        //         "User alredy existed error".to_string(),
        //     ));
        // }

        // let salt = self.salter.gerenate_salt();
        // let salted_password = self.salter.salt_password(&salt, &password)?;
        // let user = User::new(name, email, salt, salted_password);

        // let user = self.create(&user).await?;

        // Ok(user)
        todo!()
    }

    pub async fn authorize(&self, email: &String, password: &String) -> Result<User> {
        // let existed_user = self.find_by_email(&email).await?;

        // if existed_user.is_none() {
        //     return Err(RumboError::UserError("User is not registered".to_string()));
        // }

        // let existed_user = existed_user.unwrap();

        // let salt = &existed_user.salt_b64;
        // let salted_password = self.salter.salt_password(&salt, &password)?;

        // if !salted_password.eq(&existed_user.salted_password_b64) {
        //     return Err(RumboError::UserError("Incorrect password".to_string()));
        // }

        // Ok(existed_user)
        todo!()
    }

    pub async fn update_user(&self, user: User) -> Result<User> {
        // let user = self.find_by_email(&user.email).await?;

        // if user.is_none() {
        //     return Err(RumboError::UserError(
        //         "Update error, user wasn't found in DB".to_string(),
        //     ));
        // }
        // self.update(&user.unwrap()).await
        todo!()
    }

    pub async fn delete_user(&self, email: &str) -> Result<()> {
        // let user = self.find_by_email(email).await?;

        // if user.is_none() {
        //     return Err(RumboError::UserError(
        //         "Can't delete user becaue it wasn't found in DB".to_string(),
        //     ));
        // }

        // self.delete(&user.unwrap().id.unwrap().to_hex()).await
        todo!()
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        // let collection = self.get_collection();

        // let filter = get_email_filter(email);
        // let result = collection.find_one(filter, None).await?;
        // Ok(result)
        todo!()
    }

    async fn create(&self, user: &User) -> Result<User> {
        // let collection = self.get_collection();

        // let result = collection.insert_one(user, None).await?;
        // let inserted_id = result.inserted_id.as_object_id().unwrap().to_hex();
        // let user = self.get(&inserted_id).await?.unwrap();
        // Ok(user)
        todo!()
    }

    async fn get(&self, id: &str) -> Result<Option<User>> {
        // let collection = self.get_collection();

        // let filter = get_id_filter_from_str(id);
        // let result = collection.find_one(filter, None).await?;
        // Ok(result)
        todo!()
    }

    async fn delete(&self, id: &str) -> Result<()> {
        // let collection = self.get_collection();
        // let filter = get_id_filter_from_str(id);

        // let _result = collection.delete_one(filter, None).await?;
        // Ok(())
        todo!()
    }

    async fn update(&self, user: &User) -> Result<User> {
        // let collection = self.get_collection();

        // let id = user.id.unwrap();
        // let filter = get_id_filter_from_object(&id);

        // let result = collection.replace_one(filter, user, None).await?;

        // if result.modified_count > 0 {
        //     info!("Updated entities count = {}", result.modified_count);

        //     let user = self.get(&id.to_hex()).await?.unwrap();
        //     Ok(user)
        // } else {
        //     let user = self.get(&id.to_hex()).await?.unwrap();
        //     Ok(user)
        // }
        todo!()
    }
}
