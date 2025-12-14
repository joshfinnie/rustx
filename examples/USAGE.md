# RustX Usage Guide

## For End Users (Production)

Your end users need **ONLY THIS**:

```html
<!DOCTYPE html>
<html>
<body>
    <button rx-action="/increment" rx-method="POST" rx-target="#count" rx-swap="innerHTML">
        +
    </button>
    <span id="count">0</span>

    <!-- That's it! Just this one script tag -->
    <script src="https://cdn.rustx.io/rustx.js"></script>
</body>
</html>
```

**No additional JavaScript needed!**

### Backend Requirements

Your server just needs to handle the HTTP requests and return HTML:

**Python (Flask):**
```python
@app.route('/increment', methods=['POST'])
def increment():
    session['count'] = session.get('count', 0) + 1
    return str(session['count'])
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

That's it! Server returns HTML/text, RustX swaps it in.

---

## For Testing/Demo (Development)

The example files in this repo have **mock fetch** code for demonstration purposes only. This is because:

1. `python3 -m http.server` doesn't support POST requests
2. We want standalone demos that work without a backend

**The mock code is NOT needed in production!**

### Test Files (with mock):
- `dist-cdn/test.html` - CDN test with mock server
- `examples/simple-counter.html` - Demo with mock server

These use fetch interception to simulate a backend. **This is only for demos.**

### Production Files (no mock):
- `examples/basic/minimal-counter.html` - Real server example
- `examples/basic/counter.html` - Full featured, real server

These require `python3 server.py` but show real usage.

---

## Running Examples

### Option 1: With Real Server (Recommended)

```bash
cd examples/basic
python3 server.py
```

Then open:
- `http://localhost:8000/minimal-counter.html` ← **Minimal production example**
- `http://localhost:8000/counter.html` ← Full-featured example

These show **exactly** what end users need - just the script tag!

### Option 2: Standalone Demo

```bash
python3 -m http.server 8000
```

Then open:
- `http://localhost:8000/dist-cdn/test.html` ← Has mock for demo
- `http://localhost:8000/examples/simple-counter.html` ← Has mock for demo

These have extra JavaScript to mock the backend. **Don't show this to users!**

---

## What to Tell Your Users

### Installation

```html
<script src="https://cdn.rustx.io/rustx.js"></script>
```

### Usage

Add attributes to your HTML:

```html
<button rx-action="/api/data" rx-target="#result">
    Click Me
</button>
<div id="result"></div>
```

### That's All

No npm install. No build step. No JavaScript code needed.

Just make sure your backend endpoint returns HTML.

---

## Key Point

**The fetch mock in test files is ONLY for demos!**

In production, users just need:
1. The script tag: `<script src="rustx.js"></script>`
2. A backend that handles the requests
3. HTML attributes on their elements

**Zero additional JavaScript required.**

---

## File Guide

| File | Purpose | Needs Server? | Has Mock? |
|------|---------|---------------|-----------|
| `examples/basic/minimal-counter.html` | **Production example** | ✅ Yes | ❌ No |
| `examples/basic/counter.html` | Full example | ✅ Yes | ❌ No |
| `examples/basic/index.html` | Comprehensive demos | ✅ Yes | ❌ No |
| `dist-cdn/test.html` | CDN test | ❌ No | ✅ Yes (demo only) |
| `examples/simple-counter.html` | Standalone demo | ❌ No | ✅ Yes (demo only) |

**Use the files without mocks to show users how simple RustX is!**
