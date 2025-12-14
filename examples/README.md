# RustX WASM Examples

This directory contains examples demonstrating how to use rustx-wasm in your web applications.

## Quick Start

### 1. Build the WASM Module

First, you need to build the WASM module using `wasm-pack`:

```bash
# Install wasm-pack if you haven't already
cargo install wasm-pack

# Build the WASM module for web target
wasm-pack build --target web
```

This will create a `pkg/` directory in the project root with the compiled WASM module.

### 2. Copy WASM Files to Example Directory

```bash
# Copy the built WASM package to the basic example directory
cp -r pkg examples/basic/
```

### 3. Run the Example Server

```bash
cd examples/basic
python3 server.py
```

Or specify a custom port:

```bash
python3 server.py 3000
```

### 4. Open in Browser

Navigate to: `http://localhost:8000/index.html`

## What is RustX?

RustX is a lightweight reactive library for web applications, written in Rust and compiled to WebAssembly. It allows you to add dynamic behavior to your HTML using declarative attributes, similar to htmx but powered by Rust/WASM.

## Core Attributes

### `rx-action`
The URL endpoint to send the request to.

```html
<button rx-action="/api/data">Click Me</button>
```

### `rx-method`
The HTTP method to use (GET, POST, PUT, DELETE, etc.). Default is GET for most elements, POST for forms.

```html
<button rx-action="/api/create" rx-method="POST">Create</button>
```

### `rx-target`
CSS selector for the element that should receive the response. Default is the element itself.

```html
<button rx-action="/api/data" rx-target="#result">Load</button>
<div id="result"></div>
```

### `rx-swap`
How to swap the response into the target. Options:
- `innerHTML` - Replace inner content (default for most cases)
- `outerHTML` - Replace the entire element
- `beforebegin` - Insert before the target
- `afterbegin` - Insert as first child
- `beforeend` - Insert as last child
- `afterend` - Insert after the target
- `none` - Don't swap (useful for side effects)

```html
<button rx-action="/api/item" rx-target="#list" rx-swap="beforeend">
    Add Item
</button>
```

### `rx-trigger`
The event that triggers the request. Default is:
- `click` for buttons
- `submit` for forms
- `change` for inputs, selects, textareas

```html
<select rx-action="/api/filter" rx-trigger="change">
    <option>Option 1</option>
</select>
```

### `rx-ignore`
Add this attribute to an element to prevent rustx from processing it or its children.

```html
<div rx-ignore>
    <!-- rustx won't process anything in here -->
    <button rx-action="/api/test">Won't work</button>
</div>
```

## Custom Events

RustX fires several custom events during the request lifecycle that you can listen to:

### Event Lifecycle

1. **`rx:init`** - Fired when rustx initializes an element (cancelable)
2. **`rx:inited`** - Fired after element initialization completes
3. **`rx:config`** - Fired before request configuration (cancelable)
4. **`rx:before`** - Fired before the request is sent (cancelable)
5. **`rx:after`** - Fired after the response is received (cancelable)
6. **`rx:swapped`** - Fired after content is swapped into the DOM
7. **`rx:finally`** - Fired at the end of the request lifecycle

### Example: Adding Loading States

```javascript
document.addEventListener('rx:before', (e) => {
    e.target.classList.add('loading');
});

document.addEventListener('rx:after', (e) => {
    e.target.classList.remove('loading');
});
```

### Example: Preventing a Request

```javascript
document.getElementById('my-button').addEventListener('rx:before', (e) => {
    if (!confirm('Are you sure?')) {
        e.preventDefault(); // Cancel the request
    }
});
```

## Examples Included

### Counter Example (`basic/counter.html`) ⚡

The classic counter example - the "Hello World" of reactive frameworks! This demonstrates the core RustX pattern:

```html
<button rx-action="/decrement" rx-method="POST" rx-target="#count" rx-swap="innerHTML">-</button>
<span id="count">0</span>
<button rx-action="/increment" rx-method="POST" rx-target="#count" rx-swap="innerHTML">+</button>
```

Features:
- Clean, minimal example of reactive HTML
- Server-side state management
- Increment, decrement, and reset functionality
- Beautiful UI with animations

**Start here if you're new to RustX!**

### Comprehensive Examples (`basic/index.html`)

The full example page demonstrates:
- Simple button clicks
- Form submissions
- Different swap modes (innerHTML, beforeend, afterbegin)
- Custom triggers (change events)
- POST requests
- Event listeners and logging

## Building for Production

For production builds, use the release profile:

```bash
wasm-pack build --target web --release
```

This will optimize the WASM bundle for size and performance.

## Troubleshooting

### WASM Module Not Found

If you see errors about missing WASM files, make sure you:
1. Built the module with `wasm-pack build --target web`
2. Copied the `pkg/` directory to the example folder

### CORS Errors

The included Python server handles CORS automatically. If you're using a different server, make sure to set appropriate CORS headers.

### Module Not Loading

Check the browser console for detailed error messages. Common issues:
- Missing or incorrect path to the WASM module
- Server not serving WASM files with correct MIME type
- Browser doesn't support WebAssembly

## Alternative Servers

### Using Python's http.server

```bash
cd examples/basic
python3 -m http.server 8000
```

Note: This won't handle the API endpoints, but you can test the UI.

### Using Node.js http-server

```bash
npm install -g http-server
cd examples/basic
http-server -p 8000 --cors
```

### Using Rust

```bash
cargo install miniserve
cd examples/basic
miniserve . -p 8000
```

## Next Steps

- Check out the source code in `src/lib.rs` to understand how rustx works
- Modify the examples to experiment with different features
- Build your own rustx-powered application!

## License

See the main project LICENSE file.
