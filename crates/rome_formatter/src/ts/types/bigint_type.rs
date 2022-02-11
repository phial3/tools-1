use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsBigintType;
use rslint_parser::ast::TsBigintTypeSlots;

impl ToFormatElement for TsBigintType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsBigintTypeSlots { bigint_token } = self.as_slots();

        bigint_token.format(formatter)
    }
}
