#!/bin/bash
# Build RustX for CDN distribution
# Creates a simple one-script-tag installation

set -e

echo "🦀 Building RustX for CDN distribution..."
echo ""

# Clean previous builds
rm -rf dist-cdn
mkdir -p dist-cdn

# Build with no-modules target (for script tag usage)
echo "📦 Building WASM module..."
wasm-pack build --target no-modules --out-dir dist --release

# Copy and rename files for CDN
echo "📝 Preparing CDN files..."
cp dist/rustx_wasm_bg.wasm dist-cdn/rustx.wasm
cp dist/rustx_wasm.js dist-cdn/rustx.core.js

# Create the auto-initializing wrapper
cat > dist-cdn/rustx.js << 'EOF'
/**
 * RustX - Reactive HTML powered by Rust & WebAssembly
 * Simply include this script to add reactive capabilities to your HTML
 *
 * Usage:
 *   <script src="https://cdn.rustx.io/rustx.js"></script>
 *
 * No initialization needed - it works automatically!
 */

(function() {
    'use strict';

    // Load the core WASM module
    const script = document.currentScript;
    const scriptUrl = new URL(script.src);
    const baseUrl = scriptUrl.href.substring(0, scriptUrl.href.lastIndexOf('/'));

    // Load the core implementation
    const coreScript = document.createElement('script');
    coreScript.src = baseUrl + '/rustx.core.js';
    coreScript.async = false;

    coreScript.onload = function() {
        // Auto-initialize when core is loaded
        if (typeof wasm_bindgen === 'function') {
            // Calculate WASM URL
            const wasmUrl = baseUrl + '/rustx.wasm';

            wasm_bindgen(wasmUrl).then(() => {
                console.log('✨ RustX loaded and ready!');

                // Dispatch custom event for those who want to know
                if (typeof window !== 'undefined') {
                    window.dispatchEvent(new Event('rustx:ready'));
                }
            }).catch(err => {
                console.error('❌ RustX failed to initialize:', err);
            });
        }
    };

    coreScript.onerror = function() {
        console.error('❌ Failed to load RustX core module');
    };

    document.head.appendChild(coreScript);
})();
EOF

# Create a minified version info
cat > dist-cdn/version.json << EOF
{
  "version": "0.1.0",
  "build_date": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "files": {
    "rustx.js": "Main loader (auto-initializing)",
    "rustx.core.js": "Core implementation",
    "rustx.wasm": "WebAssembly module"
  },
  "usage": "<script src=\"https://cdn.rustx.io/rustx.js\"></script>"
}
EOF

# Create a simple test HTML
cat > dist-cdn/test.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>RustX CDN Test</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
            max-width: 600px;
            margin: 50px auto;
            padding: 20px;
        }
        button {
            padding: 10px 20px;
            margin: 5px;
            font-size: 16px;
            cursor: pointer;
        }
        #count {
            font-size: 48px;
            font-weight: bold;
            color: #667eea;
            margin: 0 20px;
        }
        .status {
            padding: 10px;
            margin: 10px 0;
            border-radius: 5px;
            background: #f0f0f0;
        }
    </style>
</head>
<body>
    <h1>🦀 RustX CDN Test</h1>

    <div class="status" id="status">
        Loading RustX...
    </div>

    <h2>Counter Example</h2>
    <div style="display: flex; align-items: center; justify-content: center;">
        <button rx-action="/decrement" rx-method="POST" rx-target="#count" rx-swap="innerHTML">−</button>
        <span id="count">0</span>
        <button rx-action="/increment" rx-method="POST" rx-target="#count" rx-swap="innerHTML">+</button>
    </div>

    <script>
        // Mock API endpoints - MUST be set up BEFORE RustX loads!
        let counter = 0;
        const originalFetch = window.fetch;

        window.fetch = function(url, options) {
            const urlString = typeof url === 'string' ? url : url.url;

            console.log('Fetch intercepted:', urlString, options);

            // Handle both relative and absolute URLs
            if (urlString.endsWith('/increment') || urlString === '/increment') {
                counter++;
                console.log('→ Increment! Counter is now:', counter);
                return Promise.resolve(new Response(counter.toString(), {
                    status: 200,
                    headers: { 'Content-Type': 'text/html' }
                }));
            } else if (urlString.endsWith('/decrement') || urlString === '/decrement') {
                counter--;
                console.log('→ Decrement! Counter is now:', counter);
                return Promise.resolve(new Response(counter.toString(), {
                    status: 200,
                    headers: { 'Content-Type': 'text/html' }
                }));
            }

            // For other URLs (like WASM), use the original fetch
            return originalFetch.apply(this, arguments);
        };
    </script>

    <!-- Single script tag - that's it! -->
    <script src="rustx.js"></script>

    <script>
        // Optional: Listen for when RustX is ready
        window.addEventListener('rustx:ready', () => {
            document.getElementById('status').innerHTML =
                '✅ <strong>RustX loaded!</strong> The counter buttons should work now.';
            document.getElementById('status').style.background = '#d4edda';
            document.getElementById('status').style.color = '#155724';
        });
    </script>
</body>
</html>
EOF

# Create README for CDN distribution
cat > dist-cdn/README.md << 'EOF'
# RustX CDN Distribution

## Quick Start

Add this single line to your HTML:

```html
<script src="https://cdn.rustx.io/rustx.js"></script>
```

That's it! RustX will automatically initialize and start working.

## Example

```html
<!DOCTYPE html>
<html>
<head>
    <title>My RustX App</title>
</head>
<body>
    <button rx-action="/increment" rx-method="POST" rx-target="#count" rx-swap="innerHTML">
        +
    </button>
    <span id="count">0</span>

    <!-- Just include this one script -->
    <script src="https://cdn.rustx.io/rustx.js"></script>
</body>
</html>
```

## Files in This Distribution

- `rustx.js` - Main loader (include this in your HTML)
- `rustx.core.js` - Core implementation (loaded automatically)
- `rustx.wasm` - WebAssembly module (loaded automatically)
- `version.json` - Version information

## Optional: Wait for Ready Event

If you need to run code after RustX is loaded:

```javascript
window.addEventListener('rustx:ready', () => {
    console.log('RustX is ready!');
});
```

## Hosting

To host RustX on your own CDN:

1. Upload all three files (`rustx.js`, `rustx.core.js`, `rustx.wasm`) to the same directory
2. Ensure CORS headers are set correctly
3. Serve with correct MIME types:
   - `rustx.js` and `rustx.core.js`: `application/javascript`
   - `rustx.wasm`: `application/wasm`

## Size

- `rustx.js`: ~1 KB (loader)
- `rustx.core.js`: ~27 KB (implementation)
- `rustx.wasm`: ~56 KB (WASM module)
- **Total: ~84 KB** (comparable to htmx!)

## Attributes Reference

- `rx-action` - URL endpoint
- `rx-method` - HTTP method (GET, POST, etc.)
- `rx-target` - CSS selector for target element
- `rx-swap` - How to insert response (innerHTML, outerHTML, etc.)
- `rx-trigger` - Event that triggers request

See full documentation at: https://rustx.io/docs
EOF

echo ""
echo "✅ Build complete!"
echo ""
echo "📁 CDN files created in: dist-cdn/"
echo ""
echo "Files:"
ls -lh dist-cdn/ | grep -v "^total" | awk '{print "  " $9 " (" $5 ")"}'
echo ""
echo "🧪 Test it:"
echo "  cd dist-cdn"
echo "  python3 -m http.server 8000"
echo "  open http://localhost:8000/test.html"
echo ""
