# RustX Quick Reference

## Installation

```bash
# One-liner to build and run
./run.sh

# Or manually:
wasm-pack build --target web
cp -r pkg examples/basic/
cd examples/basic && python3 server.py
```

## The Counter Example (Start Here!)

The classic RustX example - three lines of HTML, no JavaScript:

```html
<button rx-action="/decrement" rx-method="POST" rx-target="#count" rx-swap="innerHTML">-</button>
<span id="count">0</span>
<button rx-action="/increment" rx-method="POST" rx-target="#count" rx-swap="innerHTML">+</button>
```

See it live: `http://localhost:8000/counter.html`

## Attributes

| Attribute | Purpose | Default |
|-----------|---------|---------|
| `rx-action` | URL endpoint | Required |
| `rx-method` | HTTP method | GET (click), POST (form) |
| `rx-target` | CSS selector for target | Self |
| `rx-swap` | How to insert response | outerHTML |
| `rx-trigger` | Event to listen for | click/submit/change |
| `rx-ignore` | Disable rustx on element | - |

## Swap Modes

```html
<!-- Replace entire element -->
<div rx-swap="outerHTML">...</div>

<!-- Replace inner content -->
<div rx-swap="innerHTML">...</div>

<!-- Insert positions -->
<div rx-swap="beforebegin">...</div>  <!-- Before element -->
<div rx-swap="afterbegin">...</div>   <!-- First child -->
<div rx-swap="beforeend">...</div>    <!-- Last child -->
<div rx-swap="afterend">...</div>     <!-- After element -->

<!-- No swap (side effects only) -->
<div rx-swap="none">...</div>
```

## Common Patterns

### Basic Button

```html
<button rx-action="/api/data">Click Me</button>
```

### Load into Different Element

```html
<button rx-action="/api/data" rx-target="#result">Load</button>
<div id="result"></div>
```

### Form Submission

```html
<form rx-action="/api/submit" rx-target="#result">
    <input type="text" name="username">
    <button type="submit">Submit</button>
</form>
```

### Append to List

```html
<button rx-action="/api/item" rx-target="#list" rx-swap="beforeend">
    Add Item
</button>
<ul id="list"></ul>
```

### On Change Event

```html
<select rx-action="/api/filter" rx-trigger="change">
    <option>Option 1</option>
</select>
```

### POST Request

```html
<button
    rx-action="/api/create"
    rx-method="POST"
    name="action"
    value="create">
    Create
</button>
```

## Events API

### Event Lifecycle Order

```
rx:init → rx:inited → rx:config → rx:before → [request] → rx:after → rx:swapped → rx:finally
```

### Listen to Events

```javascript
// Add loading state
element.addEventListener('rx:before', (e) => {
    e.target.classList.add('loading');
});

element.addEventListener('rx:after', (e) => {
    e.target.classList.remove('loading');
});

// Cancel request
element.addEventListener('rx:before', (e) => {
    if (!confirm('Continue?')) {
        e.preventDefault();
    }
});

// Log all events
['rx:init', 'rx:before', 'rx:after', 'rx:swapped'].forEach(evt => {
    element.addEventListener(evt, (e) => console.log(evt, e));
});
```

### Cancelable Events

- `rx:init` - Cancel element initialization
- `rx:config` - Cancel request configuration
- `rx:before` - Cancel the request

### Event Properties

All events:
- `target` - The element with rx-action
- `bubbles` - Most events bubble (except rx:inited)
- `cancelable` - Init, config, before, after are cancelable

## JavaScript API

### Initialize WASM

```javascript
import init from './pkg/rustx_wasm.js';

await init();
```

### Manually Trigger Processing

```javascript
// Trigger processing on an element
const event = new CustomEvent('rx:process');
element.dispatchEvent(event);
```

### Prevent Processing

```html
<!-- rustx won't process this section -->
<div rx-ignore>
    <button rx-action="/api/data">Won't work</button>
</div>
```

## Server Response Format

RustX expects HTML responses:

```python
# Python example
def handle_request():
    return '<div>Hello from server!</div>'
```

```javascript
// Node.js example
app.get('/api/data', (req, res) => {
    res.send('<div>Hello from server!</div>');
});
```

The server receives a header: `RX-Request: true`

## Tips & Tricks

### Loading States

```css
.loading {
    opacity: 0.6;
    pointer-events: none;
}
```

```javascript
document.addEventListener('rx:before', e => e.target.classList.add('loading'));
document.addEventListener('rx:after', e => e.target.classList.remove('loading'));
```

### Form Validation

```javascript
form.addEventListener('rx:before', (e) => {
    if (!form.checkValidity()) {
        e.preventDefault();
        form.reportValidity();
    }
});
```

### Confirmation Dialogs

```javascript
deleteBtn.addEventListener('rx:before', (e) => {
    if (!confirm('Delete this item?')) {
        e.preventDefault();
    }
});
```

### Progress Indicators

```javascript
let requestCount = 0;

document.addEventListener('rx:before', () => {
    if (++requestCount === 1) {
        showProgressBar();
    }
});

document.addEventListener('rx:after', () => {
    if (--requestCount === 0) {
        hideProgressBar();
    }
});
```

## Debugging

### Enable Console Logging

```javascript
// Log all rustx events
['rx:init', 'rx:inited', 'rx:config', 'rx:before', 'rx:after', 'rx:swapped', 'rx:finally']
    .forEach(evt => {
        document.addEventListener(evt, (e) => {
            console.log(`[RustX] ${evt}`, e.target);
        });
    });
```

### Check if RustX is Loaded

```javascript
// The WASM module sets a marker on initialized elements
const isInitialized = element.__rustx !== undefined;
```

### Inspect Request Headers

All rustx requests include: `RX-Request: true`

## Browser Support

- Modern browsers with WebAssembly support
- Chrome/Edge 57+
- Firefox 52+
- Safari 11+
- No IE support

## Performance Tips

1. Use `rx-ignore` on sections that don't need reactivity
2. Prefer `innerHTML` over `outerHTML` when possible
3. Use release builds in production: `wasm-pack build --release`
4. Enable gzip compression on your server for WASM files
5. Consider code splitting for large applications
