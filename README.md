# ⚡ RustX

**Reactive HTML powered by Rust & WebAssembly**

RustX brings the simplicity of htmx with the performance of Rust. Add dynamic behavior to your HTML with declarative attributes - no build step, no npm install, just one script tag.

## Quick Start

Include this one line in your HTML:

```html
<script src="https://cdn.rustx.io/rustx.js"></script>
```

That's it! Now you can make any element reactive:

```html
<button rx-action="/increment" rx-method="POST" rx-target="#count" rx-swap="innerHTML">
    +
</button>
<span id="count">0</span>
```

## Why RustX?

- 🚀 **Fast** - Powered by WebAssembly for near-native performance
- 📦 **Tiny** - Only ~87 KB total (comparable to htmx)
- 🎯 **Simple** - Just HTML attributes, no JavaScript needed
- 🔒 **Safe** - Written in Rust with memory safety guarantees
- 💻 **Universal** - Works with any backend (Node, Python, Ruby, Go, PHP, etc.)

## The Classic Counter

```html
<!DOCTYPE html>
<html>
<body>
    <button rx-action="/decrement" rx-method="POST" rx-target="#count" rx-swap="innerHTML">-</button>
    <span id="count">0</span>
    <button rx-action="/increment" rx-method="POST" rx-target="#count" rx-swap="innerHTML">+</button>

    <script src="https://cdn.rustx.io/rustx.js"></script>
</body>
</html>
```

Your server just returns the new count value. That's it!

## Core Attributes

| Attribute | Description | Example |
|-----------|-------------|---------|
| `rx-action` | URL endpoint to request | `rx-action="/api/data"` |
| `rx-method` | HTTP method | `rx-method="POST"` |
| `rx-target` | Element to update (CSS selector) | `rx-target="#result"` |
| `rx-swap` | How to insert response | `rx-swap="innerHTML"` |
| `rx-trigger` | Event that triggers request | `rx-trigger="change"` |
| `rx-push-url` | Push URL to browser history | `rx-push-url="true"` or `rx-push-url="/page"` |
| `rx-replace-url` | Replace current URL in history | `rx-replace-url="true"` or `rx-replace-url="/page"` |
| `rx-push-title` | Set custom page title | `rx-push-title="My Page"` |

## Swap Modes

Control how content is inserted:

- `innerHTML` - Replace inner content (default)
- `outerHTML` - Replace entire element
- `beforebegin` - Insert before element
- `afterbegin` - Insert as first child
- `beforeend` - Insert as last child
- `afterend` - Insert after element
- `none` - Don't swap (side effects only)

## Examples

### Form Submission

```html
<form rx-action="/submit" rx-target="#result">
    <input type="text" name="email" required>
    <button type="submit">Submit</button>
</form>
<div id="result"></div>
```

### Append to List

```html
<button rx-action="/item" rx-target="#list" rx-swap="beforeend">
    Add Item
</button>
<ul id="list"></ul>
```

### On Change

```html
<select rx-action="/filter" rx-trigger="change" rx-target="#results">
    <option>Option 1</option>
    <option>Option 2</option>
</select>
<div id="results"></div>
```

## Trigger Modifiers

RustX supports powerful trigger modifiers to control when requests are fired:

### The `once` Modifier

Fire a request only once:

```html
<button rx-action="/api/accept" rx-trigger="click once">
    Accept Terms (works only once)
</button>
```

Use cases: One-time confirmations, dismissing notifications, tutorial steps.

### The `changed` Modifier

Only trigger if the value has actually changed:

```html
<input
    rx-action="/api/save"
    rx-trigger="blur changed"
    name="email">
```

Use cases: Auto-save forms, preventing unnecessary server requests.

### The `delay` Modifier

Delay the request by a specified time:

```html
<!-- Delay in milliseconds -->
<input
    rx-action="/api/search"
    rx-trigger="keyup delay:500ms"
    name="query">

<!-- Delay in seconds -->
<div rx-action="/api/load" rx-trigger="mouseenter delay:1s">
    Hover to load (after 1 second)
</div>
```

Use cases: Debouncing user input, delayed tooltips, hover previews.

## Trigger Filters

Use square bracket syntax to filter triggers with JavaScript expressions:

### Control/Command Key Filter

```html
<button rx-action="/api/special" rx-trigger="click[ctrlKey]">
    Ctrl+Click for special action
</button>
```

### Shift Key Filter

```html
<button rx-action="/api/multi-select" rx-trigger="click[shiftKey]">
    Shift+Click to multi-select
</button>
```

### Custom Filter Expressions

Any JavaScript expression that evaluates to true/false:

```html
<!-- Only trigger on Enter key -->
<input rx-trigger="keydown[key=='Enter']" rx-action="/submit">

<!-- Only on left mouse button -->
<div rx-trigger="click[button==0]" rx-action="/left-click">

<!-- Complex expressions -->
<div rx-trigger="click[shiftKey && ctrlKey]" rx-action="/special">
```

The filter expression can access:
- Event properties: `ctrlKey`, `shiftKey`, `altKey`, `metaKey`, `key`, `button`, etc.
- Element properties via `this`
- Global scope

### Combining Modifiers and Filters

```html
<!-- Ctrl+Click with delay and only once -->
<button
    rx-action="/api/power-action"
    rx-trigger="click[ctrlKey] delay:500ms once">
    Power User Action
</button>

<!-- Value change with delay -->
<input
    rx-action="/api/search"
    rx-trigger="input changed delay:300ms"
    name="search">
```

## History Support

RustX provides browser history integration for Single-Page Application (SPA) experiences:

