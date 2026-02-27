use indico_comrak::{LinkRule, MarkdownOptions};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize)]
#[serde(from = "(String, String)")]
struct JsAutolinkRule {
    re: String,
    url: String,
}

impl From<(String, String)> for JsAutolinkRule {
    fn from((re, url): (String, String)) -> Self {
        Self { re, url }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JsMarkdownOpts {
    unstyled: Option<bool>,
    nl2br: Option<bool>,
    target_blank: Option<bool>,
    autolink_rules: Option<Vec<JsAutolinkRule>>,
}

/// Converts markdown text to HTML while applying custom link rules
///
/// This function takes markdown text and an array of link rules from JavaScript,
/// processes them according to Indico's markdown rules, and returns the resulting HTML.
///
/// # Arguments
///
/// * `md_source` - A string slice containing the markdown text to process
/// * `opts` - A JavaScript object containing the options
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
///   ['^#(\d+)$', 'https://example.com/issues/$1'],
///   ['^@(\w+)$', 'https://example.com/users/$1']
/// ];
/// const html = indicoMarkdown("See #123 and @user", {autolinkRules});
/// ```
#[wasm_bindgen(js_name = toHtml)]
pub fn to_html(md_source: &str, opts: Option<JsValue>) -> Result<String, JsValue> {
    let mut md_opts = MarkdownOptions::new();

    let opts: JsMarkdownOpts = if let Some(opts) = opts {
        serde_wasm_bindgen::from_value(opts).map_err(|e| JsValue::from_str(&e.to_string()))?
    } else {
        JsMarkdownOpts::default()
    };

    if let Some(cfg_rules) = opts.autolink_rules
        && !cfg_rules.is_empty()
    {
        let rules: Vec<LinkRule> = cfg_rules
            .iter()
            .map(|r| LinkRule::new(&r.re, &r.url))
            .collect::<Result<_, _>>()
            .map_err(|e| e.to_string())?;
        md_opts.autolink_rules(&rules);
    }

    if let Some(unstyled) = opts.unstyled {
        md_opts.unstyled(unstyled);
    }
    if let Some(hardbreaks) = opts.nl2br {
        md_opts.hardbreaks(hardbreaks);
    }
    if let Some(target_blank) = opts.target_blank {
        md_opts.target_blank(target_blank);
    }

    md_opts
        .render_markdown(md_source)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
