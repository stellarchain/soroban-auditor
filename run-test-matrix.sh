#!/bin/bash

# Test Matrix Runner for Soroban Auditor
# Runs decompilation on all contracts and reports results

set -e

PROJECT_ROOT="/Users/fedotserghei/Projects/soroban-auditor"
WASM_DIR="$PROJECT_ROOT/playground/playground/wasm"
OUTPUT_DIR="$PROJECT_ROOT/target/test-output"
BINARY="$PROJECT_ROOT/target/release/soroban-auditor"

mkdir -p "$OUTPUT_DIR"

echo "=== Soroban Auditor Test Matrix ==="
echo "Building project..."
cd "$PROJECT_ROOT"
cargo build --release 2>&1 | grep -E "^(Compiling|Finished|error)" || true

echo ""
echo "=== Running Decompilation Tests ==="
echo "Format: CONTRACT | Size | Syntax Check | Duration"
echo "---"

total_tests=0
passed_tests=0
failed_tests=0

for wasm_file in "$WASM_DIR"/*.wasm; do
    if [ -f "$wasm_file" ]; then
        contract_name=$(basename "$wasm_file" .wasm)
        output_file="$OUTPUT_DIR/$contract_name.rs"
        
        total_tests=$((total_tests + 1))
        
        # Run with timer
        start_time=$(date +%s%N)
        if "$BINARY" "$wasm_file" "$output_file" 2>/dev/null; then
            end_time=$(date +%s%N)
            duration_ms=$(( ($end_time - $start_time) / 1000000 ))
            
            # Check syntax with rustfmt --check
            if rustfmt --check "$output_file" 2>/dev/null; then
                echo "✓ $contract_name | $(wc -l < "$output_file") lines | OK | ${duration_ms}ms"
                passed_tests=$((passed_tests + 1))
            else
                echo "⚠ $contract_name | $(wc -l < "$output_file") lines | SYNTAX ERRORS | ${duration_ms}ms"
                # Show first few errors
                rustfmt --check "$output_file" 2>&1 | head -3 | sed 's/^/   /'
                failed_tests=$((failed_tests + 1))
            fi
        else
            echo "✗ $contract_name | N/A | DECOMPILE FAILED | N/A"
            failed_tests=$((failed_tests + 1))
        fi
    fi
done

echo ""
echo "=== Summary ==="
echo "Total: $total_tests | Passed: $passed_tests | Failed: $failed_tests"
echo "Success Rate: $(echo "scale=1; $passed_tests * 100 / $total_tests" | bc)%"
echo ""
echo "Output files saved to: $OUTPUT_DIR"
