use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsSymbolType;
use rslint_parser::ast::TsSymbolTypeSlots;

impl ToFormatElement for TsSymbolType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsSymbolTypeSlots { symbol_token } = self.as_slots();

        symbol_token.format(formatter)
    }
}
