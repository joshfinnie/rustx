# 🚀 Quick Start Guide

Welcome to RustX! You now have a CDN-ready, production-ready reactive HTML library.

## For End Users (What to Tell Them)

Your end users need **ONLY THIS**:

```html
<!DOCTYPE html>
<html>
<body>
    <button rx-action="/increment" rx-method="POST" rx-target="#count" rx-swap="innerHTML">
        +
    </button>
    <span id="count">0</span>

    <!-- Just this one script tag! -->
    <script src="https://cdn.rustx.io/rustx.js"></script>
</body>
</html>
```

**No additional JavaScript. No initialization code. Just works!**

Their backend needs to handle the POST request:

```python
# Python example
@app.route('/increment', methods=['POST'])
def increment():
    count = session.get('count', 0) + 1
    session['count'] = count
    return str(count)  # Return the new count as a string
```

---

## Test It Locally

### With Real Server (Production Example)

```bash
cd examples/basic
python3 server.py
```

Then open: **`http://localhost:8000/minimal-counter.html`**

This is the **real production example** - just the script tag, no mock code!

### Standalone Demo (With Mock)

```bash
python3 -m http.server 8000
```

Then open: `http://localhost:8000/dist-cdn/test.html`

⚠️ **Note:** This has mock JavaScript to simulate a backend. End users won't need this!

---

## Project Structure

```
rustx/
├── build-cdn.sh                    # Build for CDN
├── dist-cdn/                       # CDN-ready files
│   ├── rustx.js                   # Main loader (1.5 KB)
│   ├── rustx.core.js              # Implementation (29 KB)
│   ├── rustx.wasm                 # WASM module (56 KB)
│   └── test.html                  # Test (has mock)
├── examples/
│   ├── basic/
│   │   ├── minimal-counter.html   # ⭐ PRODUCTION EXAMPLE
│   │   ├── counter.html           # Full featured
│   │   ├── server.py              # Real backend
│   │   └── index.html             # Comprehensive demos
│   ├── simple-counter.html        # Standalone (has mock)
│   └── USAGE.md                   # Usage guide
├── src/lib.rs                      # Rust source
├── README.md                       # Documentation
└── CDN-DEPLOYMENT.md              # Deployment guide
```

---

## Key Files

### For Production Examples (No Mock)
- **`examples/basic/minimal-counter.html`** ← **Start here!**
- `examples/basic/counter.html`
- `examples/basic/index.html`

These require `python3 server.py` but show exactly what end users need.

### For Standalone Demos (Has Mock)
- `dist-cdn/test.html`
- `examples/simple-counter.html`

These work with `python3 -m http.server` but include mock JavaScript for demonstration.

---

## Build Commands

### For CDN Distribution
```bash
./build-cdn.sh
```

Creates `dist-cdn/` with ~87 KB total.

### For Development
```bash
wasm-pack build --target web
```

### Run Examples
```bash
cd examples/basic
./run.sh
```

Or manually:
```bash
cd examples/basic
python3 server.py
# Open http://localhost:8000/minimal-counter.html
```

---

## What Changed?

RustX can now be included with **just one script tag**:

### Before (Complex):
```html
<script type="module">
    import init from './pkg/rustx_wasm.js';
    await init();
    // ... more code
</script>
```

### After (Simple):
```html
<script src="https://cdn.rustx.io/rustx.js"></script>
```

That's it! Auto-initializes, no JavaScript needed!

---

## Important: The Mock Code

Some test files include this:

```javascript
// Mock API endpoints
let counter = 0;
window.fetch = function(url, options) {
    if (url.endsWith('/increment')) {
        counter++;
        return Promise.resolve(new Response(counter.toString()));
    }
    // ...
};
```

**This is ONLY for standalone demos!**

End users **DON'T need this**. They only need:
1. The script tag
2. A real backend

The mock is just so demos work without a backend server.

---

## Deploy to CDN

See [CDN-DEPLOYMENT.md](./CDN-DEPLOYMENT.md) for options:
- GitHub Pages (free, easy)
- Cloudflare R2 (free tier, fast)
- AWS S3 + CloudFront
- jsDelivr/unpkg (via npm)

---

## Common Patterns

### Load Data
```html
<button rx-action="/data" rx-target="#result">Load</button>
<div id="result"></div>
```

### Submit Form
```html
<form rx-action="/submit" rx-target="#message">
    <input name="email" required>
    <button type="submit">Send</button>
</form>
```

### Append Items
```html
<button rx-action="/item" rx-target="#list" rx-swap="beforeend">Add</button>
<ul id="list"></ul>
```

---

## Summary

✅ **One script tag** - `<script src="rustx.js"></script>`
✅ **Auto-initializes** - No JavaScript code needed
✅ **Small** - ~87 KB total
✅ **Fast** - WebAssembly performance
✅ **Universal** - Works with any backend

**The mock fetch code is only in test files for demonstration!**

See `examples/basic/minimal-counter.html` for the real production example.

---

**You're all set!** 🎉

RustX is production-ready with a simple one-script-tag installation!
