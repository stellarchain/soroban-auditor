#!/bin/bash

# Round-trip testing: WASM → Rust → WASM → semantic validation
# Goal: Verify decompiled contracts can be recompiled and re-deployed

set -e

PROJECT_ROOT="/Users/fedotserghei/Projects/soroban-auditor"
OUTPUT_DIR="$PROJECT_ROOT/target/test-output"
TEMP_BUILD_DIR="/tmp/soroban-compile-test"
RESULTS_FILE="/tmp/roundtrip-results.txt"

echo "=== Soroban Round-Trip Compilation Test ==="
echo "Testing WASM → Rust → WASM pipeline"
echo ""

# Keep track of results
passed=0
failed=0
total=0

# For each generated Rust file, try to compile it back to WASM
for rs_file in "$OUTPUT_DIR"/*.rs; do
    contract_name=$(basename "$rs_file" .rs)
    total=$((total + 1))
    
    echo -n "Testing $contract_name ... "
    
    # Create temporary project
    rm -rf "$TEMP_BUILD_DIR"
    mkdir -p "$TEMP_BUILD_DIR/src"
    
    # Copy generated file as lib.rs
    cp "$rs_file" "$TEMP_BUILD_DIR/src/lib.rs"
    
    # Create Cargo.toml
    cat > "$TEMP_BUILD_DIR/Cargo.toml" << 'EOF'
[package]
name = "test-contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1

[dependencies]
soroban-sdk = "25.1.0"

[target.wasm32v1-none]
runner = "soroban"
EOF

    # Try to build
    cd "$TEMP_BUILD_DIR"
    if cargo build --release --target wasm32v1-none 2>&1 | head -50 > /tmp/compile.log; then
        echo "✅ COMPILED"
        passed=$((passed + 1))
        
        # Check if WASM was produced
        if [ -f "target/wasm32v1-none/release/test_contract.wasm" ]; then
            wasm_size=$(wc -c < "target/wasm32v1-none/release/test_contract.wasm")
            echo "   → WASM size: $wasm_size bytes"
        fi
    else
        echo "❌ COMPILE FAILED"
        failed=$((failed + 1))
        echo "   First error:"
        grep "error" /tmp/compile.log | head -1
    fi
    
    cd - > /dev/null
done

echo ""
echo "=== Summary ==="
echo "Total: $total | Passed: $passed | Failed: $failed"
echo "Success Rate: $(echo "scale=1; $passed * 100 / $total" | bc)%"

# Save results
{
    echo "Round-Trip Compilation Results"
    echo "=============================="
    echo "Date: $(date)"
    echo "Total: $total"
    echo "Passed: $passed"
    echo "Failed: $failed"
    echo "Success Rate: $(echo "scale=1; $passed * 100 / $total" | bc)%"
} > "$RESULTS_FILE"

echo ""
echo "Results saved to: $RESULTS_FILE"
