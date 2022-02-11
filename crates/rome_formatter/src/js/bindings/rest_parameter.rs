use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsRestParameter;
use rslint_parser::ast::JsRestParameterSlots;

impl ToFormatElement for JsRestParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsRestParameterSlots {
            dotdotdot_token,
            binding,
            type_annotation,
        } = self.as_slots();

        Ok(format_elements![
            dotdotdot_token.format(formatter)?,
            binding.format(formatter)?,
            type_annotation.format_or_empty(formatter)?
        ])
    }
}
