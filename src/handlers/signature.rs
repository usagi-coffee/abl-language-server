use std::collections::HashSet;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    ParameterInformation, ParameterLabel, SignatureHelp, SignatureHelpParams, SignatureInformation,
};
use tree_sitter::Node;

use crate::analysis::functions::{FunctionSignature, find_function_signature};
use crate::analysis::includes::collect_include_sites;
use crate::analysis::scopes::containing_scope;
use crate::backend::Backend;
use crate::utils::position::lsp_pos_to_utf8_byte_offset;

impl Backend {
    pub async fn handle_signature_help(
        &self,
        params: SignatureHelpParams,
    ) -> Result<Option<SignatureHelp>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let text = match self.get_document_text(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };
        let tree = match self.get_document_tree_or_parse(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };

        let Some(offset) = lsp_pos_to_utf8_byte_offset(&text, pos) else {
            return Ok(None);
        };

        let Some(call) = call_context_at_offset(tree.root_node(), text.as_bytes(), offset) else {
            return Ok(None);
        };

        let local_sig = find_function_signature(tree.root_node(), text.as_bytes(), &call.name);
        let sig = match local_sig {
            Some(sig) => sig,
            None => match self
                .find_function_signature_from_includes_for_signature_help(
                    &uri,
                    &text,
                    tree.root_node(),
                    offset,
                    &call.name,
                )
                .await
            {
                Some(sig) => sig,
                None => return Ok(None),
            },
        };

        let sig_info = to_signature_information(&sig);
        let active_param = if sig.params.is_empty() {
            None
        } else {
            Some((call.active_param.min(sig.params.len().saturating_sub(1))) as u32)
        };

        Ok(Some(SignatureHelp {
            signatures: vec![sig_info],
            active_signature: Some(0),
            active_parameter: active_param,
        }))
    }

    async fn find_function_signature_from_includes_for_signature_help(
        &self,
        uri: &tower_lsp::lsp_types::Url,
        text: &str,
        root: Node<'_>,
        offset: usize,
        symbol: &str,
    ) -> Option<FunctionSignature> {
        let scope = containing_scope(root, offset)?;
        let current_path = uri.to_file_path().ok()?;
        let include_sites = collect_include_sites(text);
        let mut seen_files = HashSet::new();

        for include in include_sites {
            if include.start_offset < scope.start || include.start_offset > scope.end {
                continue;
            }
            let Some(include_path) = self
                .resolve_include_path_for(&current_path, &include.path)
                .await
            else {
                continue;
            };
            if !seen_files.insert(include_path.clone()) {
                continue;
            }
            let Some((include_text, include_tree)) =
                self.get_cached_include_parse(&include_path).await
            else {
                continue;
            };
            if let Some(sig) =
                find_function_signature(include_tree.root_node(), include_text.as_bytes(), symbol)
            {
                return Some(sig);
            }
        }

        None
    }
}

struct CallContext {
    name: String,
    active_param: usize,
}

fn call_context_at_offset(root: Node<'_>, src: &[u8], offset: usize) -> Option<CallContext> {
    call_context_from_tree(root, src, offset).or_else(|| call_context_from_text(src, offset))
}

fn call_context_from_tree(root: Node<'_>, src: &[u8], offset: usize) -> Option<CallContext> {
    if src.is_empty() {
        return None;
    }
    let mut probe = offset.saturating_sub(1).min(src.len().saturating_sub(1));
    while probe > 0 && src[probe].is_ascii_whitespace() {
        probe = probe.saturating_sub(1);
    }
    let mut node = root.descendant_for_byte_range(probe, probe)?;

    loop {
        if node.kind() == "function_call" {
            let function = node.child_by_field_name("function")?;
            let name = function.utf8_text(src).ok()?.trim().to_string();
            if name.is_empty() {
                return None;
            }

            if let Some(arguments) = node
                .children(&mut node.walk())
                .find(|n| n.kind() == "arguments")
            {
                let start = arguments.start_byte();
                let end = arguments.end_byte();
                if offset >= start.saturating_add(1) && offset <= end {
                    let active_param = count_active_argument_index(src, start, end, offset);
                    return Some(CallContext { name, active_param });
                }
            }
        }
        let Some(parent) = node.parent() else {
            break;
        };
        node = parent;
    }
    None
}

