use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, group_elements, soft_block_indent, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsCatchDeclaration;
use rslint_parser::ast::JsCatchDeclarationSlots;

impl ToFormatElement for JsCatchDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsCatchDeclarationSlots {
            l_paren_token,
            binding,
            r_paren_token,
        } = self.as_slots();

        Ok(group_elements(formatter.format_delimited(
            &l_paren_token?,
            |open_token_trailing, close_token_leading| {
                Ok(soft_block_indent(format_elements![
                    open_token_trailing,
                    binding.format(formatter)?,
                    close_token_leading,
                ]))
            },
            &r_paren_token?,
        )?))
    }
}
