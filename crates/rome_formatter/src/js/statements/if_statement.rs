use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, soft_block_indent, space_token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};

use rslint_parser::ast::JsIfStatement;
use rslint_parser::ast::JsIfStatementSlots;

impl ToFormatElement for JsIfStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsIfStatementSlots {
            if_token,
            l_paren_token,
            test,
            r_paren_token,
            consequent,
            else_clause,
        } = self.as_slots();

        let else_caluse = else_clause.format_with_or_empty(formatter, |else_clause| {
            format_elements![space_token(), else_clause]
        })?;

        Ok(format_elements![
            group_elements(format_elements![
                if_token.format(formatter)?,
                space_token(),
                group_elements(formatter.format_delimited(
                    &l_paren_token?,
                    |open_token_trailing, close_token_leading| Ok(soft_block_indent(
                        format_elements![
                            open_token_trailing,
                            test.format(formatter)?,
                            close_token_leading
                        ]
                    )),
                    &r_paren_token?,
                )?),
                space_token(),
            ]),
            consequent.format(formatter)?,
            else_caluse
        ])
    }
}
