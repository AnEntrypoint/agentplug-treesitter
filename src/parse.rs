use tree_sitter::{Language, Parser};

use crate::abi::return_json;

fn lang_for_ext(ext: &str) -> Option<(&'static str, Language)> {
    let e = ext.to_lowercase();
    match e.as_str() {
        ".js" | ".mjs" | ".jsx" => Some(("javascript", tree_sitter_javascript::LANGUAGE.into())),
        ".ts" => Some(("typescript", tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())),
        ".tsx" => Some(("typescript", tree_sitter_typescript::LANGUAGE_TSX.into())),
        ".py" => Some(("python", tree_sitter_python::LANGUAGE.into())),
        ".rs" => Some(("rust", tree_sitter_rust::LANGUAGE.into())),
        ".go" => Some(("go", tree_sitter_go::LANGUAGE.into())),
        ".c" | ".h" => Some(("c", tree_sitter_c::LANGUAGE.into())),
        ".cpp" | ".cc" | ".hpp" | ".hh" | ".cxx" => Some(("cpp", tree_sitter_cpp::LANGUAGE.into())),
        ".glsl" | ".vert" | ".frag" | ".comp" | ".geom" | ".tesc" | ".tese" | ".vsh" | ".fsh" | ".glslv" | ".glslf" => {
            Some(("glsl", tree_sitter_c::LANGUAGE.into()))
        }
        ".java" => Some(("java", tree_sitter_java::LANGUAGE.into())),
        ".json" => Some(("json", tree_sitter_json::LANGUAGE.into())),
        ".html" | ".htm" => Some(("html", tree_sitter_html::LANGUAGE.into())),
        ".css" => Some(("css", tree_sitter_css::LANGUAGE.into())),
        ".sh" | ".bash" => Some(("bash", tree_sitter_bash::LANGUAGE.into())),
        ".md" | ".markdown" => Some(("markdown", tree_sitter_md::LANGUAGE.into())),
        ".ps1" | ".psm1" | ".psd1" => Some(("powershell", tree_sitter_powershell::LANGUAGE.into())),
        ".rb" => Some(("ruby", tree_sitter_ruby::LANGUAGE.into())),
        ".cs" => Some(("csharp", tree_sitter_c_sharp::LANGUAGE.into())),
        ".php" | ".phtml" => Some(("php", tree_sitter_php::LANGUAGE_PHP.into())),
        ".hs" | ".lhs" => Some(("haskell", tree_sitter_haskell::LANGUAGE.into())),
        ".jl" => Some(("julia", tree_sitter_julia::LANGUAGE.into())),
        _ => None,
    }
}

const CHUNK_NODE_TYPES: &[&str] = &[
    "function_declaration",
    "function_definition",
    "function_item",
    "method_declaration",
    "method_definition",
    "class_declaration",
    "class_definition",
    "impl_item",
    "struct_item",
    "enum_item",
    "trait_item",
    "arrow_function",
    "generator_function_declaration",
    "section",
];

struct Chunk {
    kind: String,
    name: String,
    line_start: usize,
    line_end: usize,
    body: String,
}

fn extract_chunks(source: &str, lang: Language) -> Vec<Chunk> {
    let mut parser = Parser::new();
    if parser.set_language(&lang).is_err() {
        return Vec::new();
    }
    let tree = match parser.parse(source, None) {
        Some(t) => t,
        None => return Vec::new(),
    };
    let mut out = Vec::new();
    let src_bytes = source.as_bytes();
    let mut stack: Vec<tree_sitter::Node> = vec![tree.root_node()];
    while let Some(node) = stack.pop() {
        let kind = node.kind();
        if CHUNK_NODE_TYPES.contains(&kind) {
            let start = node.start_byte();
            let end = node.end_byte().min(src_bytes.len());
            if end > start {
                let body = String::from_utf8_lossy(&src_bytes[start..end]).into_owned();
                let line_start = node.start_position().row + 1;
                let line_end = node.end_position().row + 1;
                let name = node
                    .child_by_field_name("name")
                    .map(|n| String::from_utf8_lossy(&src_bytes[n.start_byte()..n.end_byte().min(src_bytes.len())]).into_owned())
                    .unwrap_or_default();
                out.push(Chunk { kind: kind.to_string(), name, line_start, line_end, body });
                continue;
            }
        }
        for i in 0..node.child_count() as u32 {
            if let Some(c) = node.child(i) {
                stack.push(c);
            }
        }
    }
    out
}

