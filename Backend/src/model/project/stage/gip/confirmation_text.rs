use crate::model::contract::comment_text::CommentText;

pub struct GipConfirmationText {
    confirmed: bool,
}

impl GipConfirmationText {
    pub fn new(confirmed: bool) -> Self {
        Self { confirmed }
    }
}

impl CommentText for GipConfirmationText {
    fn text(&self) -> String {
        if self.confirmed {
            "ГИП подтвердил выполнение".to_string()
        } else {
            "ГИП снял подтверждение выполнения".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_gip_confirmation_text() {
        assert_eq!(
            GipConfirmationText::new(true).text(),
            "ГИП подтвердил выполнение"
        );
        assert_eq!(
            GipConfirmationText::new(false).text(),
            "ГИП снял подтверждение выполнения"
        );
    }
}
