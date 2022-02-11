use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    empty_element, format_elements, group_elements, if_group_fits_on_single_line,
    soft_block_indent, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsExportNamedClause;
use rslint_parser::ast::JsExportNamedClauseSlots;

impl ToFormatElement for JsExportNamedClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportNamedClauseSlots {
            l_curly_token,
            specifiers,
            r_curly_token,
            semicolon_token,
        } = self.as_slots();

        let specifiers = specifiers.format(formatter)?;

        let list = group_elements(formatter.format_delimited(
            &l_curly_token?,
            |leading, trailing| {
                let space = if leading.is_empty() && specifiers.is_empty() && trailing.is_empty() {
                    empty_element()
                } else {
                    if_group_fits_on_single_line(space_token())
                };

                Ok(format_elements!(
                    space.clone(),
                    soft_block_indent(format_elements![leading, specifiers, trailing]),
                    space,
                ))
            },
            &r_curly_token?,
        )?);

        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![list, semicolon])
    }
}
