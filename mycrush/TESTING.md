# Quick Testing Guide for mycrush WASM Package

This guide helps you verify the WASM package works correctly before publishing to npm.

## Testing Web Build

### Method 1: Simple HTTP Server

```bash
cd mycrush/examples/wasm

# Using Python
python3 -m http.server 8000

# Or using Node.js
npx http-server

# Open http://localhost:8000 in your browser
```

The browser console should show:
- ✅ WASM module loaded successfully
- Library version
- Map properties
- All operations completed successfully

### Method 2: Quick HTML Test

Create a test file:

```bash
cd mycrush/pkg

cat > test.html << 'EOF'
<!DOCTYPE html>
<html>
<head><title>Test</title></head>
<body>
<script type="module">
import init, { WasmCrushMap, getVersion } from './mycrush.js';
await init();
console.log('Version:', getVersion());
const map = new WasmCrushMap();
console.log('Tries:', map.chooseLocalTries());
console.log('✅ All working!');
</script>
</body>
</html>
EOF

python3 -m http.server 8000
# Open http://localhost:8000/test.html
```

## Testing Node.js Build

### Quick Test

```bash
cd mycrush/pkg-node

# Create test file
cat > test.mjs << 'EOF'
import { WasmCrushMap, getVersion } from './mycrush.js';

console.log('Version:', getVersion());
const map = new WasmCrushMap();
console.log('Choose Local Tries:', map.chooseLocalTries());
console.log('Has Rules:', map.hasRules());
map.finalize();
console.log('✅ All tests passed!');
EOF

# Run test
node test.mjs
```

Expected output:
```
Version: 0.1.2
Choose Local Tries: 5
Has Rules: false
✅ All tests passed!
```

### Test with npm pack

```bash
cd mycrush/pkg-node

# Create a tarball
npm pack

# In a test directory
mkdir /tmp/test-mycrush
cd /tmp/test-mycrush
npm init -y

# Install the local package
npm install /home/runner/work/libcrush/libcrush/mycrush/pkg-node/mycrush-0.1.2.tgz

# Create test
cat > test.mjs << 'EOF'
import { WasmCrushMap, getVersion } from 'mycrush';
console.log('Installed version:', getVersion());
const map = new WasmCrushMap();
console.log('✅ Package installed and working!');
EOF

node test.mjs
```

## Testing Bundler Build

### With webpack

```bash
mkdir /tmp/test-webpack
cd /tmp/test-webpack

npm init -y
npm install webpack webpack-cli --save-dev
npm install /home/runner/work/libcrush/libcrush/mycrush/pkg-bundler/mycrush-0.1.2.tgz

# Create src/index.js
mkdir src
cat > src/index.js << 'EOF'
import { WasmCrushMap, getVersion } from 'mycrush';

console.log('Version:', getVersion());
const map = new WasmCrushMap();
console.log('Map created successfully');
EOF

# Add to package.json scripts:
# "build": "webpack"

npx webpack
node dist/main.js
```

## TypeScript Testing

```bash
cd /tmp/test-typescript
npm init -y
npm install typescript --save-dev
npm install /home/runner/work/libcrush/libcrush/mycrush/pkg-bundler/mycrush-0.1.2.tgz

# Create tsconfig.json
cat > tsconfig.json << 'EOF'
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ES2020",
    "moduleResolution": "node",
    "esModuleInterop": true,
    "strict": true
  }
}
EOF

# Create test.ts
cat > test.ts << 'EOF'
import { WasmCrushMap, getVersion, WasmBucketAlgorithm } from 'mycrush';

const version: string = getVersion();
console.log('Version:', version);

const map = new WasmCrushMap();
const tries: number = map.chooseLocalTries();
console.log('Tries:', tries);

const alg: WasmBucketAlgorithm = WasmBucketAlgorithm.Straw2;
console.log('Algorithm:', alg);
EOF

npx tsc test.ts
```

## Verification Checklist

Before publishing, verify:

- [ ] Web build loads in browser
- [ ] Node.js build works with `node test.mjs`
- [ ] Bundler build works with webpack/rollup
- [ ] TypeScript definitions compile without errors
- [ ] Package.json has correct metadata
- [ ] README.md is included in package
- [ ] WASM file size is reasonable (~32KB)
- [ ] All exported functions work correctly
- [ ] No console errors or warnings

## Expected Results

All tests should show:
- ✅ Version number displayed correctly
- ✅ WasmCrushMap can be instantiated
- ✅ Methods return expected values
- ✅ No runtime errors
- ✅ TypeScript types work correctly

## Common Issues

### "Module not found"
- Make sure you're using the right build (web/node/bundler)
- Check import paths are correct

### "WebAssembly module is not defined"
- For browsers, must use a web server (not file://)
- Check Content-Type headers for .wasm files

### "Cannot find module 'mycrush'"
- Verify installation: `npm list mycrush`
- Check node_modules/mycrush/ exists

## Automated Testing

You can add this to your CI:

```yaml
- name: Test WASM Package
  run: |
    cd mycrush/pkg-node
    cat > test.mjs << 'EOF'
    import { getVersion } from './mycrush.js';
    const version = getVersion();
    if (!version) {
      console.error('❌ Version not found');
      process.exit(1);
    }
    console.log('✅ Package works, version:', version);
    EOF
    node test.mjs
```

## Ready to Publish?

Once all tests pass, you can publish:

```bash
cd mycrush/pkg-bundler
npm publish --dry-run  # Test run
npm publish           # Real publish
```

Or use the GitHub Actions workflow for automated publishing.
