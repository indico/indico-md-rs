//! Test suite for the Web and headless browsers.

// #![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use indico_md_wasm::{JsAutolinkRule, JsMarkdownOpts, to_html};
use js_sys::{Array, Object};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn function_test() {
    let md = r#"## TEST
 * TKT1234567: solved
 * Still checking gh:123
 * [gh:124](https://somewhere.else) shouldn't be autolinked
"#;
    let opts = JsMarkdownOpts {
        autolink_rules: Some(vec![
            JsAutolinkRule {
                regex: r"\bTKT(\d{7})\b".into(),
                url: "https://tkt.sys/{1}".into(),
            },
            JsAutolinkRule {
                regex: r"\bgh:(\d+)\b".into(),
                url: "https://github.com/indico/indico/issues/{1}".into(),
            },
        ]),
        ..JsMarkdownOpts::default()
    };
    let res = to_html(md, Some(serde_wasm_bindgen::to_value(&opts).unwrap())).unwrap();

    assert_eq!(
        res,
        r##"<h2><a href="#test" aria-hidden="true" class="anchor" id="indico-md-test"></a>TEST</h2>
<ul>
<li><a href="https://tkt.sys/1234567" title="TKT1234567" target="_blank">TKT1234567</a>: solved</li>
<li>Still checking <a href="https://github.com/indico/indico/issues/123" title="gh:123" target="_blank">gh:123</a></li>
<li><a href="https://somewhere.else" target="_blank">gh:124</a> shouldn't be autolinked</li>
</ul>
"##
    );

    let md = "## title\n[`link`](https://example.com)\\\n`more` **text**";
    let opts = JsMarkdownOpts {
        unstyled: Some(true),
        ..opts
    };
    let res = to_html(md, Some(serde_wasm_bindgen::to_value(&opts).unwrap())).unwrap();
    assert_eq!(res, "title\n<p>link<br />\nmore text</p>\n")
}

#[wasm_bindgen_test]
fn nl2br_test() {
    assert_eq!(
        to_html("hello\nworld", None),
        Ok("<p>hello\nworld</p>\n".into())
    );
    assert_eq!(
        to_html(
            "hello\nworld",
            Some(
                serde_wasm_bindgen::to_value(&JsMarkdownOpts {
                    unstyled: Some(true),
                    ..JsMarkdownOpts::default()
                })
                .unwrap()
            )
        ),
        Ok("<p>hello\nworld</p>\n".into())
    );
    assert_eq!(
        to_html(
            "hello\nworld",
            Some(
                serde_wasm_bindgen::to_value(&JsMarkdownOpts {
                    nl2br: Some(true),
                    ..JsMarkdownOpts::default()
                })
                .unwrap()
            )
        ),
        Ok("<p>hello<br />\nworld</p>\n".into())
    );
    assert_eq!(
        to_html(
            "hello\nworld",
            Some(
                serde_wasm_bindgen::to_value(&JsMarkdownOpts {
                    unstyled: Some(true),
                    nl2br: Some(true),
                    ..JsMarkdownOpts::default()
                })
                .unwrap()
            )
        ),
        Ok("<p>hello<br />\nworld</p>\n".into())
    );
}

#[wasm_bindgen_test]
fn interface_test() {
    assert_eq!(to_html("", None), Ok("".into()));
    assert_eq!(
        to_html("", Some(JsValue::from(Object::new()))),
        Ok("".into())
    );

    // missing keys
    let opts = Object::from_entries(&Array::of1(&Array::of2(
        &"autolinkRules".into(),
        &Array::of1(
            &Object::from_entries(&Array::of1(&Array::of2(&"regex".into(), &r"foo".into())))
                .unwrap(),
        ),
    )))
    .unwrap();
    let res = to_html("foo", Some(JsValue::from(opts)));
    assert!(res.is_err());
    assert!(
        res.err()
            .unwrap()
            .as_string()
            .expect("Error is not a string")
            .contains("missing field `url`")
    );

    // invalid type
    let opts = Object::from_entries(&Array::of1(&Array::of2(
        &"autolinkRules".into(),
        &Array::of1(
            &Object::from_entries(&Array::of1(&Array::of2(
                &"regex".into(),
                &JsValue::from_bool(true),
            )))
            .unwrap(),
        ),
    )))
    .unwrap();
    let res = to_html("foo", Some(JsValue::from(opts)));
    assert!(res.is_err());
    assert!(
        res.err()
            .unwrap()
            .as_string()
            .expect("Error is not a string")
            .contains("invalid type: boolean `true`, expected a string")
    );

    // invalid type for config value
    let opts = Object::from_entries(&Array::of1(&Array::of2(
        &"nl2br".into(),
        &JsValue::from_f64(69.0),
    )))
    .unwrap();
    let res = to_html("foo", Some(JsValue::from(opts)));
    assert!(res.is_err());
    assert!(
        res.err()
            .unwrap()
            .as_string()
            .expect("Error is not a string")
            .contains("invalid type: floating point `69.0`, expected a boolean")
    );

    // invalid type for config object
    let res = to_html("foo", Some(JsValue::from_f64(69.0)));
    assert!(res.is_err());
    assert!(
        res.err()
            .unwrap()
            .as_string()
            .expect("Error is not a string")
            .contains("invalid type: floating point `69.0`, expected struct JsMarkdownOpts")
    );
}
