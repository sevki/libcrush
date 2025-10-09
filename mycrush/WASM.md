# 🥰 mycrush WebAssembly Guide

This guide explains how to build and use mycrush as a WebAssembly package for JavaScript/TypeScript projects.

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
# Install wasm-pack
cargo install wasm-pack
```

## Building for WASM

The simplest way to build all WASM targets is to use the provided script:

```bash
cd mycrush
./build-wasm.sh
```

This will create three build outputs:
- `pkg/` - For web browsers (ES modules)
- `pkg-node/` - For Node.js
- `pkg-bundler/` - For bundlers like webpack, rollup, parcel

### Manual Build Commands

If you prefer to build manually:

```bash
# For web (ES modules)
wasm-pack build --target web --out-dir pkg

# For Node.js
wasm-pack build --target nodejs --out-dir pkg-node

# For bundlers (webpack, rollup, etc.)
wasm-pack build --target bundler --out-dir pkg-bundler
```

## Publishing to npm

Once built, you can publish to npm:

```bash
cd pkg  # or pkg-node or pkg-bundler
npm publish
```

To publish to GitHub npm registry:

```bash
cd pkg
npm publish --registry=https://npm.pkg.github.com
```

## Usage Examples

### Browser (ES Modules)

```html
<!DOCTYPE html>
<html>
<head>
    <title>mycrush Example</title>
</head>
<body>
    <script type="module">
        import init, { WasmCrushMap, getVersion } from './pkg/mycrush.js';

        async function run() {
            // Initialize the WASM module
            await init();
            
            // Get version
            console.log('Version:', getVersion());
            
            // Create a new CRUSH map
            const map = new WasmCrushMap();
            console.log('Map created');
            
            // Get properties
            console.log('Choose Local Tries:', map.chooseLocalTries());
            console.log('Has Rules:', map.hasRules());
            
            // Finalize the map
            map.finalize();
        }

        run();
    </script>
</body>
</html>
```

### Node.js

First, install the package:

```bash
npm install /path/to/mycrush/pkg-node
```

Then use it:

```javascript
import { WasmCrushMap, getVersion, isMultiplicationUnsafe } from 'mycrush';

// Get version
console.log('Version:', getVersion());

// Create a new CRUSH map
const map = new WasmCrushMap();
console.log('Map Properties:', {
    chooseLocalTries: map.chooseLocalTries(),
    chooseTotalTries: map.chooseTotalTries(),
    hasRules: map.hasRules(),
});

// Test multiplication safety
const unsafe = isMultiplicationUnsafe(1000000, 1000000);
console.log('Is unsafe:', unsafe);

// Finalize
map.finalize();
```

### TypeScript

The package includes TypeScript definitions automatically:

```typescript
import init, { WasmCrushMap, WasmBucketAlgorithm } from 'mycrush';

async function main() {
    await init();
    
    const map = new WasmCrushMap();
    const tries: number = map.chooseLocalTries();
    console.log('Tries:', tries);
}

main();
```

### With Bundlers (webpack, rollup, etc.)

Install from the bundler build:

```bash
npm install /path/to/mycrush/pkg-bundler
```

```javascript
import { WasmCrushMap, getVersion } from 'mycrush';

const map = new WasmCrushMap();
console.log('Version:', getVersion());
```

## API Reference

### Classes

#### `WasmCrushMap`

Main class for working with CRUSH maps.

**Methods:**
- `constructor()` - Create a new CRUSH map
- `static newLegacy(): WasmCrushMap` - Create a new legacy CRUSH map
- `chooseLocalTries(): number` - Get choose_local_tries parameter
- `chooseLocalFallbackTries(): number` - Get choose_local_fallback_tries parameter
- `chooseTotalTries(): number` - Get choose_total_tries parameter
- `chooseleafDescendOnce(): number` - Get chooseleaf_descend_once parameter
- `chooseleafVaryR(): number` - Get chooseleaf_vary_r parameter
- `chooseleafStable(): number` - Get chooseleaf_stable parameter
- `strawCalcVersion(): number` - Get straw_calc_version parameter
- `allowedBucketAlgs(): number` - Get allowed_bucket_algs parameter
- `hasRules(): boolean` - Check if the map has any rules
- `finalize(): void` - Finalize the map (must be called before using do_rule)
- `findRoots(): Int32Array` - Find root buckets in the map

### Enums

#### `WasmBucketAlgorithm`

Bucket algorithm types:
- `Uniform` (0)
- `List` (1)
- `Tree` (2)
- `Straw` (3)
- `Straw2` (4)

### Functions

#### `getVersion(): string`

Returns the version of the mycrush library.

#### `isMultiplicationUnsafe(a: number, b: number): boolean`

Helper function to check if multiplication would overflow.

## Examples

See the `examples/wasm/` directory for complete examples:
- `index.html` - Browser example with interactive demo
- `node-example.js` - Node.js example

To run the browser example:

```bash
# Serve the examples directory with any static server
cd examples/wasm
python3 -m http.server 8000
# Open http://localhost:8000 in your browser
```

To run the Node.js example:

```bash
cd examples/wasm
node node-example.js
```

## Performance

The WASM build is optimized for performance but does not include `wasm-opt` by default (to avoid network dependencies during build). For production builds, you can enable optimization:

1. Install `wasm-opt` from [binaryen](https://github.com/WebAssembly/binaryen)
2. Edit `Cargo.toml` and change:
   ```toml
   [package.metadata.wasm-pack.profile.release]
   wasm-opt = false
   ```
   to:
   ```toml
   [package.metadata.wasm-pack.profile.release]
   wasm-opt = ["-O4"]
   ```

## Troubleshooting

### Module not found errors

Make sure you've built the correct target for your use case:
- Browser: use `pkg/`
- Node.js: use `pkg-node/`
- Bundlers: use `pkg-bundler/`

### TypeScript errors

The package includes `.d.ts` files automatically. Make sure your TypeScript configuration includes:

```json
{
  "compilerOptions": {
    "moduleResolution": "node",
    "esModuleInterop": true
  }
}
```

## License

Dual-licensed under GPL-3.0-or-later OR LGPL-2.1-or-later

## More Information

For more details about the CRUSH algorithm and mycrush implementation, see the [main README](../README.md).
