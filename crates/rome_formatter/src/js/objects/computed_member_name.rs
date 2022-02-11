use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsComputedMemberName;
use rslint_parser::ast::JsComputedMemberNameSlots;

impl ToFormatElement for JsComputedMemberName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsComputedMemberNameSlots {
            l_brack_token,
            expression,
            r_brack_token,
        } = self.as_slots();

        Ok(format_elements![
            l_brack_token.format(formatter)?,
            expression.format(formatter)?,
            r_brack_token.format(formatter)?,
        ])
    }
}
