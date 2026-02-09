# Recomandări pentru Soroban Auditor

## Situația Actuală

### ✅ Ce Funcționează Bine
1. **Engine sofisticat de decompilare** cu ~25+ pattern-uri de transformare AST
2. **System de fingerprinting** pentru identificarea funcțiilor similare
3. **Arhitectură modulară** cu trait Pattern extensibil
4. **Specs extraction** din WASM pentru tipuri și semnături
5. **Decompilare 1:1** pentru contracte simple (hello_world, events, atomic_swap, token)

### ❌ Probleme Identificate
1. **Funcții hardcodate** în `src/patterns/` - prea specifice pentru cazuri particulare
2. **Contracte complexe** incomplet decompilate - funcții ca `claim_launch_funds` rămân cu cod low-level
3. **Coverage incomplet** - lipsesc pattern-uri pentru Map ops, math complex, iterații

---

## 🎯 Plan de Îmbunătățiri

### Prioritate 1: Extinde Engine-ul cu Pattern-uri High-Level Soroban

Adaugă în `src/engine/patterns/`:

#### 1.1 Map Operations Pattern
```rust
// src/engine/patterns/map_operations.rs
pub struct MapOperationsPattern;

// Detectează și simplifică:
// - map.get(&key).unwrap() → map[&key]
// - map.get(&key).unwrap_or(default) → map.get(&key).unwrap_or(default)
// - map.set(key, value) + map.get(key) → pattern de cache
```

#### 1.2 Math Operations Pattern
```rust
// src/engine/patterns/math_operations.rs
pub struct MathOperationsPattern;

// Transformă:
// - wrapping_add/sub/mul → operatori normali când e safe
// - Detectează overflow checks → checked_* operations
// - Pattern-uri pentru fee calculation (x * y / 10000)
```

#### 1.3 Storage Access Pattern
```rust
// src/engine/patterns/storage_access.rs
pub struct StorageAccessPattern;

// Detectează pattern-uri comune:
// - env.storage().instance().get(&key).unwrap() → self.get_storage(key)
// - env.storage().instance().set(&key, &value) → self.set_storage(key, value)
// - env.storage().persistent().get/set → similar
// - env.storage().temporary().get/set → similar
```

#### 1.4 Token Transfer Pattern
```rust
// src/engine/patterns/token_transfers.rs
pub struct TokenTransferPattern;

// Detectează:
// - token::Client::new() + transfer() → pattern de swap/transfer
// - Multiple transfers în secvență → atomic swap pattern
// - Transfer cu fee calculation → fee pattern
```

#### 1.5 Iteration Pattern
```rust
// src/engine/patterns/iteration.rs
pub struct IterationPattern;

// Detectează loops care iterează:
// - Vec.get_unchecked(i) în loop → for item in vec (deja există for_each_val)
// - Map iteration patterns
// - Range iteration patterns
```

#### 1.6 Error Handling Pattern
```rust
// src/engine/patterns/error_handling.rs
pub struct ErrorHandlingPattern;

// Detectează:
// - panic!() calls cu error codes → Result returns
// - fail_with_error() → Result::Err
// - Pattern-uri de validare → guard clauses
```

---

### Prioritate 2: Sistem de Template Matching pentru Funcții

În loc de hardcoding în `src/patterns/`, creează un sistem declarativ:

#### 2.1 Function Templates
```rust
// src/patterns/templates.rs
pub struct FunctionTemplate {
    pub name_pattern: Regex,          // Ex: "balance|get_balance"
    pub param_count: usize,
    pub param_types: Vec<TypePattern>,
    pub return_type: TypePattern,
    pub body_patterns: Vec<BodyPattern>,
    pub template: String,              // Template de cod generat
}

pub enum BodyPattern {
    StorageGet(String),                // Ex: "env.storage().get(&DataKey::Balance(...))"
    StorageSet(String),
    TokenTransfer,
    RequireAuth,
    MapOperation,
    // etc.
}
```

#### 2.2 Template Library
```rust
// Exemplu de template pentru balance functions
let balance_template = FunctionTemplate {
    name_pattern: Regex::new(r"^(get_)?balance$").unwrap(),
    param_count: 1,
    param_types: vec![TypePattern::Address],
    return_type: TypePattern::I128,
    body_patterns: vec![
        BodyPattern::StorageGet("DataKey::Balance({param0})"),
    ],
    template: r#"
    pub fn {name}(&mut self, env: Env, {param0}: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance({param0})).unwrap_or(0)
    }
    "#,
};
```

