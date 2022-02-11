use crate::utils::format_interpreter;
use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, hard_line_break, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsModule;
use rslint_parser::ast::JsModuleSlots;

impl ToFormatElement for JsModule {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsModuleSlots {
            interpreter_token,
            directives,
            items,
            eof_token,
        } = self.as_slots();

        Ok(format_elements![
            format_interpreter(interpreter_token, formatter)?,
            directives.format(formatter)?,
            formatter.format_list(items),
            eof_token.format(formatter)?,
            hard_line_break()
        ])
    }
}
