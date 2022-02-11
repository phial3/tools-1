use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsTypeofType;
use rslint_parser::ast::TsTypeofTypeSlots;

impl ToFormatElement for TsTypeofType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeofTypeSlots {
            typeof_token,
            expression_name,
        } = self.as_slots();

        let r#typeof = typeof_token.format(formatter)?;
        let expression_name = expression_name.format(formatter)?;
        Ok(format_elements![r#typeof, space_token(), expression_name])
    }
}
