use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    empty_element, format_elements, group_elements, if_group_fits_on_single_line,
    soft_block_indent, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsExportNamedFromClause;
use rslint_parser::ast::JsExportNamedFromClauseSlots;

impl ToFormatElement for JsExportNamedFromClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportNamedFromClauseSlots {
            l_curly_token,
            specifiers,
            r_curly_token,
            from_token,
            source,
            assertion,
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

        let from = from_token.format(formatter)?;
        let source = source.format(formatter)?;
        let assertion = assertion.format_with_or_empty(formatter, |assertion| {
            format_elements![space_token(), assertion]
        })?;
        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![
            list,
            space_token(),
            from,
            space_token(),
            source,
            space_token(),
            assertion,
            semicolon
        ])
    }
}
