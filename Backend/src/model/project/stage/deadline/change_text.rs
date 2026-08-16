use crate::model::contract::comment_text::CommentText;
use chrono::{DateTime, Utc};

pub struct DeadlineChangeText {
    old: DateTime<Utc>,
    new: Option<DateTime<Utc>>,
}

impl DeadlineChangeText {
    pub fn new(old: DateTime<Utc>, new: Option<DateTime<Utc>>) -> Self {
        Self { old, new }
    }
}

impl CommentText for DeadlineChangeText {
    fn text(&self) -> String {
        match self.new {
            Some(new) => format!(
                "Дедлайн изменён: {} → {}",
                self.old.format("%d.%m.%Y"),
                new.format("%d.%m.%Y")
            ),
            None => format!("Дедлайн удалён: {}", self.old.format("%d.%m.%Y")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_deadline_change_and_removal() {
        let old = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let new = Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();

        let change = DeadlineChangeText::new(old, Some(new));
        assert_eq!(change.text(), "Дедлайн изменён: 01.01.2025 → 15.01.2025");

        let removal = DeadlineChangeText::new(old, None);
        assert_eq!(removal.text(), "Дедлайн удалён: 01.01.2025");
    }
}
