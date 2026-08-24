use std::fmt;

pub struct FormattedCost(pub i32);

impl fmt::Display for FormattedCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.0.to_string();
        let bytes = s.as_bytes();
        let len = bytes.len();
        for (i, &b) in bytes.iter().enumerate() {
            if i > 0 && (len - i).is_multiple_of(3) {
                write!(f, " ")?;
            }
            write!(f, "{}", b as char)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_cost_with_thousand_separators() {
        assert_eq!(FormattedCost(1000).to_string(), "1 000");
        assert_eq!(FormattedCost(1000000).to_string(), "1 000 000");
        assert_eq!(FormattedCost(500).to_string(), "500");
    }
}
