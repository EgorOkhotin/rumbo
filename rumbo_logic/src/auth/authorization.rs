pub mod prelude {
    pub use super::super::prelude::*;
    pub use super::super::authentication::AuthenticationClaims;

    pub use super::AuthorizationResult;
    pub use super::AuthorizationData;
    pub use super::AuthorizationService;
}
use prelude::*;

pub struct AuthorizationService {
    access_rule_service: Arc<AccessRuleService>
}

pub enum AuthorizationResult {
    Granted,
    Denied,
}

pub struct AuthorizationData {
    use_case_id: i64,
    resource: String,
    resource_id: Option<i64>
}

impl AuthorizationService {
    pub fn new(access_rule_service: &Arc<AccessRuleService>) -> Self {
        Self {
            access_rule_service: access_rule_service.clone()
        }
    }

    pub fn as_arc(self) -> Arc<Self> {
        Arc::from(self)
    }

    pub async fn authorize(
        &self,
        validation_result: TokenValidationResult,
        authorization_data: AuthorizationData
    ) -> Result<AuthorizationResult> {
        match validation_result {
            TokenValidationResult::Invalid => Ok(AuthorizationResult::Denied),
            TokenValidationResult::Valid(user) => self.authorize_user(user, authorization_data).await
        }
    }

    async fn authorize_user(&self, user: AuthenticationClaims, data: AuthorizationData) -> Result<AuthorizationResult> {
        let result = self.access_rule_service.get(user.id, data.use_case_id, data.resource).await?;

        if result.len() == 0 {
            return Ok(AuthorizationResult::Denied);
        }

        let is_rule_exists = result.iter().find(|x| x.resource_id == data.resource_id).is_some();

        if is_rule_exists {
            Ok(AuthorizationResult::Granted)
        }
        else {
            Ok(AuthorizationResult::Denied)
        }
    }
}
