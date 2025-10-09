# Changelog

## [1.0.0](https://github.com/sevki/libcrush/compare/v0.1.2...v1.0.0) (2025-10-09)


### ⚠ BREAKING CHANGES

* **mycrush:** Functions no longer expose C ABI. Code depending on C compatibility will need updates.

### Features

* wasm bindings ([f29f155](https://github.com/sevki/libcrush/commit/f29f155c31ee7060fc754b37cded74d5ec430f4a))


### Bug Fixes

* missing instructions for GH Agents ([aae4cf0](https://github.com/sevki/libcrush/commit/aae4cf03f50aaf3ff0823502015845f2f5491ba4))
* **mycrush:** remove unnecessary unsafe blocks in wrapper.rs ([31d0315](https://github.com/sevki/libcrush/commit/31d0315794d42b6897c85991ebd4121c7a3a25a0))


### Code Refactoring

* **mycrush:** convert to pure Rust implementation without C ABI ([a4c821f](https://github.com/sevki/libcrush/commit/a4c821f469f82e9d5a550744f911c1132326fc20))

## [0.1.2](https://github.com/sevki/libcrush/compare/v0.1.1...v0.1.2) (2025-10-08)


### Bug Fixes

* lots of clippy lints ([301c2e3](https://github.com/sevki/libcrush/commit/301c2e3210b969a4c2c0f239bb0b84f4ed56957b))

## [0.1.1](https://github.com/sevki/libcrush/compare/v0.1.0...v0.1.1) (2025-10-08)


### Bug Fixes

* lint errors and rename things to match rust naming ([8766d18](https://github.com/sevki/libcrush/commit/8766d18bf94285b5a00afaa9d531d1fabf9aedb6))
* packaging ([cf0dfce](https://github.com/sevki/libcrush/commit/cf0dfce771990a1a26e50fa790d56f741e3e6faf))

## 0.1.0 (2025-10-07)


### Features

* pure rust implementation of libcrush translated by c2rust ([403d841](https://github.com/sevki/libcrush/commit/403d841c21e86f2a6c7702ae3e49a1ddde4b6836))
* rust bindings ([5735d00](https://github.com/sevki/libcrush/commit/5735d00bf31e23a0f91f89ef417882a62d9cc3ab))
* rust bindings ([6b302e4](https://github.com/sevki/libcrush/commit/6b302e420b8381306c44de62abba01ed1e537296))


### Bug Fixes

* release please manifest ([206f3b9](https://github.com/sevki/libcrush/commit/206f3b945b6e35f6029f81942b38bfc249863b75))
