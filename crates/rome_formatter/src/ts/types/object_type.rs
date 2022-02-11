use crate::{
    format_elements, group_elements, soft_block_indent, soft_line_break_or_space, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsObjectType;
use rslint_parser::ast::TsObjectTypeSlots;

impl ToFormatElement for TsObjectType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsObjectTypeSlots {
            l_curly_token,
            members,
            r_curly_token,
        } = self.as_slots();

        Ok(group_elements(formatter.format_delimited(
            &l_curly_token?,
            |open_token_trailing, close_token_leading| {
                let list = members.to_format_element(formatter)?;
                Ok(format_elements![
                    soft_line_break_or_space(),
                    soft_block_indent(format_elements![
                        open_token_trailing,
                        list,
                        close_token_leading
                    ]),
                    soft_line_break_or_space()
                ])
            },
            &r_curly_token?,
        )?))
    }
}
