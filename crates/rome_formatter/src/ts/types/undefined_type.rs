use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsUndefinedType;
use rslint_parser::ast::TsUndefinedTypeSlots;

impl ToFormatElement for TsUndefinedType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsUndefinedTypeSlots { undefined_token } = self.as_slots();

        undefined_token.format(formatter)
    }
}
