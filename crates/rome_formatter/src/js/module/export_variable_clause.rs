use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsExportVariableClause;
use rslint_parser::ast::JsExportVariableClauseSlots;

impl ToFormatElement for JsExportVariableClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportVariableClauseSlots {
            declaration,
            semicolon_token,
        } = self.as_slots();

        let declarations = declaration.format(formatter)?;
        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![declarations, semicolon])
    }
}
