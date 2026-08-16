use crate::model::user::contract::invite::Invite;
use uuid::Uuid;

pub struct InviteCode {
    token: Uuid,
}

impl InviteCode {
    pub fn new(token: Uuid) -> Self {
        Self { token }
    }
}

impl Invite for InviteCode {
    fn token(&self) -> Uuid {
        self.token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correct_invite_token() {
        let token = Uuid::new_v4();
        let invite = InviteCode::new(token);

        assert_eq!(invite.token(), token);
    }
}
