use crate::{DiffableArguments, Entry};
use js_sys::JsString;
use sledgehammer_bindgen::bindgen;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(inline_js = "export function takes_string(s) {}export function takes_string2(s) {}")]
extern "C" {
    #[wasm_bindgen]
    fn takes_string(s: JsString);

    #[wasm_bindgen]
    fn takes_string2(s: String);
}

#[bindgen]
mod js {
    const JS: &str = r#"
        let current_text = "";
        const cache = [];
        let temp;

        export function take() {
            temp = current_text;
            current_text = "";
            return temp;
        }
    "#;

    extern "C" {
        #[wasm_bindgen]
        fn take() -> JsString;
    }

    fn add_bool(n: u8) {
        "current_text+=n===0;"
    }

    fn add_num(n: u32) {
        "current_text+=n;"
    }

    fn add_str(str: &str<u16>) {
        "current_text+=str;"
    }

    fn add_static_str(id: u32) {
        "current_text+=cache[id];"
    }

    fn static_str(id: u32, str: &'static str<u16>) {
        "cache[id]=str;"
    }

    fn takes_string(s3: &str) {
        "let s2 = s3;"
    }
}

fn set_static_str(channel: &mut Channel, id: u32, str: &'static str) {
    static mut MAX_ID: u32 = u32::MAX;

    unsafe {
        // if id > MAX_ID || MAX_ID == u32::MAX {
        // MAX_ID = id;
        channel.static_str(id, str);
        // }
    }
}

fn format_diffable_args(channel: &mut Channel, args: DiffableArguments) -> JsString {
    for (i, (static_seg, dynamic_seg)) in args
        .static_segments
        .iter()
        .zip(args.dynamic_segments.iter())
        .enumerate()
    {
        let i = i as u32;
        if !static_seg.is_empty() {
            set_static_str(channel, i, static_seg);
            channel.add_static_str(i);
        }
        match dynamic_seg {
            Entry::U32(n) => channel.add_num(*n),
            Entry::I32(i) => channel.add_num(*i as u32),
            Entry::Bool(n) => channel.add_bool(*n as u8),
            Entry::Str(s) => channel.add_str(s),
        }
    }
    let static_seg = unsafe {
        args.static_segments
            .get_unchecked(args.static_segments.len() - 1)
    };
    let last = args.static_segments.len() as u32;
    if !static_seg.is_empty() {
        set_static_str(channel, last, static_seg);
        channel.add_static_str(last);
    }
    channel.flush();
    take()
}

impl From<DiffableArguments<'_>> for JsString {
    fn from(val: DiffableArguments<'_>) -> Self {
        let mut channel = Channel::default();
        format_diffable_args(&mut channel, val)
    }
}
