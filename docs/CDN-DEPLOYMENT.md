# CDN Deployment Guide

This guide explains how to deploy RustX to a CDN for easy public access.

## Building for CDN

Run the build script:

```bash
./build-cdn.sh
```

This creates a `dist-cdn/` folder with:
- `rustx.js` - Main loader (~1.5 KB)
- `rustx.core.js` - Core implementation (~29 KB)
- `rustx.wasm` - WebAssembly module (~56 KB)
- `version.json` - Version metadata
- `test.html` - Test page

**Total size: ~87 KB** (smaller than most JS frameworks!)

## Deployment Options

### Option 1: GitHub Pages (Free & Easy)

1. Create a new repository or use an existing one
2. Copy `dist-cdn/*` to a `cdn/` folder in your repo
3. Enable GitHub Pages in repository settings
4. Your files will be available at:
   ```
   https://yourusername.github.io/yourrepo/cdn/rustx.js
   ```

**Example:**
```bash
# In your repo
mkdir -p cdn
cp dist-cdn/* cdn/
git add cdn/
git commit -m "Add RustX CDN files"
git push
```

### Option 2: Cloudflare R2 / Workers

Cloudflare R2 offers generous free tier and excellent global performance.

1. Create an R2 bucket:
   ```bash
   wrangler r2 bucket create rustx-cdn
   ```

2. Upload files:
   ```bash
   wrangler r2 object put rustx-cdn/rustx.js --file=dist-cdn/rustx.js
   wrangler r2 object put rustx-cdn/rustx.core.js --file=dist-cdn/rustx.core.js
   wrangler r2 object put rustx-cdn/rustx.wasm --file=dist-cdn/rustx.wasm
   ```

3. Set up custom domain and CORS headers via Cloudflare Workers

### Option 3: AWS S3 + CloudFront

1. Create S3 bucket:
   ```bash
   aws s3 mb s3://rustx-cdn
   ```

2. Upload files:
   ```bash
   aws s3 sync dist-cdn/ s3://rustx-cdn/ \
     --acl public-read \
     --cache-control "max-age=31536000"
   ```

3. Set up CloudFront distribution for global CDN

4. Configure CORS:
   ```json
   {
     "CORSRules": [{
       "AllowedOrigins": ["*"],
       "AllowedMethods": ["GET"],
       "AllowedHeaders": ["*"],
       "MaxAgeSeconds": 3600
     }]
   }
   ```

### Option 4: jsDelivr (NPM Package)

1. Publish to npm:
   ```bash
   # Add to package.json
   {
     "name": "rustx-wasm",
     "version": "0.1.0",
     "files": ["dist-cdn/*"]
   }

   npm publish
   ```

2. Files automatically available at:
   ```
   https://cdn.jsdelivr.net/npm/rustx-wasm@latest/dist-cdn/rustx.js
   ```

### Option 5: unpkg (NPM Package)

After publishing to npm:
```
https://unpkg.com/rustx-wasm@latest/dist-cdn/rustx.js
```

### Option 6: Custom Domain

For `cdn.rustx.io` or similar:

