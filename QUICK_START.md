# Quick Start - Implementare Pattern-uri Noi

## Ce am adăugat deja ✅

### 3 Pattern-uri Noi în Engine:
1. **StorageAccessPattern** - simplifică accesul la storage
2. **MathOperationsPattern** - simplifică operații matematice (wrapping_add, etc.)
3. **VariableNamingPattern** - îmbunătățește numele variabilelor (var1 → address, etc.)

### Sistem de Template-uri:
- **templates.rs** - sistem declarativ pentru generare de funcții
- Template-uri predefinite pentru: balance, transfer, mint, burn, approve, etc.

## Cum să compilezi și testezi

```bash
# 1. Build
cargo build --release

# 2. Testează pe un contract simplu
./target/release/soroban-auditor tests/soroban_token_contract.optimized.wasm /tmp/test_output.rs

# 3. Compară cu output-ul anterior
diff tests/soroban_token_contract.rs /tmp/test_output.rs

# 4. Verifică îmbunătățirile
cat /tmp/test_output.rs | grep -A 5 "pub fn"
```

## Structura Proiectului

```
src/
├── engine/
│   ├── patterns/          # Pattern-uri de transformare AST
│   │   ├── storage_access.rs       ← NOU
│   │   ├── math_operations.rs      ← NOU
│   │   ├── variable_naming.rs      ← NOU
│   │   ├── for_each_val.rs
│   │   ├── loop_to_while.rs
│   │   └── ... (~25 pattern-uri control flow)
│   ├── pipeline.rs        # Înregistrarea pattern-urilor
│   ├── pattern.rs         # Trait Pattern
│   └── ir.rs              # AST representation
├── patterns/
│   ├── templates.rs       # Sistem de template-uri ← NOU
│   ├── token.rs           # Token operations (de migrat la templates)
│   ├── storage.rs         # Storage operations (de migrat la templates)
│   ├── swap.rs            # Swap operations
│   └── mod.rs
└── ...
```

## Workflow de Development

### 1. Adaugă un Pattern Nou în Engine

**Exemplu**: Pattern pentru Map operations

```bash
# Creează fișierul
touch src/engine/patterns/map_operations.rs
```

```rust
// src/engine/patterns/map_operations.rs
use crate::engine::function::FunctionBlock;
use crate::engine::ir::{flatten_nodes, parse_lines, Node};
use crate::engine::pattern::Pattern;

pub struct MapOperationsPattern;

impl MapOperationsPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for MapOperationsPattern {
    fn name(&self) -> &'static str {
        "map_operations"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        // Implementează logica de transformare
        // ...
        None
    }
}
```

**Înregistrează pattern-ul**:

```rust
// src/engine/patterns/mod.rs
mod map_operations;
pub use map_operations::MapOperationsPattern;

// src/engine/pipeline.rs
use crate::engine::patterns::{..., MapOperationsPattern};

pub fn default_engine() -> Engine {
    // ...
    engine.register(MapOperationsPattern::new());
    engine
}
```

### 2. Adaugă un Template Nou

**Exemplu**: Template pentru `set_admin` function

```rust
// În src/patterns/templates.rs, adaugă în TemplateLibrary::new():

fn add_set_admin_template(&mut self) {
    self.add_template(FunctionTemplate {
        name_pattern: r"^set_admin$".to_string(),
        param_count_min: 1,
        param_count_max: 1,
        param_types: vec![TypePattern::Address],
        return_type: None,
        body_patterns: vec![
            BodyPattern::RequireAuth { with_args: false },
            BodyPattern::StorageSet {
                storage_type: "instance".to_string(),
                key_pattern: Some("Admin".to_string()),
            },
        ],
        template: r#"    pub fn {name}(&mut self, env: Env, {param0}: {param0_type}) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &{param0});
    }"#.to_string(),
        priority: 100,
    });
}

// Nu uita să apelez în new():
pub fn new() -> Self {
    let mut lib = Self { templates: Vec::new() };
    // ...
    lib.add_set_admin_template(); // ← adaugă aici
    lib
}
```

### 3. Folosește Template-urile în Decompilare

**În src/app/decompiled.rs** (sau unde generezi funcțiile):

```rust
use crate::patterns::templates::TemplateMatcher;

let template_matcher = TemplateMatcher::new();

for spec_fn in contract_specs.functions() {
    let export_name = mangle_fn_name(spec_fn.name());

    // Încearcă mai întâi template-urile
    if let Some(generated) = template_matcher.try_generate(spec_fn, &export_name) {
        writeln!(writer, "{}", generated)?;
        continue;
    }

    // Dacă nu găsește template, folosește pattern-urile existente
    if patterns::try_emit(&mut writer, spec_fn, &ctx, &mut state) {
        continue;
    }

    // Fallback la funcții raw
    // ...
}
```

## Testing

### Unit Tests pentru Pattern-uri

```rust
// În src/engine/patterns/storage_access.rs

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::function::FunctionBlock;

    #[test]
    fn test_storage_simplification() {
        let block = FunctionBlock {
            header: "pub fn test() {".to_string(),
            body: vec![
                "        let balance = env.storage().instance().get(&key).unwrap();".to_string(),
            ],
            footer: "    }".to_string(),
            indent: "    ".to_string(),
            name: Some("test".to_string()),
        };

        let pattern = StorageAccessPattern::new();
        let result = pattern.apply(&block);

        assert!(result.is_some());
        // Verifică că transformarea a funcționat
    }
}
```

