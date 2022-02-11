use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsStringType;
use rslint_parser::ast::TsStringTypeSlots;

impl ToFormatElement for TsStringType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsStringTypeSlots { string_token } = self.as_slots();

        string_token.format(formatter)
    }
}
