# Publishing mycrush to npm

This guide explains how to publish the mycrush WASM package to npm or GitHub Packages.

## Prerequisites

1. You need an npm account. Create one at [npmjs.com](https://www.npmjs.com/signup)
2. Login to npm: `npm login`
3. For GitHub Packages, authenticate: `npm login --registry=https://npm.pkg.github.com`

## Building for Release

First, build the WASM package:

```bash
cd mycrush
./build-wasm.sh
```

This creates three package directories:
- `pkg/` - For web browsers
- `pkg-node/` - For Node.js
- `pkg-bundler/` - For bundlers

## Publishing to npm

You can publish any of the builds. The most common is the bundler build which works in all environments:

```bash
cd pkg-bundler
npm publish
```

Or for the web build:

```bash
cd pkg
npm publish
```

For Node.js:

```bash
cd pkg-node
npm publish
```

### Scoped Packages

If you want to publish under a scope (recommended):

```bash
# Edit package.json and change "name" to "@yourusername/mycrush"
cd pkg
npm publish --access public
```

## Publishing to GitHub Packages

To publish to GitHub Packages instead of npm:

1. Create a `.npmrc` file in the package directory:

```bash
cd pkg
cat > .npmrc << 'EOF'
@sevki:registry=https://npm.pkg.github.com
EOF
```

2. Update `package.json` to use scoped name:

```json
{
  "name": "@sevki/mycrush",
  ...
}
```

3. Publish:

```bash
npm publish --registry=https://npm.pkg.github.com
```

## Automating Releases

You can add these commands to your CI/CD pipeline. Example GitHub Actions workflow:

```yaml
name: Publish WASM Package

on:
  release:
    types: [created]

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
      
      - name: Install wasm-pack
        run: cargo install wasm-pack
      
      - name: Build WASM
        run: |
          cd mycrush
          ./build-wasm.sh
      
      - name: Publish to npm
        run: |
          cd mycrush/pkg-bundler
          echo "//registry.npmjs.org/:_authToken=${{ secrets.NPM_TOKEN }}" > .npmrc
          npm publish
```

## Version Management

The package version is controlled by `Cargo.toml`. When you update the version there:

```toml
[package]
version = "0.2.0"
```

The generated `package.json` will automatically use that version.

## Testing the Package

Before publishing, test the package locally:

```bash
cd pkg-bundler
npm pack
# This creates a .tgz file

# Install it in a test project
cd /path/to/test-project
npm install /path/to/mycrush/pkg-bundler/mycrush-0.1.2.tgz
```

## Package Contents

Each published package includes:
- `*.wasm` - The compiled WebAssembly binary
- `*.js` - JavaScript glue code
- `*.d.ts` - TypeScript type definitions
- `package.json` - Package metadata
- `README.md` - Documentation

## Post-Publishing

After publishing, users can install with:

```bash
npm install mycrush
```

Or for scoped packages:

```bash
npm install @sevki/mycrush
```

Then use it:

```javascript
import { WasmCrushMap, getVersion } from 'mycrush';
```

## Updating the Package

To publish a new version:

1. Update version in `mycrush/Cargo.toml`
2. Rebuild: `./build-wasm.sh`
3. Publish: `cd pkg-bundler && npm publish`

## Troubleshooting

### Permission Denied

If you get permission errors, make sure you're logged in:
```bash
npm login
```

### Package Already Exists

If the unscoped `mycrush` name is taken, use a scoped package:
```bash
# In package.json, change to:
"name": "@yourusername/mycrush"
```

### 402 Payment Required

This usually means the package name is reserved or requires a paid account. Use a scoped package instead.
