use std::alloc::{alloc, dealloc, Layout};
use std::mem;

#[no_mangle]
pub extern "C" fn plugkit_alloc(len: u32) -> u32 {
    if len == 0 {
        return 0;
    }
    let layout = Layout::from_size_align(len as usize, mem::align_of::<u8>()).unwrap();
    unsafe { alloc(layout) as u32 }
}

#[no_mangle]
pub extern "C" fn plugkit_free(ptr: u32, len: u32) {
    if ptr == 0 || len == 0 {
        return;
    }
    let layout = Layout::from_size_align(len as usize, mem::align_of::<u8>()).unwrap();
    unsafe { dealloc(ptr as *mut u8, layout) };
}

pub fn read_str(ptr: u32, len: u32) -> String {
    if len == 0 {
        return String::new();
    }
    unsafe {
        let slice = std::slice::from_raw_parts(ptr as *const u8, len as usize);
        String::from_utf8_lossy(slice).into_owned()
    }
}

pub fn return_bytes(bytes: Vec<u8>) -> u64 {
    if bytes.is_empty() {
        return 0;
    }
    let len = bytes.len();
    let ptr = plugkit_alloc(len as u32);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr as *mut u8, len);
    }
    (ptr as u64 & 0xffff_ffff) | ((len as u64) << 32)
}

pub fn return_json(v: serde_json::Value) -> u64 {
    return_bytes(v.to_string().into_bytes())
}

#[no_mangle]
pub extern "C" fn plugin_call(verb_ptr: u32, verb_len: u32, body_ptr: u32, body_len: u32) -> u64 {
    let verb = read_str(verb_ptr, verb_len);
    let body_str = read_str(body_ptr, body_len);
    let body: serde_json::Value = serde_json::from_str(&body_str).unwrap_or(serde_json::json!({}));

    match verb.as_str() {
        "parse" => crate::parse::handle_parse(&body),
        "extract_chunks" => crate::parse::handle_extract_chunks(&body),
        "lang_for_ext" => crate::parse::handle_lang_for_ext(&body),
        // Answers "what can you do" without side effects, so a caller can
        // probe before dispatching instead of discovering a missing verb as
        // an indistinguishable ok:false at the call site.
        "capabilities" => return_json(serde_json::json!({
            "ok": true,
            "plugin": "treesitter",
            "verbs": ["parse", "extract_chunks", "lang_for_ext", "capabilities"],
            "payload_field": {
                "parse": "nodes", "extract_chunks": "chunks", "lang_for_ext": "lang",
            },
            "unresolved_lang_is_ok_true_with_empty_payload": true,
        })),
        _ => return_json(serde_json::json!({"ok": false, "error": "unknown_verb", "verb": verb})),
    }
}
