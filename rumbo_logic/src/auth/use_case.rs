pub mod prelude {
    pub use super::super::prelude::*;

    pub use super::NewUseCase;
    pub use super::UseCase;
    pub use super::UseCaseService;
}
use prelude::*;

#[derive(Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = crate::schema::use_cases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UseCase {
    id: i64,
    name: String,
    description: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::use_cases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUseCase {
    name: String,
    description: Option<String>,
}

#[derive(Clone)]
pub struct UseCaseService {
    db_adapter: Arc<DbAdapter>,
}

impl UseCaseService {
    pub fn new(adapter: &Arc<DbAdapter>) -> Self {
        Self {
            db_adapter: adapter.clone(),
        }
    }

    pub fn as_arc(self) -> Arc<Self> {
        Arc::from(self)
    }

    pub async fn create(&self, name: String, description: Option<String>) -> Result<UseCase> {
        use crate::schema::use_cases;

        let new_use_case = NewUseCase { name, description };

        let mut connection = self.db_adapter.get_connection()?;

        let result = diesel::insert_into(use_cases::table)
            .values(new_use_case)
            .returning(UseCase::as_returning())
            .get_result(&mut connection)?;

        Ok(result)
    }

    pub async fn get(&self, use_case_id: i64) -> Result<Option<UseCase>> {
        use crate::schema::use_cases::dsl::*;
        let mut connection = self.db_adapter.get_connection()?;

        let result = use_cases
            .find(use_case_id)
            .first::<UseCase>(&mut connection)
            .optional()?;

        Ok(result)
    }

    pub async fn update(&self, case: UseCase) -> Result<UseCase> {
        use crate::schema::use_cases::dsl::*;
        let mut connection = self.db_adapter.get_connection()?;

        let result = diesel::update(use_cases.find(case.id))
            .set(case)
            .returning(UseCase::as_returning())
            .get_result(&mut connection)?;

        Ok(result)
    }

    pub async fn delete(&self, use_case_id: i64) -> Result<()> {
        use crate::schema::use_cases::dsl::*;
        let mut connection = self.db_adapter.get_connection()?;
        diesel::delete(use_cases.find(use_case_id)).execute(&mut connection)?;

        Ok(())
    }
}
