use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsCatchClause;
use rslint_parser::ast::JsCatchClauseSlots;

impl ToFormatElement for JsCatchClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsCatchClauseSlots {
            catch_token,
            declaration,
            body,
        } = self.as_slots();

        declaration.format_with_or(
            formatter,
            |declaration| {
                Ok(format_elements![
                    catch_token.format(formatter)?,
                    space_token(),
                    declaration,
                    space_token(),
                    body.format(formatter)?
                ])
            },
            || {
                Ok(format_elements![
                    catch_token.format(formatter)?,
                    space_token(),
                    body.format(formatter)?
                ])
            },
        )
    }
}