fn call_context_from_text(src: &[u8], offset: usize) -> Option<CallContext> {
    if src.is_empty() {
        return None;
    }
    let mut i = offset.min(src.len());
    let mut depth = 0usize;
    let mut in_string = false;

    while i > 0 {
        i -= 1;
        let b = src[i];
        if in_string {
            if b == b'"' {
                in_string = false;
            }
            continue;
        }
        match b {
            b'"' => in_string = true,
            b')' | b']' | b'}' => depth += 1,
            b'(' | b'[' | b'{' => {
                if depth == 0 {
                    if b != b'(' {
                        continue;
                    }
                    let (name, _) = extract_call_name_before_open_paren(src, i)?;
                    let active_param = count_active_argument_index(src, i, offset, offset);
                    if !name.is_empty() {
                        return Some(CallContext { name, active_param });
                    }
                    return None;
                }
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
    }

    None
}

fn extract_call_name_before_open_paren(src: &[u8], open_paren: usize) -> Option<(String, usize)> {
    if open_paren == 0 {
        return None;
    }
    let mut end = open_paren;
    while end > 0 && src[end - 1].is_ascii_whitespace() {
        end -= 1;
    }
    if end == 0 {
        return None;
    }

    let mut start = end;
    while start > 0 {
        let c = src[start - 1];
        let is_name = c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-' | b'.' | b':');
        if !is_name {
            break;
        }
        start -= 1;
    }
    if start == end {
        return None;
    }
    let name = std::str::from_utf8(&src[start..end])
        .ok()?
        .trim()
        .to_string();
    if name.is_empty() {
        None
    } else {
        Some((name, start))
    }
}

fn count_active_argument_index(
    src: &[u8],
    args_start: usize,
    args_end: usize,
    offset: usize,
) -> usize {
    if args_start >= src.len() {
        return 0;
    }
    let scan_end = offset.min(args_end).min(src.len());
    if scan_end <= args_start {
        return 0;
    }

    let mut idx = 0usize;
    let mut depth = 0usize;
    let mut in_string = false;
    let mut i = args_start.saturating_add(1);

    while i < scan_end {
        let b = src[i];
        if in_string {
            if b == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        match b {
            b'"' => in_string = true,
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            b',' if depth == 0 => idx += 1,
            _ => {}
        }
        i += 1;
    }

    idx
}

fn to_signature_information(sig: &FunctionSignature) -> SignatureInformation {
    let params_text = sig.params.join(", ");
    let label = match sig.return_type.as_deref() {
        Some(ret) => format!("FUNCTION {}({}) RETURNS {}", sig.name, params_text, ret),
        None => format!("FUNCTION {}({})", sig.name, params_text),
    };
    let parameters = sig
        .params
        .iter()
        .map(|p| ParameterInformation {
            label: ParameterLabel::Simple(p.clone()),
            documentation: None,
        })
        .collect::<Vec<_>>();

    SignatureInformation {
        label,
        documentation: None,
        parameters: Some(parameters),
        active_parameter: None,
    }
}

#[cfg(test)]
mod tests {
    use super::{call_context_at_offset, count_active_argument_index};
    use crate::analysis::functions::find_function_signature;

    fn parse(src: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_abl::LANGUAGE.into())
            .expect("set abl language");
        parser.parse(src, None).expect("parse source")
    }

    #[test]
    fn detects_call_context_and_active_param_on_complete_call() {
        let src = r#"
FUNCTION local_mul RETURNS INTEGER (INPUT p_a AS INTEGER, INPUT p_b AS INTEGER):
  RETURN p_a * p_b.
END FUNCTION.
DEFINE VARIABLE lv_counter AS INTEGER NO-UNDO.
lv_counter = local_mul(lv_counter, 2).
"#;
        let tree = parse(src);
        let offset = src.find("lv_counter, 2").expect("arg span") + "lv_counter, ".len();
        let call =
            call_context_at_offset(tree.root_node(), src.as_bytes(), offset).expect("call context");
        assert_eq!(call.name.to_ascii_lowercase(), "local_mul");
        assert_eq!(call.active_param, 1);
    }

    #[test]
    fn detects_call_context_while_typing_after_comma_without_closing_paren() {
        let src = r#"
FUNCTION local_mul RETURNS INTEGER (INPUT p_a AS INTEGER, INPUT p_b AS INTEGER):
  RETURN p_a * p_b.
END FUNCTION.
lv_counter = local_mul(lv_counter, 
"#;
        let tree = parse(src);
        let offset = src.len();
        let call =
            call_context_at_offset(tree.root_node(), src.as_bytes(), offset).expect("call context");
        assert_eq!(call.name.to_ascii_lowercase(), "local_mul");
        assert_eq!(call.active_param, 1);
    }

    #[test]
    fn counts_argument_index_with_nested_calls() {
        let src = b"foo(a, bar(1, 2), c)";
        let args_start = src.iter().position(|b| *b == b'(').expect("start");
        let args_end = src.len() - 1;
        let offset = src.len() - 2;
        let idx = count_active_argument_index(src, args_start, args_end, offset);
        assert_eq!(idx, 2);
    }

    #[test]
    fn finds_function_signature_for_call_name() {
        let src = r#"
FUNCTION local_mul RETURNS INTEGER (INPUT p_a AS INTEGER, INPUT p_b AS INTEGER):
  RETURN p_a * p_b.
END FUNCTION.
lv_counter = local_mul(1, 2).
"#;
        let tree = parse(src);
        let sig =
            find_function_signature(tree.root_node(), src.as_bytes(), "local_mul").expect("sig");
        assert_eq!(sig.params.len(), 2);
        assert_eq!(sig.return_type.as_deref(), Some("INTEGER"));
    }
}
