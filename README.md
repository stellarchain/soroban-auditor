# Soroban Auditor

**soroban-auditor** is a decompiler for WebAssembly Smart Contracts binaries deployed on the Stellar ledger. It can decompile WASM binaries from the MVP version 1; however, it's still under development, and some features, such as proper type recovery, are still missing.

## Building from source
Building or installing **soroban-auditor** from source requires a working Rust Installation (probably at least version 1.37.0).

## Features
- **Tlsh**: This package provides functionality for calculating the Trend Micro Locality Sensitive Hash (TLSH) of input data.
- **Lcs**: The Longest Common Subsequence (LCS) package enables finding the longest subsequence present in given sequences.
- **levenshtein**: The Levenshtein package offers tools for computing the Levenshtein distance between two strings.

```
cargo build 
./soroban-auditor tests/test_contract.wasm //this will generate rust file with disassembled code.
```


### Example

```
fn func40<I: Imports<Memory = M>>(
    &mut self, 
    imports: &mut I, 
    mut var0: i64
) -> i64 {
        let mut var1: i64 = 0;
        if (var0 & 255i64 != 77i64) as i32 != 0 {
            unreachable!();
        }
        let var2 = self.func22(imports);
        var1 = var2;
        let var3 = imports._0(self, var1);
        var3;
        self.func38(imports);
        self.func24(imports, var0);
        let var4 = self.func41(imports, 4083516257707209486i64, var1);
        let var5 = imports._1(self, var4, var0);
        var5;
        2i64
}
```

## Developers
[stellarchain.io](https://stellarchain.io)

## Development Plan
https://stellarchain.notion.site/Stellar-Soroban-Smart-Contract-Audit-235524ad585a4183a7e0f0025cb18abf
