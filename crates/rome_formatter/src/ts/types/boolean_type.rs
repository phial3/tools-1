use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsBooleanType;
use rslint_parser::ast::TsBooleanTypeSlots;

impl ToFormatElement for TsBooleanType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsBooleanTypeSlots { boolean_token } = self.as_slots();

        boolean_token.format(formatter)
    }
}
