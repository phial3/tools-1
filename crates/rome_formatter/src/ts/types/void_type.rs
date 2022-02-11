use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsVoidType;
use rslint_parser::ast::TsVoidTypeSlots;

impl ToFormatElement for TsVoidType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsVoidTypeSlots { void_token } = self.as_slots();

        void_token.format(formatter)
    }
}
