# 🚀 WASM-Pack Setup Complete!

This PR adds complete wasm-pack support to the mycrush library, enabling it to be published to npm or GitHub Packages.

## What's New

### Core Changes

1. **Cargo.toml Updates** (`mycrush/Cargo.toml`):
   - Added `crate-type = ["cdylib", "rlib"]` for WASM builds
   - Added `wasm-bindgen` dependency for WASM target
   - Configured wasm-pack metadata

2. **WASM Bindings** (`mycrush/src/wasm.rs`):
   - Created JavaScript-friendly API wrappers
   - Exposed `WasmCrushMap` class for JS/TS
   - Added helper functions like `getVersion()` and `isMultiplicationUnsafe()`
   - TypeScript definitions auto-generated

3. **Build Script** (`mycrush/build-wasm.sh`):
   - Builds for three targets:
     - Web (ES modules)
     - Node.js
     - Bundlers (webpack, rollup, etc.)
   - Simple one-command build process

### Documentation

1. **WASM.md** - Comprehensive guide for using the WASM build:
   - Building instructions
   - Usage examples for browser, Node.js, and bundlers
   - Complete API reference
   - TypeScript examples

2. **PUBLISHING.md** - Step-by-step npm publishing guide:
   - Publishing to npm
   - Publishing to GitHub Packages
   - Version management
   - CI/CD integration

3. **Examples** (`mycrush/examples/wasm/`):
   - `index.html` - Interactive browser demo
   - `node-example.js` - Node.js usage example

### CI/CD

**GitHub Actions Workflow** (`.github/workflows/publish-wasm.yml`):
- Automated WASM building
- One-click publishing to npm or GitHub Packages
- Can be triggered manually or on release
- Includes caching for faster builds

## How to Use

### Building WASM Packages

```bash
cd mycrush
./build-wasm.sh
```

This creates:
- `pkg/` - For web browsers
- `pkg-node/` - For Node.js  
- `pkg-bundler/` - For bundlers

### Publishing to npm

```bash
cd mycrush/pkg-bundler
npm publish
```

Or use the GitHub Actions workflow for automated publishing.

### Using in JavaScript/TypeScript

**Browser:**
```javascript
import init, { WasmCrushMap } from 'mycrush';

await init();
const map = new WasmCrushMap();
console.log('Tries:', map.chooseLocalTries());
```

**Node.js:**
```javascript
import { WasmCrushMap, getVersion } from 'mycrush';

const map = new WasmCrushMap();
console.log('Version:', getVersion());
```

## Testing

All existing Rust tests pass:
```bash
cargo test --package mycrush
```

WASM builds successfully for all targets:
```bash
./mycrush/build-wasm.sh
```

## Generated Package Contents

Each published package includes:
- `*.wasm` - WebAssembly binary (~31KB)
- `*.js` - JavaScript glue code
- `*.d.ts` - TypeScript type definitions
- `package.json` - NPM metadata
- `README.md` - Documentation

## API Exposed to JavaScript

### WasmCrushMap Class
- `constructor()` / `newLegacy()` - Create maps
- Property getters (chooseLocalTries, etc.)
- `finalize()` - Prepare map for use
- `findRoots()` - Get root buckets
- `hasRules()` - Check for rules

### Helper Functions
- `getVersion()` - Library version
- `isMultiplicationUnsafe(a, b)` - Overflow check

### Enums
- `WasmBucketAlgorithm` - Bucket types (Uniform, List, Tree, Straw, Straw2)

## Benefits

✅ **Ready for npm** - Publish to npm or GitHub Packages immediately  
✅ **Multi-platform** - Works in browsers, Node.js, and with bundlers  
✅ **Type-safe** - Full TypeScript support with auto-generated definitions  
✅ **Well-documented** - Comprehensive guides and examples  
✅ **Automated** - GitHub Actions workflow for easy publishing  
✅ **Tested** - All existing tests pass, WASM builds verified  
✅ **Small footprint** - ~31KB WASM binary  

## Next Steps

To publish the package:

1. **Review** the generated `package.json` files
2. **Test** locally with `npm pack` and install the tarball
3. **Publish** via `npm publish` or use the GitHub Actions workflow
4. **Configure** npm token secret in GitHub for automated publishing

For detailed instructions, see:
- [WASM.md](mycrush/WASM.md) - Usage guide
- [PUBLISHING.md](mycrush/PUBLISHING.md) - Publishing guide

## Files Changed

- `mycrush/Cargo.toml` - WASM dependencies and config
- `mycrush/src/lib.rs` - Include wasm module
- `mycrush/src/wasm.rs` - NEW: WASM bindings
- `mycrush/README.md` - Updated build instructions
- `mycrush/build-wasm.sh` - NEW: Build script
- `mycrush/WASM.md` - NEW: WASM usage guide
- `mycrush/PUBLISHING.md` - NEW: Publishing guide
- `mycrush/examples/wasm/` - NEW: Examples
- `.github/workflows/publish-wasm.yml` - NEW: CI/CD workflow
- `mycrush/.gitignore` - Ignore generated packages

No breaking changes to existing Rust API or builds!
