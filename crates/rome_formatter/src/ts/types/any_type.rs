use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsAnyType;
use rslint_parser::ast::TsAnyTypeSlots;

impl ToFormatElement for TsAnyType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAnyTypeSlots { any_token } = self.as_slots();

        any_token.format(formatter)
    }
}
