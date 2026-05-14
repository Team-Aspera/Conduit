use std::collections::HashMap;
use std::sync::LazyLock;

use crate::types::Language;

fn load_translations(json: &str) -> HashMap<&'static str, &'static str> {
    let map: HashMap<String, String> =
        serde_json::from_str(json).expect("Failed to parse locale file");
    map.into_iter()
        .map(|(k, v)| {
            let k: &'static str = Box::leak(k.into_boxed_str());
            let v: &'static str = Box::leak(v.into_boxed_str());
            (k, v)
        })
        .collect()
}

static ZH: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| load_translations(include_str!("../locales/zh-CN.json")));

static EN: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| load_translations(include_str!("../locales/en.json")));

pub fn get(lang: Language, key: &str) -> &'static str {
    let map = match lang {
        Language::Chinese => &*ZH,
        Language::English => &*EN,
    };
    map.get(key).copied().unwrap_or("Unknown")
}
