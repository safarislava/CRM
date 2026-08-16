use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CommentId {
    id: Uuid,
}

impl CommentId {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_comment_id() {
        let uuid = Uuid::new_v4();
        let comment_id = CommentId::new(uuid);

        assert_eq!(comment_id.id(), uuid);
        assert_eq!(CommentId::new(uuid), comment_id);
    }
}