---

### Prioritate 3: Improved Pattern Recognition

#### 3.1 Analiza Statistică a Pattern-urilor
```rust
// src/patterns/analyzer.rs
pub struct PatternAnalyzer {
    // Analizează toate funcțiile decompilate și găsește pattern-uri comune
    // Generează automat template-uri noi
}

// Rulează pe un corpus de contracte decompilate:
// - Identifică secvențe de cod care se repetă
// - Generează fingerprint-uri pentru pattern-uri
// - Sugerează template-uri noi
```

#### 3.2 Machine Learning pentru Pattern Detection (opțional)
- Antrenează un model pe contracte decompilate manual
- Folosește pentru a prezice pattern-uri în contracte noi
- Poate identifica funcții similare chiar dacă au implementări diferite

---

### Prioritate 4: Îmbunătățiri la Decompilarea Funcțiilor Complexe

#### 4.1 Control Flow Recovery
```rust
// Îmbunătățește detectarea:
// - State machines
// - Complex branching
// - Nested loops
// - Match statements (din if-else chains)
```

#### 4.2 Variable Naming
```rust
// src/engine/patterns/variable_naming.rs
pub struct VariableNamingPattern;

// Detectează tipul variabilei și dă nume sugestive:
// - var1: Address → user/from/to/admin
// - var2: i128 → amount/balance/fee
// - var3: u64 → timestamp/ledger/id
// Bazat pe context de utilizare
```

#### 4.3 Function Extraction
```rust
// Detectează cod duplicat în funcții și extrage în helper functions
pub struct FunctionExtractionPattern;
```

---

### Prioritate 5: Testing & Validation

#### 5.1 Test Suite Expansion
```bash
# Adaugă mai multe contracte test în tests/
- Contracte simple (✓ deja există)
- Contracte medii (token, swap - ✓ există)
- Contracte complexe (✗ lipsesc - ex: DEX, lending, governance)
```

#### 5.2 Round-Trip Testing
```rust
// Verifică că codul decompilat poate fi recompilat
// și produce același WASM (sau similar semantic)
#[test]
fn test_round_trip_compilation() {
    let wasm_original = load_wasm("tests/contract.wasm");
    let rust_decompiled = decompile(wasm_original);
    let wasm_recompiled = compile_rust(rust_decompiled);
    assert_semantically_equivalent(wasm_original, wasm_recompiled);
}
```

#### 5.3 Differential Testing
```rust
// Compară output-ul decompiler-ului cu source-ul original
// Pentru contractele unde avem source-ul
#[test]
fn test_decompile_accuracy() {
    let original = load_source("tests/token.rs");
    let wasm = compile_rust(original);
    let decompiled = decompile(wasm);
    assert_similar(original, decompiled, threshold=0.8);
}
```

---

## 🛠️ Implementare Practică

### Pas 1: Adaugă Pattern-uri Noi în Engine (1-2 săptămâni)

1. Creează fișierele în `src/engine/patterns/`:
   - `map_operations.rs`
   - `math_operations.rs`
   - `storage_access.rs`
   - `token_transfers.rs`
   - `error_handling.rs`
   - `variable_naming.rs`

2. Implementează trait Pattern pentru fiecare

3. Înregistrează în `default_engine()` în `src/engine/pipeline.rs`

4. Testează pe contractele existente

### Pas 2: Creează Sistemul de Template Matching (2-3 săptămâni)

1. Creează `src/patterns/templates.rs` cu:
   - `FunctionTemplate` struct
   - `TemplateLibrary` cu template-uri predefinite
   - `TemplateMatcher` pentru matching

2. Migrează funcțiile hardcodate din:
   - `src/patterns/token.rs` → templates
   - `src/patterns/storage.rs` → templates
   - `src/patterns/swap.rs` → templates

3. Creează template-uri pentru:
   - Balance/allowance operations
   - Transfer operations
   - Admin functions
   - Storage getters/setters
   - Version functions
   - Owner management

### Pas 3: Pattern Analyzer (1-2 săptămâni)