### Integration Tests

```bash
# Adaugă un test în tests/
# tests/template_generation.rs

use soroban_auditor::patterns::templates::TemplateMatcher;

#[test]
fn test_balance_template() {
    // Creează un FunctionContractSpec mock
    // Verifică că template-ul generează cod corect
}
```

## Debugging

### Verifică ce pattern-uri se aplică

```bash
# Adaugă logging în pipeline.rs
pub fn apply(&self, input: String) -> String {
    for pattern in &self.patterns {
        eprintln!("Applying pattern: {}", pattern.name());
        // ...
    }
}

# Rulează cu logging
RUST_LOG=debug ./target/release/soroban-auditor tests/contract.wasm
```

### Compară output-ul

```bash
# Script pentru comparare
cat > compare_output.sh << 'EOF'
#!/bin/bash
CONTRACT=$1
OLD_OUTPUT="${CONTRACT%.wasm}.rs"
NEW_OUTPUT="/tmp/${CONTRACT##*/}.rs"

./target/release/soroban-auditor "$CONTRACT" "$NEW_OUTPUT"

echo "=== Differences ==="
diff -u "$OLD_OUTPUT" "$NEW_OUTPUT" | head -50
EOF

chmod +x compare_output.sh
./compare_output.sh tests/soroban_token_contract.optimized.wasm
```

## Next Steps

### Prioritate 1: Testează pattern-urile noi (1 zi)

```bash
# Rulează pe toate contractele
for wasm in tests/*.wasm; do
    echo "Testing $wasm"
    ./target/release/soroban-auditor "$wasm" "/tmp/$(basename $wasm .wasm).rs"
done

# Verifică rezultatele
ls -lh /tmp/*.rs
```

### Prioritate 2: Adaugă mai multe template-uri (2-3 zile)

Template-uri de adăugat:
- [ ] `decimals`, `name`, `symbol` (metadata getters)
- [ ] `transfer_from` (token transfers cu allowance)
- [ ] `burn_from` (burn cu allowance)
- [ ] `initialize` / `__constructor` (initialization)
- [ ] `upgrade` (contract upgrade)
- [ ] Getters/setters generici pentru storage

### Prioritate 3: Migrează funcțiile hardcodate (1 săptămână)

```bash
# Migrează funcțiile din src/patterns/token.rs la templates
# Migrează funcțiile din src/patterns/storage.rs la templates
# Verifică că toate funcțiile existente încă funcționează
```

### Prioritate 4: Pattern Analyzer (viitor)

```rust
// src/patterns/analyzer.rs

pub struct PatternAnalyzer {
    // Analizează contracte decompilate
    // Găsește pattern-uri comune
    // Sugerează template-uri noi
}

impl PatternAnalyzer {
    pub fn analyze_corpus(&self, contracts: Vec<String>) -> Vec<PatternSuggestion> {
        // Implementează analiza
    }
}
```

## Metrici de Success

**Înainte** (baseline):
```bash
# Rulează pe toate contractele și numără:
grep -c "panic!(\"decompilation pending\")" tests/*.rs
# → ~15 funcții incomplete în contracte complexe
```

**După pattern-uri noi**:
```bash
# Același test
grep -c "panic!(\"decompilation pending\")" /tmp/*.rs
# → Țintă: <5 funcții incomplete
```

## Resurse Utile

### Documentație Soroban SDK:
- https://docs.rs/soroban-sdk/latest/soroban_sdk/
- https://developers.stellar.org/docs/smart-contracts

### Exemple de Contracte:
- https://github.com/stellar/soroban-examples
- Toate au și source-ul original pentru comparație

### Debug Tools:
```bash
# Inspectează WASM
wasm2wat tests/contract.wasm | less

# Verifică specs în WASM
./target/release/soroban-auditor tests/contract.wasm --specs-only

# Compare fingerprints
./target/release/soroban-auditor tests/contract1.wasm --fingerprint
./target/release/soroban-auditor tests/contract2.wasm --fingerprint
```

## Troubleshooting

### Pattern nu se aplică

1. Verifică că e înregistrat în `pipeline.rs`
2. Adaugă logging în `apply()` pentru a vedea ce se întâmplă
3. Verifică că `Pattern::name()` returnează ceva unic
4. Testează izolat cu un unit test

### Template nu match-ează

1. Verifică regex pattern-ul: `Regex::new(&pattern).unwrap()`
2. Verifică `param_count_min/max`
3. Verifică `param_types` - folosește `TypePattern::Any` pentru testing
4. Print debug în `matches()` să vezi unde fail-uiește

### Build errors

```bash
# Verifică sintaxa
cargo check

# Fix warnings
cargo fix --allow-dirty

# Clean rebuild
cargo clean && cargo build --release
```

## Contact & Contribuții

Pentru întrebări:
- Check RECOMANDARI.md pentru detalii tehnice
- Vezi examples în src/engine/patterns/ pentru inspirație
- Testează întotdeauna pe contractele din tests/

Happy coding! 🚀
