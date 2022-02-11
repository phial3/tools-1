use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsTypeAliasDeclaration;
use rslint_parser::ast::TsTypeAliasDeclarationSlots;

impl ToFormatElement for TsTypeAliasDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeAliasDeclarationSlots {
            declare_token,
            type_token,
            binding_identifier,
            type_parameters,
            eq_token,
            ty,
            semicolon_token,
        } = self.as_slots();

        let declare_token = declare_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let type_token = type_token.format(formatter)?;
        let binding_identifier = binding_identifier.format(formatter)?;
        let type_parameters = type_parameters.format_or_empty(formatter)?;
        let equal_token = eq_token.format(formatter)?;
        let ty = ty.format(formatter)?;
        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![
            declare_token,
            type_token,
            space_token(),
            binding_identifier,
            type_parameters,
            equal_token,
            space_token(),
            ty,
            semicolon
        ])
    }
}
