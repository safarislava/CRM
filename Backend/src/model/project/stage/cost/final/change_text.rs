use crate::model::contract::comment_text::CommentText;
use crate::model::project::stage::cost::formatted_cost::FormattedCost;

pub struct FinalCostChangeText {
    old: i32,
    new: Option<i32>,
}

impl FinalCostChangeText {
    pub fn new(old: i32, new: Option<i32>) -> Self {
        Self { old, new }
    }
}

impl CommentText for FinalCostChangeText {
    fn text(&self) -> String {
        match self.new {
            Some(new) => format!(
                "Окончательная оплата изменена: {} ₽ → {} ₽",
                FormattedCost(self.old),
                FormattedCost(new)
            ),
            None => format!(
                "Окончательная оплата удалена: {} ₽",
                FormattedCost(self.old)
            ),
        }
    }
}
