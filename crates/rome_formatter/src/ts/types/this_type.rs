use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsThisType;

impl ToFormatElement for TsThisType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.this_token().format(formatter)
    }
}
