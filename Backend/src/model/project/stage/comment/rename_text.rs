use crate::model::contract::comment_text::CommentText;

pub struct RenameText {
    old: String,
    new: String,
}

impl RenameText {
    pub fn new(old: String, new: String) -> Self {
        Self { old, new }
    }
}

impl CommentText for RenameText {
    fn text(&self) -> String {
        format!("Название изменено: «{}» → «{}»", self.old, self.new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_rename_comment_text() {
        let comment = RenameText::new("Этап 1".to_string(), "Разработка РД".to_string());
        assert_eq!(
            comment.text(),
            "Название изменено: «Этап 1» → «Разработка РД»"
        );
    }
}
