# Reverse Engineering Plan - Source-Like Decompilation (No Hardcoding!)

## 🎯 Goal
Transform unreadable WASM decompilation into source-like code through **intelligent pattern recognition**, NOT hardcoding.

## 📊 Current State: Groth16 Example
- **Original:** 68 lines, clean algorithm
- **Decompiled:** 494 lines, complete spaghetti
- **Target:** ~100-150 lines, recognizable algorithm

---

## 🔬 Phase 1: Helper Function Analysis & Inlining (CRITICAL)

### Problem
```rust
// Current: Opaque helper calls
self.func17(env, vk, 1048600, 5, var3.wrapping_add(8), 5);
self.func15(env, var3.wrapping_add(96), slot_var3_8_i64);
```

### Solution: Automatic Helper Recognition
1. **Fingerprint-based detection**
   - Analyze func15, func17, func18 bytecode patterns
   - Detect: "tiny function that just calls 1 import"
   - Pattern: `GetLocal(0), GetLocal(1), ..., Call(import_idx), End`

2. **Inline small forwarders automatically**
   ```rust
   fn is_simple_forwarder(func_body) -> Option<(import_name, arg_mapping)> {
       if func_body.len() < 10 instructions &&
          ends_with_single_call_to_import() {
           return Some(extract_forwarding_info());
       }
   }
   ```

3. **Build call graph**
   - func17 → calls "map_unpack_to_linear_memory"
   - func15 → calls "bytes_copy_from_linear_memory"
   - Inline these automatically during decompilation

**No hardcoding!** Just pattern: "if function is tiny + calls 1 SDK function → inline it"

---

## 🔬 Phase 2: SDK Call Recognition & High-Level Naming

### Problem
```rust
// Current: Low-level SDK calls
self.bls12_381_g1_mul(...)  // Hidden in func17
```

### Solution: SDK Call Mapping
1. **Use existing SdkFunctionDetector**
   - Already have 525 SDK functions mapped
   - Already detecting calls in scan_body

2. **Map to high-level names**
   ```rust
   fn high_level_name_for_sdk_call(sdk_fn: &str, context: &CallContext) -> String {
       match sdk_fn {
           "bls12_381_g1_mul" => "bls.g1_mul",
           "bls12_381_g1_add" => "bls.g1_add",
           "bls12_381_multi_pairing_check" => "bls.pairing_check",
           "vec_len" => ".len()",
           "vec_get" => ".get()",
           _ => sdk_fn  // fallback
       }
   }
   ```

3. **Context-aware naming**
   - If calling `vec_len` on variable X → generate `X.len()`
   - If calling `bls12_381_g1_mul` → generate `bls.g1_mul(&a, &b)`
   - **Pattern-based, not hardcoded per contract!**

---

## 🔬 Phase 3: Control Flow Reconstruction

### Problem
```rust
// Current: Nested loop hell
let mut __exit_label4: i32 = 0;
loop {
    loop {
        if (var4 as u32 >= var14 as u32) as i32 != 0 {
            break;
        }
        // ...
    }
}
```

### Solution: Iterator Pattern Detection
1. **Detect counted loops**
   ```rust
   Pattern:
   - var_i = 0
   - loop { if var_i >= var_len { break } ... var_i += 1 }

   → Transform to: for i in 0..len
   ```

2. **Detect vector iteration**
   ```rust
   Pattern:
   - loop with vec_get(i)
   - incrementing counter
   - bounds check against vec_len

   → Transform to: for elem in vec.iter()
   ```

3. **Detect zip patterns**
   ```rust
   Pattern:
   - Two parallel iterations
   - Same counter for both

   → Transform to: iter1.zip(iter2)
   ```

**Algorithm:**
- Analyze loop structure
- Find iterator variable
- Check if it's used for array access
- Reconstruct high-level iterator

---

## 🔬 Phase 4: Type Inference & Conversion Elimination

