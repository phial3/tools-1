use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsArrayBindingPatternRestElement;
use rslint_parser::ast::JsArrayBindingPatternRestElementSlots;

impl ToFormatElement for JsArrayBindingPatternRestElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsArrayBindingPatternRestElementSlots {
            dotdotdot_token,
            pattern,
        } = self.as_slots();

        Ok(format_elements![
            dotdotdot_token.format(formatter)?,
            pattern.format(formatter)?,
        ])
    }
}
