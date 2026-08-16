use crate::model::contract::comment_text::CommentText;

pub struct AdvancePaymentConfirmationText {
    confirmed: bool,
}

impl AdvancePaymentConfirmationText {
    pub fn new(confirmed: bool) -> Self {
        Self { confirmed }
    }
}

impl CommentText for AdvancePaymentConfirmationText {
    fn text(&self) -> String {
        if self.confirmed {
            "Аванс подтверждён".to_string()
        } else {
            "Подтверждение аванса снято".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_advance_payment_confirmation() {
        assert_eq!(
            AdvancePaymentConfirmationText::new(true).text(),
            "Аванс подтверждён"
        );
        assert_eq!(
            AdvancePaymentConfirmationText::new(false).text(),
            "Подтверждение аванса снято"
        );
    }
}
