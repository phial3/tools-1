use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, group_elements, join_elements, soft_block_indent, soft_line_break_or_space,
    space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsEnumDeclaration;
use rslint_parser::ast::TsEnumDeclarationSlots;

impl ToFormatElement for TsEnumDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsEnumDeclarationSlots {
            declare_token,
            const_token,
            enum_token,
            id,
            l_curly_token,
            members,
            r_curly_token,
        } = self.as_slots();

        let declare_token = declare_token.format_with_or_empty(formatter, |declare_token| {
            format_elements![declare_token, space_token()]
        })?;
        let const_token = const_token.format_with_or_empty(formatter, |const_token| {
            format_elements![const_token, space_token()]
        })?;
        let enum_token = enum_token.format_with(formatter, |enum_token| {
            format_elements![enum_token, space_token()]
        })?;
        let id = id.format_with(formatter, |id| format_elements![id, space_token()])?;

        let members = formatter.format_separated(members, || token(","))?;
        let list = group_elements(formatter.format_delimited(
            &l_curly_token?,
            |open_token_trailing, close_token_leading| {
                Ok(format_elements![
                    soft_line_break_or_space(),
                    soft_block_indent(format_elements![
                        open_token_trailing,
                        join_elements(soft_line_break_or_space(), members),
                        close_token_leading,
                    ]),
                    soft_line_break_or_space(),
                ])
            },
            &r_curly_token?,
        )?);

        Ok(format_elements![
            declare_token,
            const_token,
            enum_token,
            id,
            list,
        ])
    }
}
