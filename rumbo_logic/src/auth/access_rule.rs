pub mod prelude {
    pub(super) use super::super::prelude::*;
    pub(super) use diesel::dsl::*;

    pub use super::AccessRule;
    pub use super::AccessRuleService;
    pub use super::NewAccessRule;
}
use prelude::*;

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, PartialEq)]
#[diesel(table_name = crate::schema::access_rules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccessRule {
    id: i64,
    user_id: i64,
    use_case_id: i64,
    resource_type: String,
    pub resource_id: Option<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::access_rules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAccessRule {
    user_id: i64,
    use_case_id: i64,
    resource_type: String,
    resource_id: Option<i64>,
}

pub struct AccessRuleService {
    db_adapter: Arc<DbAdapter>,
}

impl AccessRuleService {
    pub fn new(db: &Arc<DbAdapter>) -> Self {
        Self {
            db_adapter: db.clone(),
        }
    }

    pub fn as_arc(self) -> Arc<Self> {
        Arc::from(self)
    }

    pub async fn create(&self, rule: NewAccessRule) -> Result<AccessRule> {
        let mut connection = self.db_adapter.get_connection()?;

        let result = diesel::insert_into(crate::schema::access_rules::table)
            .values(rule)
            .returning(AccessRule::as_returning())
            .get_result(&mut connection)?;

        Ok(result)
    }

    pub async fn get(
        &self,
        usr_id: i64,
        case_id: i64,
        resource: String,
    ) -> Result<Vec<AccessRule>> {
        use crate::schema::access_rules::dsl::*;
        let mut connection = self.db_adapter.get_connection()?;

        let result = access_rules
            .filter(with_use_case(case_id))
            .filter(with_user_id(usr_id))
            .filter(with_resource_type(resource))
            .load::<AccessRule>(&mut connection)?;

        Ok(result)
    }

    pub async fn delete(&self, rule: AccessRule) -> Result<()> {
        use crate::schema::access_rules::dsl::*;
        let mut connection = self.db_adapter.get_connection()?;
        diesel::delete(access_rules.find(rule.id)).execute(&mut connection)?;

        Ok(())
    }
}

type WithUserId = Eq<crate::schema::access_rules::user_id, i64>;
type WithUseCaseId = Eq<crate::schema::access_rules::use_case_id, i64>;
type WithResourceType = Eq<crate::schema::access_rules::resource_type, String>;

fn with_user_id(usr_id: i64) -> WithUserId {
    crate::schema::access_rules::user_id.eq(usr_id)
}

fn with_use_case(case_id: i64) -> WithUseCaseId {
    crate::schema::access_rules::use_case_id.eq(case_id)
}

fn with_resource_type(resource: String) -> WithResourceType {
    crate::schema::access_rules::resource_type.eq(resource)
}
