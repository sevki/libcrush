// Example usage of mycrush in Node.js
// Make sure to install the package first:
// npm install /path/to/mycrush/pkg-node

import { WasmCrushMap, getVersion, isMultiplicationUnsafe } from 'mycrush';

console.log('🥰 mycrush Node.js Example\n');

// Get version
console.log(`📦 Library version: ${getVersion()}\n`);

// Create a new CRUSH map
const map = new WasmCrushMap();
console.log('🗺️  Created new CRUSH map');

// Get some properties
console.log('\nMap Properties:');
console.log(`  Choose Local Tries: ${map.chooseLocalTries()}`);
console.log(`  Choose Total Tries: ${map.chooseTotalTries()}`);
console.log(`  Chooseleaf Vary R: ${map.chooseleafVaryR()}`);
console.log(`  Has Rules: ${map.hasRules()}`);

// Test multiplication safety check
const a = 1000000;
const b = 1000000;
const unsafe = isMultiplicationUnsafe(a, b);
console.log(`\n🔢 Is ${a} * ${b} unsafe? ${unsafe}`);

// Create a legacy map
const legacyMap = WasmCrushMap.newLegacy();
console.log('\n🗺️  Created legacy CRUSH map');

// Finalize the map
map.finalize();
console.log('✅ Map finalized');

console.log('\n🎉 All operations completed successfully!');