/// Splits an oversized chunk's body into overlapping sub-chunks -- ported
/// verbatim from rs-plugkit's code_index.rs (same threshold/overlap
/// constants), since a caller (gm's own indexing plugin) needs this applied
/// BEFORE its own storage-side truncation, same ordering as the original.
const OVERSIZED_CHUNK_SPLIT_THRESHOLD: usize = 8192;
const OVERSIZED_CHUNK_OVERLAP: usize = 800;

fn split_oversized_chunk(c: &Chunk) -> Vec<Chunk> {
    if c.body.len() <= OVERSIZED_CHUNK_SPLIT_THRESHOLD {
        return vec![Chunk { kind: c.kind.clone(), name: c.name.clone(), line_start: c.line_start, line_end: c.line_end, body: c.body.clone() }];
    }
    let total_lines = c.line_end.saturating_sub(c.line_start).max(1);
    let bytes_per_line = (c.body.len() as f64 / total_lines as f64).max(1.0);
    let stride = OVERSIZED_CHUNK_SPLIT_THRESHOLD - OVERSIZED_CHUNK_OVERLAP;
    let mut out = Vec::new();
    let mut start = 0usize;
    let mut part = 0usize;
    let body = c.body.as_str();
    while start < body.len() {
        let mut end = (start + OVERSIZED_CHUNK_SPLIT_THRESHOLD).min(body.len());
        while end > start && !body.is_char_boundary(end) {
            end -= 1;
        }
        let sub_body = &body[start..end];
        let sub_line_start = c.line_start + ((start as f64 / bytes_per_line) as usize);
        let sub_line_end = c.line_start + ((end as f64 / bytes_per_line) as usize);
        let sub_name = if part == 0 { c.name.clone() } else { format!("{}#part{}", c.name, part + 1) };
        out.push(Chunk { kind: c.kind.clone(), name: sub_name, line_start: sub_line_start, line_end: sub_line_end.max(sub_line_start), body: sub_body.to_string() });
        if end >= body.len() {
            break;
        }
        let mut next_start = end.saturating_sub(OVERSIZED_CHUNK_OVERLAP);
        while next_start > 0 && !body.is_char_boundary(next_start) {
            next_start -= 1;
        }
        start = next_start.max(start + stride.min(1));
        part += 1;
    }
    out
}

pub fn handle_lang_for_ext(body: &serde_json::Value) -> u64 {
    let ext = body.get("ext").and_then(|v| v.as_str()).unwrap_or("");
    match lang_for_ext(ext) {
        Some((name, _)) => return_json(serde_json::json!({"ok": true, "lang": name})),
        None => return_json(serde_json::json!({"ok": true, "lang": null})),
    }
}

/// verb "parse": {"ext": ".rs", "source": "..."} -> {"ok": true, "lang": "rust", "chunks": [{kind,name,line_start,line_end,body}, ...]}
/// Oversized-chunk splitting is applied unconditionally -- the caller never
/// needs to know about the threshold, it always gets storage-ready chunks.
pub fn handle_parse(body: &serde_json::Value) -> u64 {
    let ext = body.get("ext").and_then(|v| v.as_str()).unwrap_or("");
    let source = body.get("source").and_then(|v| v.as_str()).unwrap_or("");
    let Some((lang_name, lang)) = lang_for_ext(ext) else {
        return return_json(serde_json::json!({"ok": true, "lang": null, "chunks": []}));
    };

    let mut chunks = extract_chunks(source, lang);
    if chunks.is_empty() && lang_name == "markdown" && !source.trim().is_empty() {
        let whole = source.chars().take(4000).collect::<String>();
        let line_end = source.lines().count().max(1);
        chunks.push(Chunk { kind: "document".to_string(), name: String::new(), line_start: 1, line_end, body: whole });
    }
    let needs_split = chunks.iter().any(|c| c.body.len() > OVERSIZED_CHUNK_SPLIT_THRESHOLD);
    if needs_split {
        chunks = chunks.iter().flat_map(split_oversized_chunk).collect();
    }

    let json_chunks: Vec<serde_json::Value> = chunks
        .into_iter()
        .map(|c| serde_json::json!({"kind": c.kind, "name": c.name, "line_start": c.line_start, "line_end": c.line_end, "body": c.body}))
        .collect();
    return_json(serde_json::json!({"ok": true, "lang": lang_name, "chunks": json_chunks}))
}
