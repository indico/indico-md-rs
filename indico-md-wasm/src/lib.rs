use indico_comrak::{LinkRule, MarkdownOptions};
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;

/// Converts markdown text to HTML while applying custom link rules
///
/// This function takes markdown text and an array of link rules from JavaScript,
/// processes them according to Indico's markdown rules, and returns the resulting HTML.
///
/// # Arguments
///
/// * `md_source` - A string slice containing the markdown text to process
/// * `js_rules` - A JavaScript array containing pairs of RegExp and URL pattern strings
///
/// # Returns
///
/// * `Result<String, JsValue>` - The processed HTML string on success, or a JsValue error on failure
///
/// # Errors
///
/// Returns a JsValue error if:
/// * The URL pattern is not a valid string
/// * The regular expression is not a valid string
/// * The link rule creation fails
///
/// # Example (JavaScript)
///
/// ```javascript
/// const autolinkRules = [
///   [/^#(\d+)$/, 'https://example.com/issues/$1'],
///   [/^@(\w+)$/, 'https://example.com/users/$1']
/// ];
/// const html = indicoMarkdown("See #123 and @user", {autolinkRules});
/// ```
#[wasm_bindgen(js_name = toHtml)]
pub fn to_html(md_source: &str, opts: &Object) -> Result<String, JsValue> {
    let mut md_opts = MarkdownOptions::new();

    if let Ok(js_rules) = Reflect::get(opts, &JsValue::from_str("autolinkRules"))
        && js_rules.is_array()
    {
        let mut rules = Vec::new();
        for res in Array::from(&js_rules).values() {
            let array: js_sys::Array = res?.into();
            let vec: Vec<_> = array.to_vec();
            let re: js_sys::RegExp = vec[0].clone().into();
            let url_pattern = vec[1]
                .as_string()
                .ok_or(JsValue::from_str("URL pattern is not a valid string"))?;

            rules.push(
                LinkRule::new(
                    &re.source().as_string().ok_or(JsValue::from_str(
                        "Regular expression is not a valid string",
                    ))?,
                    &url_pattern,
                )
                .map_err(|e| e.to_string())?,
            );
        }
        md_opts.autolink_rules(&rules);
    }

    if let Ok(js_unstyled) = Reflect::get(opts, &JsValue::from_str("unstyled"))
        && let Some(js_unstyled) = js_unstyled.as_bool()
    {
        md_opts.unstyled(js_unstyled);
    }
    if let Ok(js_nl2br) = Reflect::get(opts, &JsValue::from_str("nl2br"))
        && let Some(js_nl2br) = js_nl2br.as_bool()
    {
        md_opts.hardbreaks(js_nl2br);
    }
    if let Ok(js_target_blank) = Reflect::get(opts, &JsValue::from_str("target_blank"))
        && let Some(js_target_blank) = js_target_blank.as_bool()
    {
        md_opts.target_blank(js_target_blank);
    }

    md_opts
        .render_markdown(md_source)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
