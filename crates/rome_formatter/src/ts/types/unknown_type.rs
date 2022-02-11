use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsUnknownType;
use rslint_parser::ast::TsUnknownTypeSlots;

impl ToFormatElement for TsUnknownType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsUnknownTypeSlots { unknown_token } = self.as_slots();

        unknown_token.format(formatter)
    }
}
