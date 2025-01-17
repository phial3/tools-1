use std::borrow::Cow;

use crate::format_element::normalize_newlines;

use crate::{format_element::Token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsStringLiteralExpression;

impl ToFormatElement for JsStringLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let value_token = self.value_token()?;
        let quoted = value_token.text_trimmed();

        // uses single quotes
        let content = if quoted.starts_with('\'') {
            let s = &quoted[1..quoted.len() - 1];
            let s = format!("\"{}\"", s);
            match normalize_newlines(&s, ['\r']) {
                Cow::Borrowed(_) => s,
                Cow::Owned(s) => s,
            }
        } else {
            normalize_newlines(quoted, ['\r']).into_owned()
        };

        formatter.format_replaced(
            &value_token,
            Token::new_dynamic(content, value_token.text_trimmed_range()).into(),
        )
    }
}
