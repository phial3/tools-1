use crate::{
    block_indent, format_elements, formatter_traits::FormatTokenAndNode, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsFunctionBody;
use rslint_parser::ast::JsFunctionBodySlots;

impl ToFormatElement for JsFunctionBody {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsFunctionBodySlots {
            l_curly_token,
            directives,
            statements,
            r_curly_token,
        } = self.as_slots();

        formatter.format_delimited(
            &l_curly_token?,
            |open_token_trailing, close_token_leading| {
                Ok(block_indent(format_elements![
                    open_token_trailing,
                    directives.format(formatter)?,
                    formatter.format_list(statements),
                    close_token_leading,
                ]))
            },
            &r_curly_token?,
        )
    }
}
