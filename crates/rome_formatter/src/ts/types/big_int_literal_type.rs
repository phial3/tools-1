use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsBigIntLiteralType;
use rslint_parser::ast::TsBigIntLiteralTypeSlots;

impl ToFormatElement for TsBigIntLiteralType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsBigIntLiteralTypeSlots {
            minus_token,
            literal_token,
        } = self.as_slots();

        let minus = minus_token.format_or_empty(formatter)?;
        let literal = literal_token.format(formatter)?;
        Ok(format_elements![minus, literal])
    }
}
