use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsNumberType;
use rslint_parser::ast::TsNumberTypeSlots;

impl ToFormatElement for TsNumberType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsNumberTypeSlots { number_token } = self.as_slots();

        number_token.format(formatter)
    }
}
