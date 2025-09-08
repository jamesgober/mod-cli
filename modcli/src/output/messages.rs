use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

static CATALOG: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
static INTERCEPTOR: OnceLock<
    Mutex<Option<Box<dyn Fn(&str, &str) -> Cow<'static, str> + Send + Sync>>>,
> = OnceLock::new();

fn catalog() -> &'static Mutex<HashMap<String, String>> {
    CATALOG.get_or_init(|| Mutex::new(HashMap::new()))
}

fn interceptor(
) -> &'static Mutex<Option<Box<dyn Fn(&str, &str) -> Cow<'static, str> + Send + Sync>>> {
    INTERCEPTOR.get_or_init(|| Mutex::new(None))
}

/// Set or override a message value for a given key.
pub fn set_message<K: Into<String>, V: Into<String>>(key: K, value: V) {
    if let Ok(mut map) = catalog().lock() {
        map.insert(key.into(), value.into());
    }
}

/// Remove a customized message for a key (falls back to default when used).
pub fn reset_message(key: &str) {
    if let Ok(mut map) = catalog().lock() {
        map.remove(key);
    }
}

/// Get a customized message for a key, if present.
pub fn get_message(key: &str) -> Option<String> {
    catalog().lock().ok().and_then(|m| m.get(key).cloned())
}

/// Return a customized message if present, otherwise the provided default.
pub fn message_or_default<'a>(key: &str, default: &'a str) -> Cow<'a, str> {
    if let Some(val) = get_message(key) {
        Cow::Owned(val)
    } else {
        Cow::Borrowed(default)
    }
}

/// Set a global output interceptor. The interceptor receives a category and text and
/// returns the (possibly transformed) text to print.
pub fn set_output_interceptor<F>(f: F)
where
    F: Fn(&str, &str) -> Cow<'static, str> + Send + Sync + 'static,
{
    if let Ok(mut slot) = interceptor().lock() {
        *slot = Some(Box::new(f));
    }
}

/// Clear the output interceptor.
pub fn clear_output_interceptor() {
    if let Ok(mut slot) = interceptor().lock() {
        *slot = None;
    }
}

/// Apply the interceptor to a given category/text if one is set.
pub fn intercept<'a>(category: &str, text: &'a str) -> Cow<'a, str> {
    if let Ok(slot) = interceptor().lock() {
        if let Some(ref cb) = *slot {
            // Promote to 'static by cloning into owned when changed
            let owned: Cow<'static, str> = cb(category, text);
            return Cow::Owned(owned.into_owned());
        }
    }
    Cow::Borrowed(text)
}

/// Load messages from a JSON object file mapping string keys to string values.
/// Example JSON: { "help.header": "Commands", "unknown": "..." }
#[cfg(feature = "theme-config")]
pub fn load_messages_from_json(path: &str) -> Result<(), String> {
    let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let map: HashMap<String, String> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    if let Ok(mut cat) = catalog().lock() {
        for (k, v) in map {
            cat.insert(k, v);
        }
    }
    Ok(())
}

#[cfg(not(feature = "theme-config"))]
pub fn load_messages_from_json(_path: &str) -> Result<(), String> {
    Err("messages JSON loader requires feature: theme-config".into())
}
