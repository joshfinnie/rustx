# Testing RustX WASM

## Quick Test

I've identified and fixed two major issues:

### Issues Fixed:

1. **DOMContentLoaded Timing** - The WASM module was loading after DOMContentLoaded fired, so elements weren't being initialized. Fixed by checking `document.readyState`.

2. **GET/DELETE Query Parameters** - The code was creating URL parameters but not using them. Fixed to properly append form data as query params for GET/DELETE requests.

### Test the Counter:

```bash
# From the project root, run:
python3 test-server.py
```

Then open in your browser:
- Test page with debugging: `http://localhost:8000/test.html`
- Full counter example: `http://localhost:8000/counter.html`

### What to Check:

1. Open the browser console (F12)
2. On test.html, you should see:
   - "✅ WASM loaded successfully!"
   - Event logs when clicking buttons
   - Counter should increment/decrement

3. The buttons should have `__rustx` property set after initialization

### If it Still Doesn't Work:

Check the browser console for:
- WASM loading errors
- Network errors (check Network tab)
- JavaScript errors

Common issues:
- Browser caching old WASM - hard refresh (Ctrl+Shift+R / Cmd+Shift+R)
- CORS errors - make sure using the test server
- Module not found - make sure pkg/ folder is in examples/basic/

### Manual Build & Test:

```bash
# 1. Build WASM
wasm-pack build --target web

# 2. Copy to examples
cp -r pkg examples/basic/

# 3. Run server
cd examples/basic
python3 server.py

# 4. Open browser
# http://localhost:8000/counter.html
```

Let me know what you see in the console!
