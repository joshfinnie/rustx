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

Events: `rx:init`, `rx:inited`, `rx:config`, `rx:before`, `rx:after`, `rx:swapped`, `rx:finally`

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

Contributions welcome! Please read [CONTRIBUTING.md](./CONTRIBUTING.md) first.

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
