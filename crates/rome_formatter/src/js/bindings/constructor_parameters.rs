use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, group_elements, soft_block_indent,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsConstructorParameters;
use rslint_parser::ast::JsConstructorParametersSlots;

impl ToFormatElement for JsConstructorParameters {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsConstructorParametersSlots {
            l_paren_token,
            parameters,
            r_paren_token,
        } = self.as_slots();

        Ok(group_elements(formatter.format_delimited(
            &l_paren_token?,
            |open_token_trailing, close_token_leading| {
                Ok(soft_block_indent(format_elements![
                    open_token_trailing,
                    parameters.format(formatter)?,
                    close_token_leading,
                ]))
            },
            &r_paren_token?,
        )?))
    }
}
