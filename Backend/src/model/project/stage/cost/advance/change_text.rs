use crate::model::contract::comment_text::CommentText;
use crate::model::project::stage::cost::formatted_cost::FormattedCost;

pub struct AdvanceCostChangeText {
    old: i32,
    new: Option<i32>,
}

impl AdvanceCostChangeText {
    pub fn new(old: i32, new: Option<i32>) -> Self {
        Self { old, new }
    }
}

impl CommentText for AdvanceCostChangeText {
    fn text(&self) -> String {
        match self.new {
            Some(new) => format!(
                "Аванс изменён: {} ₽ → {} ₽",
                FormattedCost(self.old),
                FormattedCost(new)
            ),
            None => format!("Аванс удалён: {} ₽", FormattedCost(self.old)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_advance_cost_change_and_removal() {
        let change = AdvanceCostChangeText::new(100000, Some(250000));
        assert_eq!(change.text(), "Аванс изменён: 100 000 ₽ → 250 000 ₽");

        let removal = AdvanceCostChangeText::new(100000, None);
        assert_eq!(removal.text(), "Аванс удалён: 100 000 ₽");
    }
}
