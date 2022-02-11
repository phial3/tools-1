use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsComputedMemberAssignment;
use rslint_parser::ast::JsComputedMemberAssignmentSlots;

impl ToFormatElement for JsComputedMemberAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsComputedMemberAssignmentSlots {
            object,
            l_brack_token,
            member,
            r_brack_token,
        } = self.as_slots();

        Ok(format_elements![
            object.format(formatter)?,
            l_brack_token.format(formatter)?,
            member.format(formatter)?,
            r_brack_token.format(formatter)?,
        ])
    }
}
