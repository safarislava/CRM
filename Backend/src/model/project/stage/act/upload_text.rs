use crate::model::contract::comment_text::CommentText;

pub struct ActUploadText {
    filename: String,
}

impl ActUploadText {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }
}

impl CommentText for ActUploadText {
    fn text(&self) -> String {
        format!("Загружен акт: {}", self.filename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_act_upload_text() {
        let text = ActUploadText::new("act_2025.pdf".to_string());
        assert_eq!(text.text(), "Загружен акт: act_2025.pdf");
    }
}
