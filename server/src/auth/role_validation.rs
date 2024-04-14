use std::str::FromStr;
use crate::error::ApiError;
use well_i_known_core::modal::user::UserRole;
use tracing::*;

const ADMIN_ROLES: [UserRole; 2] = [UserRole::Admin, UserRole::Root];

pub struct RoleValidationUtil {}
impl RoleValidationUtil {
    /// Check if the given role is in the list of roles with access.
    pub fn authorized_role(requester_role: &str, role_with_access: &[UserRole]) -> bool {
        match UserRole::from_str(requester_role) {
            Ok(role) => role_with_access.contains(&role),
            Err(_) => false,
        }
    }

    pub fn is_admin(requester_role: &str) -> bool {
        RoleValidationUtil::authorized_role(requester_role, &ADMIN_ROLES)
    }

    pub fn is_root(requester_role: &str) -> bool {
        RoleValidationUtil::authorized_role(requester_role, &[UserRole::Root])
    }

    /// Check if the given role is admin or the user himself.
    pub fn is_admin_or_self(requester_role: &str, requester_username: &str, request_username: &str) -> bool {
        RoleValidationUtil::is_admin(requester_role) || requester_username == request_username
    }

    /// Check if the given account can be created by the requester.
    /// 1. Root can create any account.
    /// 2. Admin can create App account.
    /// 3. App cannot create any account.
    pub fn can_create_account(requester_role: &UserRole, role_to_create: &UserRole) -> bool {
        match requester_role {
            UserRole::Root => true,
            UserRole::Admin => {
                match role_to_create {
                    UserRole::Root => false,
                    UserRole::Admin => false,
                    UserRole::App => true,
                }
            },
            UserRole::App => false,
        }
    }

    /// Default auth error handler.
    /// Throw ApiError::Unauthorized if the user is not authorized.
    pub fn throw_if_unauthorized(authorized: bool, username: &str, operation: &str) -> Result<(), ApiError> {
        if authorized {
            Ok(())
        } else {
            let error_message = format!("User '{}' is unauthorized to {}.", username, operation);
            warn!(error_message);
            return Err(ApiError::Unauthorized { 
                message: error_message,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorized_role() {
        let roles = vec![UserRole::Root, UserRole::Admin];
        assert_eq!(RoleValidationUtil::authorized_role("Root", &roles), true);
        assert_eq!(RoleValidationUtil::authorized_role("Admin", &roles), true);
        assert_eq!(RoleValidationUtil::authorized_role("App", &roles), false);
        assert_eq!(RoleValidationUtil::authorized_role("abc", &roles), false);
    }
}