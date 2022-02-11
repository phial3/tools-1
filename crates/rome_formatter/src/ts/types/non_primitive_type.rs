use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsNonPrimitiveType;
use rslint_parser::ast::TsNonPrimitiveTypeSlots;

impl ToFormatElement for TsNonPrimitiveType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsNonPrimitiveTypeSlots { object_token } = self.as_slots();

        object_token.format(formatter)
    }
}