### Problem
```rust
// Current: Conversion hell
let var34 = val_to_i64(Vec::<Val>::from_val(env,
    &val_from_i64(slot_var3_8_i64))
    .get_unchecked(...));
```

### Solution: Type Propagation
1. **Track types through SSA**
   ```rust
   struct TypeInfo {
       base_type: String,  // Vec, Map, Address, etc.
       generic_args: Vec<String>,
       is_val: bool,
   }

   fn propagate_types(block: &FunctionBlock) -> HashMap<String, TypeInfo> {
       // Track variable assignments
       // Infer types from SDK calls
       // Eliminate redundant conversions
   }
   ```

2. **Detect conversion chains**
   ```rust
   Pattern: val_to_i64(val_from_i64(x)) → x
   Pattern: Vec::from_val(env, &val_from_i64(x)) where x: Vec → x
   ```

3. **Generate proper types**
   ```rust
   // Instead of: let var5: i64 = ...;
   // Generate:   let vec_items: Vec<G1Affine> = ...;
   ```

---

## 🔬 Phase 5: Semantic Variable Naming

### Problem
```rust
// Current: Generic names
let mut var5: i64 = 0;
let mut var6: i64 = 0;
```

### Solution: Context-Based Naming
1. **Analyze usage patterns**
   ```rust
   fn infer_variable_name(var: &str, usages: &[Usage]) -> String {
       // Check what SDK functions are called with this variable
       for usage in usages {
           if usage.is_sdk_call("vec_len") {
               return format!("{}_len", var);
           }
           if usage.is_sdk_call("bls12_381_g1_mul") && usage.arg_index == 0 {
               return "point".to_string();
           }
           if usage.is_sdk_call("bls12_381_g1_mul") && usage.arg_index == 1 {
               return "scalar".to_string();
           }
       }
   }
   ```

2. **Use parameter names from contract spec**
   ```rust
   // Function signature says: verify_proof(vk: VerificationKey, ...)
   // If var5 comes from vk.ic → name it "ic_element" or "vk_point"
   ```

3. **Crypto-specific heuristics**
   ```rust
   fn crypto_variable_name(usage_pattern: &str) -> Option<String> {
       match usage_pattern {
           "g1_mul_result" => Some("prod"),
           "g1_add_accumulator" => Some("sum"),
           "pairing_check_input" => Some("pairing_points"),
           _ => None
       }
   }
   ```

---

## 🔬 Phase 6: Memory Operation Elimination

### Problem
```rust
// Current: Manual memory management
var3 = var17.wrapping_sub(192);
mstore64!(var3.wrapping_add(8) as usize, value);
let x = mload64!(var3 as usize + 16);
```

### Solution: Stack-to-Variable Transformation
1. **Detect stack frame allocation**
   ```rust
   Pattern:
   - var_frame = global0.wrapping_sub(N)
   - mstore at var_frame + offset
   - mload from var_frame + offset

   → Replace with local variables
   ```

2. **Map slots to semantics**
   ```rust
   // Instead of: slot_var3_8_i64
   // Analyze what's stored there:
   //   - If result of vec_new → "new_vec"
   //   - If result of g1_mul → "product"
   //   - If used for error code → "status"
   ```

3. **Eliminate frame completely**
   ```rust
   // Pattern already exists in StackFramePattern
   // Enhance it to track semantic meaning of slots
   ```

---

## 🔬 Phase 7: Expression Simplification

### Problem
```rust
// Current: Verbose expressions
if (var4 as u32 >= var14 as u32) as i32 != 0 {
if ((var29 as u64) < 4294967296 as u64) as i32 != 0 {
```

### Solution: Boolean Optimization
1. **Simplify boolean tests**
   ```rust
   Pattern: (expr) as i32 != 0 → expr
   Pattern: !(expr) as i32 != 0 → !expr
   ```

