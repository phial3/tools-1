use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsMethodClassMember;
use rslint_parser::ast::JsMethodClassMemberSlots;

impl ToFormatElement for JsMethodClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsMethodClassMemberSlots {
            access_modifier,
            static_token,
            abstract_token,
            async_token,
            star_token,
            name,
            question_mark_token,
            type_parameters,
            parameters,
            return_type_annotation,
            body,
        } = self.as_slots();

        let access_modifier = access_modifier
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let static_token = static_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let abstract_token = abstract_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let async_token = async_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;

        let star_token = star_token.format_or_empty(formatter)?;
        let name = name.format(formatter)?;
        let question_mark_token = question_mark_token.format_or_empty(formatter)?;
        let type_parameters = type_parameters.format_or_empty(formatter)?;
        let params = parameters.format(formatter)?;
        let return_type_annotation = return_type_annotation.format_or_empty(formatter)?;
        let body = body.format(formatter)?;

        Ok(format_elements![
            access_modifier,
            static_token,
            abstract_token,
            async_token,
            star_token,
            name,
            question_mark_token,
            type_parameters,
            params,
            return_type_annotation,
            space_token(),
            body
        ])
    }
}
