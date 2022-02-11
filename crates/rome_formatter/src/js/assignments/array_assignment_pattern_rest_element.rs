use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsArrayAssignmentPatternRestElement;
use rslint_parser::ast::JsArrayAssignmentPatternRestElementSlots;

impl ToFormatElement for JsArrayAssignmentPatternRestElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsArrayAssignmentPatternRestElementSlots {
            dotdotdot_token,
            pattern,
        } = self.as_slots();

        Ok(format_elements![
            dotdotdot_token.format(formatter)?,
            pattern.format(formatter)?
        ])
    }
}
