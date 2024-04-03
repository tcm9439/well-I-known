use std::str::FromStr;
use well_i_known_core::modal::user::UserRole;

/// Check if the given role is in the list of roles with access.
pub fn authorized_role(role: &str, role_with_access: &[UserRole]) -> bool {
    match UserRole::from_str(role) {
        Ok(role) => role_with_access.contains(&role),
        Err(_) => false,
    }
}

// pub fn is_admin

// pub fn is_root

/// Check if the given role is admin or the user himself.
// pub fn is_admin_or_self

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorized_role() {
        let roles = vec![UserRole::Root, UserRole::Admin];
        assert_eq!(authorized_role("Root", &roles), true);
        assert_eq!(authorized_role("Admin", &roles), true);
        assert_eq!(authorized_role("App", &roles), false);
        assert_eq!(authorized_role("abc", &roles), false);
    }
}