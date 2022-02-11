use crate::utils::format_interpreter;
use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, hard_line_break, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsScript;
use rslint_parser::ast::JsScriptSlots;

impl ToFormatElement for JsScript {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsScriptSlots {
            interpreter_token,
            directives,
            statements,
            eof_token,
        } = self.as_slots();

        Ok(format_elements![
            format_interpreter(interpreter_token, formatter)?,
            directives.format(formatter)?,
            formatter.format_list(statements),
            eof_token.format(formatter)?,
            hard_line_break()
        ])
    }
}
