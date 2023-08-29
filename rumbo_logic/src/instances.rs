pub mod prelude {
    pub(super) use diesel::{Insertable, Queryable, Selectable};

    pub use serde::{Deserialize, Serialize};

    // Loading the lib.rs prelude
    pub use super::super::prelude::*;

    pub use super::Instance;
    pub use super::InstanceService;
}
use prelude::*;

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::instances)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Instance {
    pub id: i64,
    name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::instances)]
struct NewInstance<'a> {
    name: &'a str,
}

#[derive(Clone)]
pub struct InstanceService {
    db_adapter: Arc<DbAdapter>,
}

impl InstanceService {
    pub fn new(db_adapter: &Arc<DbAdapter>) -> Self {
        InstanceService {
            db_adapter: db_adapter.clone(),
        }
    }

    pub fn as_arc(self) -> Arc<Self> {
        Arc::from(self)
    }

    pub async fn create(&self, instance: &Instance) -> Result<Instance> {
        use crate::schema::instances;

        let instance = NewInstance {
            name: &instance.name,
        };

        let mut connection = self.db_adapter.get_connection()?;
        let result = diesel::insert_into(instances::table)
            .values(instance)
            .returning(Instance::as_returning())
            .get_result(&mut connection)?;

        Ok(result)
    }

    pub async fn get(&self, instance_id: i64) -> Result<Option<Instance>> {
        use crate::schema::instances::dsl::*;

        let mut connection = self.db_adapter.get_connection()?;
        let result = instances
            .find(instance_id)
            .first::<Instance>(&mut connection)
            .optional()?;

        Ok(result)
    }

    pub async fn with_page(&self, skip: i64, top: i64) -> Result<Vec<Instance>> {
        use crate::schema::instances::dsl::*;

        let mut connection = self.db_adapter.get_connection()?;
        let result = instances
            .order(id.asc())
            .offset(skip)
            .limit(top)
            .load::<Instance>(&mut connection)?;

        Ok(result)
    }

    pub async fn delete(&self, instance_id: i64) -> Result<()> {
        use crate::schema::instances::dsl::*;

        let mut connection = self.db_adapter.get_connection()?;
        diesel::delete(instances.find(instance_id)).execute(&mut connection)?;
        Ok(())
    }

    pub async fn update(&self, instance: &Instance) -> Result<Instance> {
        use crate::schema::instances::dsl::*;

        let mut connection = self.db_adapter.get_connection()?;
        let result = diesel::update(instances.find(instance.id))
            .set(instance)
            .returning(Instance::as_returning())
            .get_result(&mut connection)?;
        Ok(result)
    }
}