1. Choose a hosting provider (Cloudflare, AWS, etc.)
2. Upload the `dist-cdn/` files
3. Configure DNS to point to your CDN
4. Set up SSL certificate (Let's Encrypt, Cloudflare, etc.)
5. Configure CORS headers

**Nginx example config:**
```nginx
server {
    listen 443 ssl http2;
    server_name cdn.rustx.io;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    root /var/www/rustx-cdn;

    # CORS headers
    add_header Access-Control-Allow-Origin *;
    add_header Access-Control-Allow-Methods "GET, OPTIONS";
    add_header Access-Control-Allow-Headers "Content-Type";

    # Cache control
    location ~* \.(js|wasm)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Correct MIME types
    types {
        application/javascript js;
        application/wasm wasm;
    }
}
```

## Required Configuration

### MIME Types

Ensure your server sends correct MIME types:
- `*.js` → `application/javascript` or `text/javascript`
- `*.wasm` → `application/wasm`

### CORS Headers

For cross-origin requests:
```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, OPTIONS
Access-Control-Allow-Headers: Content-Type
```

### Cache Headers

For optimal performance:
```
Cache-Control: public, max-age=31536000, immutable
```

## Versioning Strategy

### Immutable URLs (Recommended)

Include version in URL:
```
https://cdn.rustx.io/v0.1.0/rustx.js
https://cdn.rustx.io/v0.1.0/rustx.core.js
https://cdn.rustx.io/v0.1.0/rustx.wasm
```

Benefits:
- Aggressive caching (1 year+)
- No breaking changes for existing users
- Easy rollback

### SemVer Aliases

Provide version aliases:
```
https://cdn.rustx.io/v0/rustx.js       → latest v0.x
https://cdn.rustx.io/v0.1/rustx.js     → latest v0.1.x
https://cdn.rustx.io/latest/rustx.js   → latest release
```

## Testing Your CDN Deployment

1. Test from different locations:
   ```bash
   curl https://cdn.rustx.io/rustx.js
   curl https://cdn.rustx.io/rustx.core.js
   curl https://cdn.rustx.io/rustx.wasm
   ```

2. Check CORS:
   ```bash
   curl -H "Origin: https://example.com" \
        -H "Access-Control-Request-Method: GET" \
        -X OPTIONS \
        https://cdn.rustx.io/rustx.js
   ```

3. Verify MIME types:
   ```bash
   curl -I https://cdn.rustx.io/rustx.wasm | grep content-type
   # Should show: content-type: application/wasm
   ```

4. Test the actual page:
   ```html
   <!DOCTYPE html>
   <html>
   <body>
     <button rx-action="/test" rx-method="POST">Test</button>
     <script src="https://cdn.rustx.io/rustx.js"></script>
     <script>
       window.addEventListener('rustx:ready', () => {
         console.log('✅ RustX loaded from CDN!');
       });
     </script>
   </body>
   </html>
   ```

## Monitoring

Track CDN usage with:
- **Cloudflare Analytics** - Free with Cloudflare
- **AWS CloudWatch** - For S3/CloudFront
- **Google Analytics** - Add tracking to rustx.js

## Security Considerations

1. **Subresource Integrity (SRI)**

   Generate SRI hashes:
   ```bash
   cat dist-cdn/rustx.js | openssl dgst -sha384 -binary | openssl base64 -A
   ```

   Users can verify integrity:
   ```html
   <script src="https://cdn.rustx.io/rustx.js"
           integrity="sha384-HASH_HERE"
           crossorigin="anonymous"></script>
   ```

2. **HTTPS Only**
   - Always serve over HTTPS
   - Redirect HTTP to HTTPS

3. **Rate Limiting**
   - Prevent abuse with rate limits
   - Cloudflare provides this by default

## Cost Estimates

### GitHub Pages
- **Free** for public repos
- Unlimited bandwidth for reasonable use

### Cloudflare R2
- First 10 GB/month: **Free**
- After: $0.015/GB
- Very generous free tier

### AWS S3 + CloudFront
- S3 Storage: $0.023/GB/month
- CloudFront: $0.085/GB (first 10 TB)
- Estimate: $5-20/month for moderate traffic

### jsDelivr/unpkg
- **Free** for open source
- Global CDN included

## Recommended Setup

For maximum reliability and performance:

1. **Primary CDN:** Cloudflare R2 with Workers
2. **Fallback:** jsDelivr (via npm)
3. **Custom domain:** cdn.rustx.io
4. **Versioning:** Immutable URLs with version numbers

Example HTML with fallback:
```html
<script
  src="https://cdn.rustx.io/v0.1.0/rustx.js"
  onerror="this.onerror=null; this.src='https://cdn.jsdelivr.net/npm/rustx-wasm@0.1.0/dist-cdn/rustx.js'">
</script>
```

## Next Steps

1. Choose your deployment platform
2. Run `./build-cdn.sh` to create distribution files
3. Upload to your chosen CDN
4. Test with the test.html file
5. Update documentation with your CDN URL
6. Announce to users!

## Questions?

Open an issue on GitHub or check the documentation at https://rustx.io
