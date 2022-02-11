use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsExtendsClause;
use rslint_parser::ast::JsExtendsClauseSlots;

impl ToFormatElement for JsExtendsClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExtendsClauseSlots {
            extends_token,
            super_class,
            type_arguments,
        } = self.as_slots();

        Ok(format_elements![
            extends_token.format(formatter)?,
            space_token(),
            super_class.format(formatter)?,
            type_arguments.format_or_empty(formatter)?,
        ])
    }
}
