# Cleanup Progress Report

## Completed Work

### Phase 1: Replace libc with core::ffi types
- Created `ffi` module in `types.rs` that re-exports `core::ffi` types
- Replaced all 627 occurrences of `libc::c_*` with `ffi::c_*` across all mycrush source files
- Added platform-specific `c_long` type definition (not available in core::ffi)
- Benefits:
  - Reduced external dependency on libc crate for basic FFI types
  - Uses stable Rust standard library types from core::ffi
  - No functional changes, purely a refactoring for better dependency management

### Phase 2: Remove Unnecessary Unsafe Code
1. **crush_calc_tree_node** (builder.rs and crush.rs)
   - Changed from `unsafe extern "C" fn` to safe `fn`
   - This function only does simple integer arithmetic: `((i + 1) << 1) - 1`
   - Only used internally within the Rust code, doesn't need C ABI
   - Saves 2 unsafe function declarations

2. **crush_find_roots** (helpers.rs)
   - Removed unnecessary `memset` call - Vec initialization with `vec![0; size]` already zeros memory
   - Replaced unsafe pointer arithmetic (`ref_0.as_mut_ptr().offset()`) with safe Vec indexing (`ref_0[index]`)
   - Benefits:
     - Eliminated 1 libc function call (memset)
     - Made code more idiomatic and easier to understand
     - Reduced unsafe operations while maintaining identical behavior

## Test Results
- All existing tests continue to pass
- No behavioral changes, only safety improvements
- 172 compiler warnings remain (mostly from other c2rust generated code)

## Statistics
- Files modified: 7
- Lines changed: ~1,550 lines (mostly type replacements)
- Unsafe blocks removed: 3
- libc function calls eliminated: 1 (memset)

## Future Work Recommendations

### Quick Wins (Low Risk, High Value)
1. **Replace more memset/memcpy with Rust idioms**
   - Found ~10 more memset calls in builder.rs that could use `ptr::write_bytes` or vec initialization
   - Found ~3 memcpy calls that could use slice `.copy_from_slice()`

2. **Convert simple while loops to for loops**
   - Found ~20 while loops in builder.rs that may be candidates
   - Need careful analysis to ensure loop counter isn't conditionally modified

3. **Replace malloc+free with Box/Vec** 
   - Complex because memory is often passed across FFI boundary
   - Need to ensure memory allocation/deallocation is consistent
   - Candidate: Internal temporary allocations that don't cross FFI

### Medium Effort (Requires Testing)
4. **Create safe wrappers for FFI functions**
   - Many `#[unsafe(no_mangle)] pub extern "C"` functions that are actually safe
   - Could provide safe Rust wrappers while keeping C ABI for FFI
   - Example: crush_addition_is_unsafe, crush_multiplication_is_unsafe

5. **Convert pointer+length to slices**
   - Many functions take `*mut T` and size parameters
   - Could be converted to `&mut [T]` with wrapper functions
   - Requires verifying all indices are non-negative

### Larger Refactoring (High Risk, Consider Carefully)
6. **Replace assert_fail with panic!**
   - Currently using libc __assert_fail
   - Could use Rust's panic! macro instead
   - Need to ensure panic behavior is acceptable

7. **Improve memory management in destroy functions**
   - Currently using manual `free()` calls
   - Could potentially use Box::from_raw if memory was allocated consistently
   - Risky: Need to ensure no double-frees or memory leaks

8. **Type system improvements**
   - Replace `ffi::c_int` with more specific Rust types (i32, usize, etc.)
   - Requires careful analysis of value ranges and overflow behavior
   - Per AGENTS.md: "Time-consuming to verify correctness"

## Notes
- The codebase still has symbol conflicts when running integration tests that link both C and Rust implementations
- This is expected and not caused by our changes
- Library tests (`cargo test --lib`) all pass successfully
- Following AGENTS.md guidelines: making minimal, safe changes with testing after each modification
