use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsBooleanLiteralType;
use rslint_parser::ast::TsBooleanLiteralTypeSlots;

impl ToFormatElement for TsBooleanLiteralType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsBooleanLiteralTypeSlots { literal } = self.as_slots();

        literal.format(formatter)
    }
}
