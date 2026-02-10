# Soroban SDK Function Detection

## Overview

The soroban-auditor now includes comprehensive Soroban SDK function detection and analysis capabilities. This system automatically detects and reports usage of all 525+ SDK functions defined in the Soroban environment.

## Components

### 1. SDK Function Detector (`src/sdk/detector.rs`)

The core detection engine that:
- Loads all SDK function definitions from `common_env_soroban.json`
- Provides lookup by function name or export identifier
- Categorizes functions by module (ledger, context, vec, buf, crypto, etc.)
- Categorizes functions by high-level purpose (Ledger, Context, Collections, Math, etc.)

**Key APIs:**
```rust
let detector = SdkFunctionDetector::default();

// Check if a function is an SDK function
if detector.is_sdk_function("vec_new") { ... }

// Get function information
if let Some(info) = detector.get_by_name("require_auth") {
    println!("Module: {}", info.module_name);
    println!("Docs: {}", info.docs);
}

// Get all storage-related functions
let storage_funcs = detector.get_storage_functions();
```

### 2. SDK Usage Analyzer (`src/sdk/analyzer.rs`)

Analyzes SDK usage patterns and generates reports:
- Tracks total calls and unique functions used
- Groups by module and category
- Identifies key metrics (auth checks, storage operations, events)
- Generates human-readable and compact reports

**Key APIs:**
```rust
let analyzer = SdkUsageAnalyzer::default();
let report = analyzer.analyze(&sdk_calls);
println!("{}", analyzer.format_report(&report));
```

### 3. SDK Analyze Tool (`sdk-analyze` binary)

A standalone tool for analyzing WASM contracts:

```bash
# Basic analysis
./target/release/sdk-analyze tests/another_random.wasm

# Detailed per-function breakdown
./target/release/sdk-analyze tests/another_random.wasm --detailed
```

**Output includes:**
- Total SDK calls and unique functions
- Calls by module (vec, buf, ledger, crypto, etc.)
- Calls by category (Collections, Ledger, Math, etc.)
- Key metrics (auth, storage, events, collections)
- Top 20 most used SDK functions with documentation
- Per-function breakdown (with --detailed)

## Example Output

```
=== Soroban SDK Usage Report ===

Total SDK calls: 56
Unique SDK functions: 32

Calls by Module:
  vec: 21 calls
  buf: 11 calls
  ledger: 9 calls
  int: 4 calls
  context: 3 calls
  crypto: 3 calls
  address: 3 calls
  map: 2 calls

Calls by Category:
  Collections: 34 calls
  Ledger: 9 calls
  Math: 4 calls
  Context: 3 calls
  Other: 3 calls
  Address: 3 calls

Key Metrics:
  Authentication checks: 2
  Storage reads: 3
  Storage writes: 3
  Event emissions: 1
  Vec operations: 21
  Map operations: 2

Top 20 Most Used SDK Functions:
  vec_len                                  (vec            ) - 7 calls
    └─ Returns length of the vector.
  vec_new                                  (vec            ) - 5 calls
  vec_push_back                            (vec            ) - 5 calls
  ...
```

## Integration with Decompiler

The SDK detection system is integrated into the decompilation process:

1. **BodyUsage Tracking**: The `BodyUsage` struct now includes an `sdk_function_calls` field that tracks all SDK function calls detected during scanning.

2. **Comprehensive Detection**: Instead of hardcoding ~40 SDK functions, the system now automatically detects all 525+ functions from the JSON database.

3. **Future Enhancement**: The SDK usage data can be used to:
   - Generate better function names based on SDK usage patterns
   - Optimize code generation by recognizing SDK patterns
   - Add inline documentation from SDK function docs
   - Detect common contract patterns (token, NFT, DeFi, etc.)

## SDK Function Database

The system uses `/src/soroban/common_env_soroban.json` which contains:
- **Modules**: context, ledger, buf, vec, map, int, crypto, address, call, test, prng
- **525+ Functions**: All Soroban host functions with full metadata
- **Metadata**: Function names, exports, arguments, return types, documentation

Example entry:
```json
{
  "export": "1",
  "name": "contract_event",
  "args": [
    {"name": "topics", "type": "VecObject"},
    {"name": "data", "type": "Val"}
  ],
  "return": "Void",
  "docs": "Records a contract event. topics is expected to be a SCVec..."
}
```

## Use Cases

### 1. Security Auditing
- Identify missing authorization checks
- Detect excessive storage operations
- Find unprotected admin functions

### 2. Contract Analysis
- Understand contract complexity
- Identify performance bottlenecks
- Compare similar contracts

### 3. Code Quality
- Ensure proper SDK usage
- Detect anti-patterns
- Verify best practices

### 4. Documentation
- Auto-generate SDK usage summaries
- Document external dependencies
- Track API evolution

## Future Enhancements

1. **Pattern Recognition**: Detect common contract patterns (SEP-41 token, NFT, etc.)
2. **Optimization Suggestions**: Recommend more efficient SDK usage
3. **Gas Estimation**: Estimate gas costs based on SDK function usage
4. **Security Warnings**: Flag potentially unsafe SDK usage patterns
5. **Contract Fingerprinting**: Create signatures based on SDK usage
6. **Differential Analysis**: Compare SDK usage between contract versions

## Technical Details

### Performance
- SDK detector initialized once per analysis (lazy static)
- JSON parsing happens at startup
- HashMap lookups for O(1) function detection
- Minimal overhead during WASM scanning

### Accuracy
- 100% coverage of official Soroban SDK functions
- Automatic updates from `common_env_soroban.json`
- No false positives (only detects actual SDK calls)
- No false negatives (comprehensive function database)

### Extensibility
- Easy to add new analysis categories
- Simple to extend with custom metrics
- Pluggable report formats
- Modular architecture

## Building and Testing

```bash
# Build both binaries
cargo build --release

# Test the analyzer
./target/release/sdk-analyze tests/soroban_token_contract.optimized.wasm

# Run with detailed output
./target/release/sdk-analyze tests/another_random.wasm --detailed

# Analyze all test contracts
for wasm in tests/*.wasm; do
    echo "=== $wasm ==="
    ./target/release/sdk-analyze "$wasm"
    echo
done
```

## Conclusion

The SDK detection system provides comprehensive visibility into how Soroban contracts use the SDK, enabling better analysis, optimization, and security auditing of smart contracts.