2. **Simplify comparisons**
   ```rust
   Pattern: (a as u32 >= b as u32) as i32 != 0 → a >= b
   ```

3. **Constant folding**
   ```rust
   Pattern: x.wrapping_shl(32) | 0 → x.wrapping_shl(32)
   ```

---

## 🎯 Implementation Order (By Impact)

### Week 1: Foundation (80% improvement)
1. ✅ **Helper function inlining** (DONE: internal_call_forwarders exists)
   - Enhance to inline ALL simple forwarders
   - Build complete call graph

2. ✅ **SDK call recognition** (DONE: SdkFunctionDetector exists)
   - Map SDK calls to high-level names
   - Generate proper method syntax

3. ✅ **Stack frame elimination** (DONE: StackFramePattern exists)
   - Enhance to track slot semantics
   - Complete memory operation removal

### Week 2: Control Flow (15% improvement)
4. ⏳ **Iterator pattern detection**
   - Detect counted loops → for i in 0..n
   - Detect vec iteration → for elem in vec.iter()
   - Detect zip patterns

5. ⏳ **Exit label elimination**
   - Better loop reconstruction
   - Proper break/continue

### Week 3: Naming & Types (5% improvement)
6. ⏳ **Type propagation**
   - Track types through variables
   - Eliminate conversions

7. ⏳ **Semantic variable naming**
   - Usage-based naming
   - Crypto-specific heuristics

---

## 🔧 Implementation Approach

### 1. Analysis Phase
```rust
// src/engine/analyzer/mod.rs
pub struct FunctionAnalyzer {
    sdk_detector: SdkFunctionDetector,
    call_graph: CallGraph,
    type_tracker: TypeTracker,
}

impl FunctionAnalyzer {
    pub fn analyze(&self, func: &Function) -> FunctionAnalysis {
        FunctionAnalysis {
            sdk_calls: self.detect_sdk_calls(func),
            forwarders: self.find_forwarders(func),
            loop_patterns: self.detect_loops(func),
            type_flow: self.infer_types(func),
        }
    }
}
```

### 2. Transformation Phase
```rust
// src/engine/transform/mod.rs
pub struct CodeTransformer {
    analyzer: FunctionAnalyzer,
}

impl CodeTransformer {
    pub fn transform(&self, code: String) -> String {
        let mut result = code;

        // Apply transformations in order
        result = self.inline_forwarders(result);
        result = self.map_sdk_calls(result);
        result = self.eliminate_stack_frames(result);
        result = self.reconstruct_iterators(result);
        result = self.rename_variables(result);
        result = self.simplify_expressions(result);

        result
    }
}
```

### 3. Pattern Library
```rust
// All patterns are generic, data-driven
pub struct PatternLibrary {
    forwarder_patterns: Vec<ForwarderPattern>,
    loop_patterns: Vec<LoopPattern>,
    sdk_mappings: HashMap<String, SdkMapping>,
    naming_heuristics: Vec<NamingHeuristic>,
}
```

---

## 📊 Success Metrics

### Groth16 Target
- Lines: 494 → **~120** (75% reduction)
- Generic vars: 16 → **~5** (69% reduction)
- Helper calls: 50 → **0** (100% elimination)
- Nested loops: 5 levels → **1-2 levels** (60% reduction)
- Readability: 2/10 → **7/10** (250% improvement)

### General Targets
- **80%** of contracts should have recognizable structure
- **90%** of SDK calls should be high-level named
- **95%** of helper functions should be inlined
- **70%** of loops should be iterator-style

---

## 🚀 Next Steps

1. **Enhance internal_call_forwarders** to inline ALL forwarders
2. **Create SdkCallMapper** for high-level SDK naming
3. **Build IteratorPatternDetector** for loop reconstruction
4. **Implement TypePropagationEngine** for better types
5. **Create SemanticNamer** for variable naming

All generic, pattern-based, zero hardcoding! 🎯
