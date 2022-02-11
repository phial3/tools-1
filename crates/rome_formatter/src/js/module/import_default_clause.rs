use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsImportDefaultClause;
use rslint_parser::ast::JsImportDefaultClauseSlots;

impl ToFormatElement for JsImportDefaultClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportDefaultClauseSlots {
            local_name,
            from_token,
            source,
            assertion,
        } = self.as_slots();

        let local_name = local_name.format(formatter)?;
        let from = from_token.format(formatter)?;
        let source = source.format(formatter)?;
        let assertion = assertion.format_with_or_empty(formatter, |assertion| {
            format_elements![space_token(), assertion]
        })?;

        Ok(format_elements![
            local_name,
            space_token(),
            from,
            space_token(),
            source,
            assertion
        ])
    }
}
