use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsThisType;
use rslint_parser::ast::TsThisTypeSlots;

impl ToFormatElement for TsThisType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsThisTypeSlots { this_token } = self.as_slots();

        this_token.format(formatter)
    }
}
