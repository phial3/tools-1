use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsLabeledStatement;
use rslint_parser::ast::JsLabeledStatementSlots;

impl ToFormatElement for JsLabeledStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsLabeledStatementSlots {
            label_token,
            colon_token,
            body,
        } = self.as_slots();

        let label = label_token.format(formatter)?;
        let colon = colon_token.format(formatter)?;
        let statement = body.format(formatter)?;

        Ok(format_elements![label, colon, space_token(), statement])
    }
}
