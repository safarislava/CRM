use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum Role {
    Gip,
    Lawyer,
    Accountant,
    Admin,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_and_deserializes_roles() {
        assert_eq!(serde_json::to_string(&Role::Gip).unwrap(), "\"gip\"");
        assert_eq!(serde_json::to_string(&Role::Lawyer).unwrap(), "\"lawyer\"");
        assert_eq!(
            serde_json::to_string(&Role::Accountant).unwrap(),
            "\"accountant\""
        );
        assert_eq!(serde_json::to_string(&Role::Admin).unwrap(), "\"admin\"");

        let r: Role = serde_json::from_str("\"admin\"").unwrap();
        assert_eq!(r, Role::Admin);
    }
}
