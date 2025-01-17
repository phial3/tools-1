//! Top level functions for parsing a script or module, also includes module specific items.

use super::module::parse_module_body;
use super::stmt::parse_statements;
use crate::state::{ChangeParserState, EnableStrictMode};
use crate::syntax::stmt::directives;
use crate::{JsSyntaxKind::*, *};

// test_err unterminated_unicode_codepoint
// let s = "\u{200";

pub fn parse(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.eat(JS_SHEBANG);

    let strict_snapshot = directives(p);

    let result = match p.syntax.file_kind {
        FileKind::Script => {
            parse_statements(p, false);
            m.complete(p, JS_SCRIPT)
        }
        FileKind::Module | FileKind::TypeScript => parse_module_body(p, m),
    };

    if let Some(strict_snapshot) = strict_snapshot {
        EnableStrictMode::restore(&mut p.state, strict_snapshot);
    }

    result
}
