//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsAnyModuleName;
impl ToFormatElement for TsAnyModuleName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsIdentifierBinding(node) => node.to_format_element(formatter),
            Self::TsQualifiedModuleName(node) => node.to_format_element(formatter),
        }
    }
}