1. Creează `src/patterns/analyzer.rs`

2. Implementează:
   - Pattern frequency analysis
   - Template suggestion
   - Fingerprint clustering

3. Rulează pe corpusul de contracte și generează raport

### Pas 4: Testing Suite (1 săptămână)

1. Adaugă contracte complexe în `tests/`

2. Implementează round-trip testing

3. Adaugă differential testing pentru contractele cu source

4. Creează CI pentru teste automate

---

## 📈 Metrici de Success

### Înainte (situație actuală)
- ✅ Contracte simple: ~95% acuratețe
- ⚠️ Contracte medii: ~70% acuratețe
- ❌ Contracte complexe: ~40% acuratețe
- Funcții hardcodate: ~15 funcții specifice

### După (țintă)
- ✅ Contracte simple: ~98% acuratețe
- ✅ Contracte medii: ~90% acuratețe
- ⚠️ Contracte complexe: ~75% acuratețe
- Template-uri: ~50+ pattern-uri reutilizabile
- Coverage: ~80% din funcțiile comune Soroban

---

## 🔄 Workflow Recomandat

### Pentru Debugging/Development:
```bash
# 1. Decompilează un contract
./target/release/soroban-auditor tests/contract.wasm output.rs

# 2. Compară cu originalul (dacă există)
diff tests/contract.original.rs output.rs

# 3. Identifică pattern-uri care lipsesc
# 4. Adaugă pattern în engine/patterns/
# 5. Re-decompilează și verifică

# 6. Testează pe toate contractele
./run_all_tests.sh
```

### Pentru Adăugarea de Pattern-uri Noi:
```bash
# 1. Identifică pattern-ul în contracte manuale
# 2. Creează pattern în src/engine/patterns/new_pattern.rs
# 3. Adaugă în pipeline (src/engine/pipeline.rs)
# 4. Testează izolat
# 5. Testează pe toate contractele
# 6. Commit
```

---

## 📚 Resurse Utile

### Pentru Pattern Matching:
- Rust syn/quote pentru AST manipulation (deja folosești quote)
- tree-sitter pentru parsing mai robust (opțional)
- regex pentru text-based patterns

### Pentru ML Pattern Detection (opțional viitor):
- Tree-LSTM pentru pattern recognition în AST
- Graph Neural Networks pentru CFG analysis
- Clustering algorithms pentru fingerprint grouping

### Contracte Test:
- Soroban examples: https://github.com/stellar/soroban-examples
- Stellar contracts: https://github.com/stellar/stellar-contract
- Community contracts: explorează Stellar ecosystem

---

## 🎯 Next Steps Immediate

1. **Săptămâna 1-2**: Implementează top 3 pattern-uri noi:
   - MapOperationsPattern
   - StorageAccessPattern
   - MathOperationsPattern

2. **Săptămâna 3-4**: Testează pe contracte complexe și ajustează

3. **Săptămâna 5-6**: Începe sistemul de template matching

4. **Săptămâna 7+**: Pattern analyzer și ML (dacă vrei)

---

## 💡 Idei pentru Viitor

### 1. Interactive Decompiler
```bash
# Decompilare interactivă cu sugestii
soroban-auditor --interactive contract.wasm
# "Found unknown pattern at line 42. Suggest name: [balance_check]"
# User can accept/reject/modify suggestions
```

### 2. Pattern Library Website
- Community-contributed patterns
- Share template libraries
- Download pre-trained models

### 3. IDE Integration
- VS Code extension pentru decompilare în real-time
- Hover over WASM → see decompiled Rust
- Suggestions inline

### 4. Audit Tools
- Detectare de vulnerabilități comune
- Pattern matching pentru anti-patterns
- Security scoring

---

## 📝 Concluzie

Proiectul e deja foarte solid cu engine-ul CFG și pattern-uri. Următorii pași:

1. **Short-term**: Adaugă mai multe pattern-uri high-level în engine
2. **Mid-term**: Generalizează cu template matching
3. **Long-term**: Pattern analyzer și ML pentru auto-discovery

Focusează-te pe **pattern coverage** în loc de hardcoding specific cases.
Scopul: 80%+ din funcțiile comune Soroban să fie decompilate corect.

---

**Contact**: Pentru întrebări sau clarificări despre implementare.