```html
<!-- Push new URL to history (creates back button entry) -->
<button
    rx-action="/page1"
    rx-push-url="true"
    rx-target="#content"
    rx-push-title="Page 1">
    Go to Page 1
</button>

<!-- Use custom URL instead of endpoint URL -->
<button
    rx-action="/api/page2"
    rx-push-url="/page-2"
    rx-target="#content">
    Go to Page 2
</button>

<!-- Replace URL without creating history entry -->
<button
    rx-action="/filter"
    rx-replace-url="true"
    rx-target="#results">
    Apply Filter
</button>
```

### How it Works

1. **Before navigation**: RustX snapshots the current page (HTML, scroll position, title)
2. **After swap**: The URL is pushed/replaced and the snapshot is saved to browser history
3. **On back/forward**: The snapshot is restored instantly (no server request needed)

### History Attributes

- **`rx-push-url`** - Adds a new entry to browser history
  - `"true"` - Uses the response URL or request URL
  - `"/custom-url"` - Uses the specified URL

- **`rx-replace-url`** - Updates current history entry (no new back button entry)
  - `"true"` - Uses the response URL or request URL
  - `"/custom-url"` - Uses the specified URL
  - Great for filters, tabs, or modal states that shouldn't clutter history

- **`rx-push-title`** - Sets a custom page title for the history entry
  - Updates both `document.title` and the browser history title

### History Events

```javascript
document.addEventListener('rx:history-pushed', (e) => {
    console.log('New history entry created');
});

document.addEventListener('rx:history-replaced', (e) => {
    console.log('History entry replaced');
});

document.addEventListener('rx:restored', (e) => {
    console.log('Page restored from history snapshot');
});
```

## Event Lifecycle

RustX fires custom events you can listen to:

```javascript
element.addEventListener('rx:before', (e) => {
    // Before request is sent
    e.target.classList.add('loading');
});

element.addEventListener('rx:after', (e) => {
    // After response received
    e.target.classList.remove('loading');
});
```

Events: `rx:init`, `rx:inited`, `rx:config`, `rx:before`, `rx:after`, `rx:swapped`, `rx:finally`, `rx:history-pushed`, `rx:history-replaced`, `rx:restored`

## Server Requirements

RustX works with any server that can return HTML:

**Python (Flask):**
```python
@app.route('/increment', methods=['POST'])
def increment():
    count = session.get('count', 0) + 1
    session['count'] = count
    return str(count)
```

**Node.js (Express):**
```javascript
app.post('/increment', (req, res) => {
    req.session.count = (req.session.count || 0) + 1;
    res.send(String(req.session.count));
});
```

**Go:**
```go
http.HandleFunc("/increment", func(w http.ResponseWriter, r *http.Request) {
    count := getCount() + 1
    setCount(count)
    fmt.Fprintf(w, "%d", count)
})
```

## Installation

### CDN (Recommended)

```html
<script src="https://cdn.rustx.io/rustx.js"></script>
```

### Self-Hosted

1. Build the distribution:
   ```bash
   ./build-cdn.sh
   ```

2. Copy `dist-cdn/*` to your web server

3. Include in your HTML:
   ```html
   <script src="/path/to/rustx.js"></script>
   ```

### NPM (Coming Soon)

```bash
npm install rustx-wasm
```

## Building from Source

Requirements:
- Rust 1.70+
- wasm-pack

```bash
# Clone the repo
git clone https://github.com/yourusername/rustx
cd rustx

# Build for CDN
./build-cdn.sh

# Or build for development
wasm-pack build --target web

# Run examples
cd examples/basic
python3 server.py
```

## Browser Support

- Chrome/Edge 57+
- Firefox 52+
- Safari 11+
- All modern browsers with WebAssembly support

## Comparison

|  | RustX | htmx | Alpine.js | React |
|---|-------|------|-----------|-------|
| Size | ~87 KB | ~47 KB | ~44 KB | ~140 KB (min) |
| Language | Rust/WASM | JavaScript | JavaScript | JavaScript |
| Build Step | No | No | No | Yes |
| Learning Curve | Minimal | Minimal | Low | High |
| Performance | Native-like | Good | Good | Good |

## Documentation

- [Examples](./examples/) - Working examples with local server
- [Trigger Modifiers Demo](./examples/basic/trigger-demo.html) - Interactive trigger modifiers & filters demo
- [History Demo](./examples/basic/history-demo.html) - Interactive history support demo
- [CDN Deployment](./CDN-DEPLOYMENT.md) - How to deploy to a CDN
- [Quick Reference](./examples/basic/QUICKREF.md) - Cheat sheet
- [API Documentation](./docs/) (Coming soon)

## Philosophy

RustX follows these principles:

1. **Simplicity First** - HTML attributes over JavaScript code
2. **Progressive Enhancement** - Works without JS, better with it
3. **Server-Side Rendering** - Let the server do the work
4. **Zero Build** - No webpack, no babel, no npm install
5. **Performance** - WASM-native speed where it matters

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](./docs/CONTRIBUTING.md) first.

## License

MIT License - see [LICENSE](./LICENSE)

## Acknowledgments

- Inspired by [htmx](https://htmx.org/) - the original HTML-first approach
- Built with [wasm-pack](https://rustwasm.github.io/wasm-pack/) - Rust to WASM tooling
- Powered by [web-sys](https://rustwasm.github.io/wasm-bindgen/web-sys/) - Rust Web APIs

## Status

⚠️ **Alpha** - RustX is in early development. APIs may change. Not recommended for production yet.

## Questions?

- 📖 [Documentation](./examples/README.md)
- 💬 [Discussions](https://github.com/yourusername/rustx/discussions)
- 🐛 [Issues](https://github.com/yourusername/rustx/issues)
- 🐦 [Twitter](https://twitter.com/rustx_io)

---

Made with 🦀 by the RustX team
