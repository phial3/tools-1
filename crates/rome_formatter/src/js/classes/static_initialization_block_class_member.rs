use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    block_indent, format_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsStaticInitializationBlockClassMember;
use rslint_parser::ast::JsStaticInitializationBlockClassMemberSlots;

impl ToFormatElement for JsStaticInitializationBlockClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStaticInitializationBlockClassMemberSlots {
            static_token,
            l_curly_token,
            statements,
            r_curly_token,
        } = self.as_slots();

        let static_token = static_token.format(formatter)?;
        let separated = formatter.format_delimited(
            &l_curly_token?,
            |open_token_trailing, close_token_leading| {
                Ok(block_indent(format_elements![
                    open_token_trailing,
                    formatter.format_list(statements),
                    close_token_leading,
                ]))
            },
            &r_curly_token?,
        )?;
        Ok(format_elements![static_token, space_token(), separated])
    }
}
