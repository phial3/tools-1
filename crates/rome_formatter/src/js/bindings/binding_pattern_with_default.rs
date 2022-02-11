use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsBindingPatternWithDefault;
use rslint_parser::ast::JsBindingPatternWithDefaultSlots;

impl ToFormatElement for JsBindingPatternWithDefault {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBindingPatternWithDefaultSlots {
            pattern,
            eq_token,
            default,
        } = self.as_slots();

        Ok(format_elements![
            pattern.format(formatter)?,
            space_token(),
            eq_token.format(formatter)?,
            space_token(),
            default.format(formatter)?
        ])
    }
}
