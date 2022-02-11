use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsImportBareClause;
use rslint_parser::ast::JsImportBareClauseSlots;

impl ToFormatElement for JsImportBareClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportBareClauseSlots { source, assertion } = self.as_slots();

        let source = source.format(formatter)?;
        let assertion = assertion.format_with_or_empty(formatter, |assertion| {
            format_elements![space_token(), assertion]
        })?;

        Ok(format_elements![source, assertion])
    }
}
