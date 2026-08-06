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
        ".yaml" | ".yml" => Some(("yaml", tree_sitter_yaml::LANGUAGE.into())),
        ".toml" | ".ini" | ".cfg" | ".conf" => Some(("toml", tree_sitter_toml_ng::LANGUAGE.into())),
        ".sql" => Some(("sql", tree_sitter_sequel::LANGUAGE.into())),
        ".lua" => Some(("lua", tree_sitter_lua::LANGUAGE.into())),
        ".kt" | ".kts" => Some(("kotlin", tree_sitter_kotlin_ng::LANGUAGE.into())),
        ".swift" => Some(("swift", tree_sitter_swift::LANGUAGE.into())),
        ".zig" => Some(("zig", tree_sitter_zig::LANGUAGE.into())),
        ".ex" | ".exs" => Some(("elixir", tree_sitter_elixir::LANGUAGE.into())),
        ".scala" | ".sc" => Some(("scala", tree_sitter_scala::LANGUAGE.into())),
        ".pl" | ".pm" => Some(("perl", tree_sitter_perl::LANGUAGE.into())),
        ".r" => Some(("r", tree_sitter_r::LANGUAGE.into())),
        ".m" | ".mm" => Some(("objc", tree_sitter_objc::LANGUAGE.into())),
        ".xml" => Some(("xml", tree_sitter_xml::LANGUAGE_XML.into())),
        ".dockerfile" => Some(("dockerfile", tree_sitter_containerfile::LANGUAGE.into())),
        ".graphql" | ".gql" => Some(("graphql", tree_sitter_graphql::LANGUAGE.into())),
        ".proto" => Some(("proto", tree_sitter_proto::LANGUAGE.into())),
        _ => None,
    }
}

/// Resolves a caller-supplied language NAME (not a file extension) directly
/// to a grammar. Exists because plugkit-core's own lang_for_ext (the
/// gm-side caller) returns a distinct name per grammar variant -- notably
/// "tsx" for .tsx files, disambiguated from "typescript" for plain .ts,
/// even though this plugin's own ext table groups both under the
/// "typescript" display name. Accepting both naming conventions here (verb
/// body can carry "ext" OR "lang") means the two repos' ABIs don't have to
/// be edited in lockstep -- a caller migrated independently still resolves
/// correctly against whichever names it already sends.
fn lang_by_name(name: &str) -> Option<Language> {
    match name {
        "javascript" => Some(tree_sitter_javascript::LANGUAGE.into()),
        "typescript" => Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
        "tsx" => Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
        "python" => Some(tree_sitter_python::LANGUAGE.into()),
        "rust" => Some(tree_sitter_rust::LANGUAGE.into()),
        "go" => Some(tree_sitter_go::LANGUAGE.into()),
        "c" | "glsl" => Some(tree_sitter_c::LANGUAGE.into()),
        "cpp" => Some(tree_sitter_cpp::LANGUAGE.into()),
        "java" => Some(tree_sitter_java::LANGUAGE.into()),
        "json" => Some(tree_sitter_json::LANGUAGE.into()),
        "html" => Some(tree_sitter_html::LANGUAGE.into()),
        "css" => Some(tree_sitter_css::LANGUAGE.into()),
        "bash" => Some(tree_sitter_bash::LANGUAGE.into()),
        "markdown" => Some(tree_sitter_md::LANGUAGE.into()),
        "powershell" => Some(tree_sitter_powershell::LANGUAGE.into()),
        "ruby" => Some(tree_sitter_ruby::LANGUAGE.into()),
        "csharp" => Some(tree_sitter_c_sharp::LANGUAGE.into()),
        "php" => Some(tree_sitter_php::LANGUAGE_PHP.into()),
        "haskell" => Some(tree_sitter_haskell::LANGUAGE.into()),
        "julia" => Some(tree_sitter_julia::LANGUAGE.into()),
        "yaml" => Some(tree_sitter_yaml::LANGUAGE.into()),
        "toml" => Some(tree_sitter_toml_ng::LANGUAGE.into()),
        "sql" => Some(tree_sitter_sequel::LANGUAGE.into()),
        "lua" => Some(tree_sitter_lua::LANGUAGE.into()),
        "kotlin" => Some(tree_sitter_kotlin_ng::LANGUAGE.into()),
        "swift" => Some(tree_sitter_swift::LANGUAGE.into()),
        "zig" => Some(tree_sitter_zig::LANGUAGE.into()),
        "elixir" => Some(tree_sitter_elixir::LANGUAGE.into()),
        "scala" => Some(tree_sitter_scala::LANGUAGE.into()),
        "perl" => Some(tree_sitter_perl::LANGUAGE.into()),
        "r" => Some(tree_sitter_r::LANGUAGE.into()),
        "objc" => Some(tree_sitter_objc::LANGUAGE.into()),
        "xml" => Some(tree_sitter_xml::LANGUAGE_XML.into()),
        "dockerfile" => Some(tree_sitter_containerfile::LANGUAGE.into()),
        "graphql" => Some(tree_sitter_graphql::LANGUAGE.into()),
        "proto" => Some(tree_sitter_proto::LANGUAGE.into()),
        _ => None,
    }
}

// Default chunk-node-type filter, used ONLY by the legacy "extract_chunks"
// verb (kept for callers migrated before this plugin's response shape was
// corrected -- see handle_parse's doc comment). The "parse" verb itself is
// policy-free: it returns every node's position/kind/name, no filtering,
// since baking a "code chunk" concept into a generic tree-sitter plugin
// contradicts the point of this being a reusable, gm-agnostic plugin -- a
// future non-gm consumer of agentplug may want every node, or a different
// filter entirely, and that decision belongs to the caller, not this plugin.
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

