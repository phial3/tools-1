use lspower::lsp;
use std::sync::Arc;
use tracing::error;

use crate::{documents::Document, session::Session};

/// Handler for `textDocument/didOpen` LSP notification
pub(crate) async fn did_open(session: Arc<Session>, params: lsp::DidOpenTextDocumentParams) {
    let url = params.text_document.uri.clone();
    let file_id = session.file_id(url.clone());
    let version = params.text_document.version;
    let language_id = match params.text_document.language_id.as_str().try_into() {
        Ok(id) => id,
        Err(err) => return error!("{}", err),
    };

    let doc = Document::new(file_id, language_id, version, params.text_document.text);
    session.insert_document(url.clone(), doc);

    if let Err(err) = session.update_diagnostics(url).await {
        error!("Failed to update diagnostics: {}", err);
    }
}

/// Handler for `textDocument/didChange` LSP notification
pub(crate) async fn did_change(session: Arc<Session>, params: lsp::DidChangeTextDocumentParams) {
    let url = params.text_document.uri;
    let version = params.text_document.version;

    let doc = match session.document(&url) {
        Ok(doc) => doc,
        Err(err) => return error!("{}", err),
    };

    // Because of TextDocumentSyncKind::Full, there should only be one change.
    let mut content_changes = params.content_changes;
    let text = match content_changes.pop() {
        Some(change) => change.text,
        None => return error!("Invalid textDocument/didChange for {:?}", url),
    };

    let doc = Document::new(doc.file_id, doc.language_id, version, text);
    session.insert_document(url.clone(), doc);

    if let Err(err) = session.update_diagnostics(url).await {
        error!("Failed to update diagnostics: {}", err);
    }
}

/// Handler for `textDocument/didClose` LSP notification
pub(crate) async fn did_close(session: Arc<Session>, params: lsp::DidCloseTextDocumentParams) {
    let url = params.text_document.uri;
    session.remove_document(&url);
    let diagnostics = vec![];
    let version = None;
    session
        .client
        .publish_diagnostics(url, diagnostics, version)
        .await;
}
