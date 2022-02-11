use crate::formatter_traits::FormatTokenAndNode;

use crate::{block_indent, FormatResult};

use crate::{
    format_elements, group_elements, join_elements_hard_line, soft_block_indent, space_token,
    FormatElement, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsSwitchStatement;
use rslint_parser::ast::JsSwitchStatementSlots;

impl ToFormatElement for JsSwitchStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsSwitchStatementSlots {
            switch_token,
            l_paren_token,
            discriminant,
            r_paren_token,
            l_curly_token,
            cases,
            r_curly_token,
        } = self.as_slots();

        Ok(format_elements![
            switch_token.format(formatter)?,
            space_token(),
            group_elements(formatter.format_delimited(
                &l_paren_token?,
                |open_token_trailing, close_token_leading| Ok(soft_block_indent(format_elements![
                    open_token_trailing,
                    discriminant.format(formatter)?,
                    close_token_leading,
                ])),
                &r_paren_token?,
            )?),
            space_token(),
            group_elements(formatter.format_delimited(
                &l_curly_token?,
                |open_token_trailing, close_token_leading| {
                    Ok(block_indent(format_elements![
                        open_token_trailing,
                        join_elements_hard_line(
                            cases
                                .clone()
                                .into_iter()
                                .zip(formatter.format_nodes(cases)?)
                        ),
                        close_token_leading,
                    ]))
                },
                &r_curly_token?
            )?)
        ])
    }
}
