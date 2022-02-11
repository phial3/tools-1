use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsStringLiteralType;
use rslint_parser::ast::TsStringLiteralTypeSlots;

impl ToFormatElement for TsStringLiteralType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsStringLiteralTypeSlots { literal_token } = self.as_slots();

        literal_token.format(formatter)
    }
}