struct Node {
    kind: String,
    name: String,
    start_byte: usize,
    end_byte: usize,
    start_row: usize,
    end_row: usize,
}

struct Chunk {
    kind: String,
    name: String,
    line_start: usize,
    line_end: usize,
    body: String,
}

/// Policy-free walk: every node in the tree, no filtering. This is what the
/// "parse" verb returns -- callers (e.g. plugkit-core's code_index.rs)
/// apply their own CHUNK_NODE_TYPES-equivalent filter over the result.
fn walk_all_nodes(source: &str, lang: Language) -> Vec<Node> {
    let mut parser = Parser::new();
    if parser.set_language(&lang).is_err() {
        return Vec::new();
    }
    let tree = match parser.parse(source, None) {
        Some(t) => t,
        None => return Vec::new(),
    };
    let src_bytes = source.as_bytes();
    let mut out = Vec::new();
    let mut stack: Vec<tree_sitter::Node> = vec![tree.root_node()];
    while let Some(node) = stack.pop() {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte().min(src_bytes.len());
        let name = node
            .child_by_field_name("name")
            .map(|n| String::from_utf8_lossy(&src_bytes[n.start_byte()..n.end_byte().min(src_bytes.len())]).into_owned())
            .unwrap_or_default();
        out.push(Node {
            kind: node.kind().to_string(),
            name,
            start_byte,
            end_byte,
            start_row: node.start_position().row,
            end_row: node.end_position().row,
        });
        for i in 0..node.child_count() as u32 {
            if let Some(c) = node.child(i) {
                stack.push(c);
            }
        }
    }
    out
}

/// Legacy chunk-extraction walk (CHUNK_NODE_TYPES-filtered, early-exit on
/// match rather than descending into a matched node's children) -- kept
/// behind the separate "extract_chunks" verb only, for backward
/// compatibility with any caller still expecting this plugin's original
/// (policy-baked) response shape.
fn extract_chunks_with(source: &str, lang: Language, extra_node_types: &[String]) -> Vec<Chunk> {
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
        if CHUNK_NODE_TYPES.contains(&kind) || extra_node_types.iter().any(|t| t == kind) {
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

fn resolve_lang<'a>(ext: &str, lang_field: &'a str) -> Option<(&'a str, Language)>
where
    'static: 'a,
{
    if !lang_field.is_empty() {
        if let Some(l) = lang_by_name(lang_field) {
            return Some((lang_field, l));
        }
    }
    lang_for_ext(ext).map(|(name, l)| (name, l))
}

/// verb "parse": accepts EITHER {"ext": ".rs", "source": "..."} (file
/// extension, resolved via lang_for_ext) OR {"lang": "rust", "source": "..."}
/// (an explicit language name, resolved via lang_by_name) -- "lang" is tried
/// first when present, since an explicit language name may disambiguate
/// variants ext alone can't (e.g. "tsx" vs "typescript" for .tsx vs .ts,
/// both grouped under lang_for_ext's single "typescript" ext-table entry).
/// Returns {"ok":true,"lang":"rust","nodes":[{kind,name,start_byte,end_byte,
/// start_row,end_row}, ...]} -- POLICY-FREE, every node in the tree, no
/// chunk-type filtering, no oversized-body splitting. This plugin doesn't
/// know what a "code chunk" is; that's the caller's concept. Returns
/// {"ok":true,"lang":null,"nodes":[]} when neither ext nor lang resolves --
/// never an error envelope, since "no grammar for this input" is expected,
/// not exceptional.
pub fn handle_parse(body: &serde_json::Value) -> u64 {
    let ext = body.get("ext").and_then(|v| v.as_str()).unwrap_or("");
    let lang_field = body.get("lang").and_then(|v| v.as_str()).unwrap_or("");
    let source = body.get("source").and_then(|v| v.as_str()).unwrap_or("");

    let Some((lang_name, lang)) = resolve_lang(ext, lang_field) else {
        return return_json(serde_json::json!({"ok": true, "lang": null, "nodes": []}));
    };

    let nodes = walk_all_nodes(source, lang);
    let json_nodes: Vec<serde_json::Value> = nodes
        .into_iter()
        .map(|n| {
            serde_json::json!({
                "kind": n.kind, "name": n.name,
                "start_byte": n.start_byte, "end_byte": n.end_byte,
                "start_row": n.start_row, "end_row": n.end_row,
            })
        })
        .collect();
    return_json(serde_json::json!({"ok": true, "lang": lang_name, "nodes": json_nodes}))
}

/// verb "extract_chunks": legacy policy-baked shape (CHUNK_NODE_TYPES
/// filter + oversized-body splitting applied server-side), kept for any
/// caller still expecting this plugin's original response shape rather
/// than "parse"'s policy-free node list. New callers should prefer "parse"
/// and apply their own filtering, matching how plugkit-core's own
/// code_index.rs was actually written.
pub fn handle_extract_chunks(body: &serde_json::Value) -> u64 {
    let ext = body.get("ext").and_then(|v| v.as_str()).unwrap_or("");
    let lang_field = body.get("lang").and_then(|v| v.as_str()).unwrap_or("");
    let source = body.get("source").and_then(|v| v.as_str()).unwrap_or("");

    let Some((lang_name, lang)) = resolve_lang(ext, lang_field) else {
        return return_json(serde_json::json!({"ok": true, "lang": null, "chunks": []}));
    };

    // Additive, never replacing: a caller naming node types for a grammar this
    // build does not special-case gets them chunked alongside the builtins,
    // rather than having to fork the list. Replacing the builtins instead
    // would silently stop chunking every language the caller did not enumerate.
    let extra_node_types: Vec<String> = body
        .get("extra_node_types")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
        .unwrap_or_default();

    let mut chunks = extract_chunks_with(source, lang, &extra_node_types);
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
