use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::NewTarget;
use rslint_parser::ast::NewTargetSlots;

impl ToFormatElement for NewTarget {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let NewTargetSlots {
            new_token,
            dot_token,
            target_token,
        } = self.as_slots();

        Ok(format_elements![
            new_token.format(formatter)?,
            dot_token.format(formatter)?,
            target_token.format(formatter)?,
        ])
    }
}
