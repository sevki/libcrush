# 🚀 Quick Start: Publishing mycrush to npm

This is a quick-start guide for publishing the mycrush WASM package to npm.

## Prerequisites

- [x] Rust and Cargo installed
- [x] wasm-pack installed (`cargo install wasm-pack`)
- [x] npm account (create at [npmjs.com](https://www.npmjs.com/signup))
- [x] Logged in to npm (`npm login`)

## Step 1: Build the WASM Package

```bash
cd mycrush
./build-wasm.sh
```

This creates three package directories:
- `pkg/` - For web (ES modules)
- `pkg-node/` - For Node.js
- `pkg-bundler/` - For bundlers (recommended for publishing)

## Step 2: Test Locally

```bash
cd pkg-node

# Quick test
cat > test.mjs << 'EOF'
import { WasmCrushMap, getVersion } from './mycrush.js';
console.log('Version:', getVersion());
const map = new WasmCrushMap();
console.log('✅ Works!');
EOF

node test.mjs
```

You should see:
```
Version: 0.1.2
✅ Works!
```

## Step 3: Choose Package Name

The default package name is `mycrush`. If this is taken on npm, you have two options:

### Option A: Use a Scoped Package (Recommended)

Edit `pkg-bundler/package.json`:
```json
{
  "name": "@yourusername/mycrush",
  ...
}
```

### Option B: Choose a Different Name

Edit `pkg-bundler/package.json`:
```json
{
  "name": "mycrush-wasm",
  ...
}
```

## Step 4: Publish to npm

```bash
cd pkg-bundler

# Dry run (test without actually publishing)
npm publish --dry-run

# Real publish
npm publish --access public
```

For scoped packages, the `--access public` flag is required for free accounts.

## Step 5: Verify Publication

```bash
# Check it's available
npm view @yourusername/mycrush

# Or for unscoped
npm view mycrush-wasm
```

## Step 6: Test Installation

```bash
# In a new directory
mkdir /tmp/test-install
cd /tmp/test-install

# Install your package
npm install @yourusername/mycrush

# Create test file
cat > test.mjs << 'EOF'
import { WasmCrushMap, getVersion } from '@yourusername/mycrush';
console.log('Installed version:', getVersion());
EOF

node test.mjs
```

## Alternative: Publish to GitHub Packages

If you prefer to publish to GitHub Packages instead:

### Step 1: Update package.json

```bash
cd pkg-bundler

# Update package name
node -e "const pkg=require('./package.json'); pkg.name='@sevki/mycrush'; require('fs').writeFileSync('package.json', JSON.stringify(pkg, null, 2));"
```

### Step 2: Authenticate

```bash
npm login --registry=https://npm.pkg.github.com
```

### Step 3: Publish

```bash
npm publish --registry=https://npm.pkg.github.com
```

### Step 4: Install from GitHub Packages

Create `.npmrc` in your project:
```
@sevki:registry=https://npm.pkg.github.com
```

Then install:
```bash
npm install @sevki/mycrush
```

## Using the Published Package

### In Browser (ES Modules)

```html
<script type="module">
import init, { WasmCrushMap } from 'https://unpkg.com/@yourusername/mycrush';
await init();
const map = new WasmCrushMap();
</script>
```

### In Node.js

```javascript
import { WasmCrushMap, getVersion } from '@yourusername/mycrush';

const map = new WasmCrushMap();
console.log('Version:', getVersion());
```

### With TypeScript

```typescript
import { WasmCrushMap, WasmBucketAlgorithm } from '@yourusername/mycrush';

const map = new WasmCrushMap();
const tries: number = map.chooseLocalTries();
```

## Automated Publishing with GitHub Actions

For automated publishing on releases:

1. Add NPM_TOKEN to GitHub Secrets:
   - Go to GitHub repository → Settings → Secrets
   - Add new secret: `NPM_TOKEN`
   - Value: Your npm access token (get from npmjs.com)

2. The workflow is already set up in `.github/workflows/publish-wasm.yml`

3. Publish by:
   - Creating a release on GitHub, OR
   - Manually triggering the workflow in Actions tab

## Version Updates

To publish a new version:

1. Update version in `mycrush/Cargo.toml`:
   ```toml
   version = "0.2.0"
   ```

2. Rebuild and republish:
   ```bash
   ./build-wasm.sh
   cd pkg-bundler
   npm publish
   ```

## Troubleshooting

### "You do not have permission to publish"
- Make sure you're logged in: `npm whoami`
- Use scoped packages: `@yourusername/mycrush`

### "Package name already exists"
- Choose a different name or use a scoped package
- Check availability: `npm view mycrush`

### "WASM module not found"
- Make sure you built with `./build-wasm.sh`
- Check that `*.wasm` files are in the package

## Success Criteria

You've successfully published when:
- ✅ `npm publish` completes without errors
- ✅ `npm view @yourusername/mycrush` shows your package
- ✅ Installing in a new project works
- ✅ Importing and using the package works

## Next Steps

After publishing:
- Update your project README with installation instructions
- Share on social media, Reddit, etc.
- Add a badge to your GitHub README:
  ```markdown
  [![npm version](https://badge.fury.io/js/%40yourusername%2Fmycrush.svg)](https://www.npmjs.com/package/@yourusername/mycrush)
  ```

## Support

For detailed documentation, see:
- [WASM.md](WASM.md) - Complete WASM usage guide
- [PUBLISHING.md](PUBLISHING.md) - Detailed publishing guide
- [TESTING.md](TESTING.md) - Testing instructions

## Need Help?

- Check npm docs: https://docs.npmjs.com/
- Ask in GitHub issues
- Check the examples in `examples/wasm/`
