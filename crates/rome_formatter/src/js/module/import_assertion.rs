use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    empty_element, format_elements, group_elements, soft_block_indent, space_token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsImportAssertion;
use rslint_parser::ast::JsImportAssertionSlots;

impl ToFormatElement for JsImportAssertion {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportAssertionSlots {
            assert_token,
            l_curly_token,
            assertions,
            r_curly_token,
        } = self.as_slots();

        let assert_token = assert_token.format(formatter)?;
        let assertions = assertions.format(formatter)?;

        let result = group_elements(formatter.format_delimited(
            &l_curly_token?,
            |leading, trailing| {
                let space = if leading.is_empty() && assertions.is_empty() && trailing.is_empty() {
                    empty_element()
                } else {
                    space_token()
                };

                Ok(format_elements!(
                    space.clone(),
                    soft_block_indent(format_elements![leading, assertions, trailing]),
                    space,
                ))
            },
            &r_curly_token?,
        )?);

        Ok(format_elements![assert_token, space_token(), result])
    }
}
