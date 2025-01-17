use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsTypeParameterName;

impl ToFormatElement for TsTypeParameterName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.ident_token().format(formatter)
    }
}
