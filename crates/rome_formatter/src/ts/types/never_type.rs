use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsNeverType;
use rslint_parser::ast::TsNeverTypeSlots;

impl ToFormatElement for TsNeverType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsNeverTypeSlots { never_token } = self.as_slots();

        never_token.format(formatter)
    }
}
