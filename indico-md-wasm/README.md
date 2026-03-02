# indico-md-wasm 📦

A tiny WebAssembly build of the indico markdown renderer for use in browsers and Node.js.

## Features
- Fast, Rust-based markdown rendering compiled to WASM;
- Minimal JavaScript glue via wasm-bindgen;
- Works in browser (ESM) and Node (CommonJS / ESM);
- Zero-dependency runtime for embedding in web apps.

## Quickstart

### Prerequisites
- Rust toolchain (rustup)
- wasm-pack (or cargo + wasm-bindgen)
- Node.js (for Node usage) or a static server for browser

### Build locally
```bash
# build a package suitable for bundlers (webpack/rollup)
wasm-pack build --release --target bundler

# OR for direct browser usage
wasm-pack build --release --target web
```

Browser usage (ESM)
```html
<script type="module">
  import init, { toHtml } from "./pkg/indico_md_wasm.js";

  await init(); // loads and initializes the .wasm

  // pass an array of [RegExp, string] pairs for custom link rules,
  // or an empty array if you don't need custom rules.
  const html = toHtml("# Hello\nThis is indico-md-wasm.", [], false);
  document.body.innerHTML = html;
</script>
```

Node usage (ESM)
```js
import init, { toHtml } from "./pkg/indico_md_wasm.js";
await init();
console.log(toHtml("**bold** text", [], false));
```

Link rules example
```js
const autolinkRules = [
  {regex: '^#(\\d+)$', url: 'https://example.com/issues/$1'},
  {regex: '^@(\\w+)$', url: 'https://example.com/users/$'},
];

const html = toHtml("See #123 and @user", {autolinkRules});
```

API (exports)
- (default) `init(): Promise<void>` — initializes the WASM module
- `toHtml(source: string, opts?: Object): string` — converts Indico-flavored markdown to HTML; `opts` is a JS object with these (optional) keys:
  - `unstyled` - a bool whether to generate unstyled output (defaults to false)
  - `nl2br` - a bool whether to convert `\n` to `<br>` like (defaults to false)
  - `targetBlank` - a bool whether links should open in a new window (defaults to true)
  - `autolinkRules` - an array of `{regex, url}` objects

### Tests
```bash
wasm-pack test --node
```
