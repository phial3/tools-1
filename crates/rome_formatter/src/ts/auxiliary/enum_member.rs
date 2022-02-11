use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::format_initializer_clause;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsEnumMember;
use rslint_parser::ast::TsEnumMemberSlots;

impl ToFormatElement for TsEnumMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsEnumMemberSlots { name, initializer } = self.as_slots();

        let name = name.format(formatter)?;
        let initializer = format_initializer_clause(formatter, initializer)?;

        Ok(format_elements![name, initializer])
    }
}
