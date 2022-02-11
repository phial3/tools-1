use crate::{
    format_elements, group_elements, join_elements, soft_block_indent, soft_line_break,
    soft_line_break_or_space, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsTypeParameters;
use rslint_parser::ast::TsTypeParametersSlots;

impl ToFormatElement for TsTypeParameters {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeParametersSlots {
            l_angle_token,
            items,
            r_angle_token,
        } = self.as_slots();

        let items = formatter.format_separated(items, || token(","))?;

        Ok(group_elements(formatter.format_delimited(
            &l_angle_token?,
            |open_token_trailing, close_token_leading| {
                Ok(format_elements![
                    soft_line_break(),
                    soft_block_indent(format_elements![
                        open_token_trailing,
                        join_elements(soft_line_break_or_space(), items),
                        close_token_leading,
                    ]),
                    soft_line_break()
                ])
            },
            &r_angle_token?,
        )?))
    }
}
