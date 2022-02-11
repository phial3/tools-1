use rslint_parser::ast::ImportMeta;

use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::ImportMetaSlots;

impl ToFormatElement for ImportMeta {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let ImportMetaSlots {
            import_token,
            dot_token,
            meta_token,
        } = self.as_slots();

        Ok(format_elements![
            import_token.format(formatter)?,
            dot_token.format(formatter)?,
            meta_token.format(formatter)?,
        ])
    }
}
