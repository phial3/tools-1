use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsNullLiteralType;
use rslint_parser::ast::TsNullLiteralTypeSlots;

impl ToFormatElement for TsNullLiteralType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsNullLiteralTypeSlots { literal_token } = self.as_slots();

        literal_token.format(formatter)
    }
}
