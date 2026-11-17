# Lab 14: CLI application (capstone)

**Due:** Sunday at 11:59 PM  
**Points:** 10

This lab spans three source files. Implement every `todo!()` in all three. The test suite is pre-written, so make it pass. Do not modify the test modules.

When implementing a function, rename parameters by dropping the leading `_`.

## What to implement

**`src/generator.rs`**: password generation (uses the `rand` crate)

| Function | Description |
|----------|-------------|
| `generate_random(length, use_symbols)` | Random password from `a-z A-Z 0-9` (+ `!@#$%^&*` when `use_symbols` is true) |
| `generate_passphrase(word_count, separator)` | Random words from `WORD_LIST` joined by `separator` |
| `generate_pin(length)` | String of random decimal digits |

**`src/validator.rs`**: strength rating

| Function | Description |
|----------|-------------|
| `validate_strength(password)` | Score 0–7 based on length tiers (≥8/12/16) and character classes (lower/upper/digit/symbol); map to `PasswordStrength` |
| `check_common_patterns(password)` | `true` if all chars identical or password is in `COMMON_PASSWORDS` (case-insensitive) |
| `calculate_entropy(password)` | `length × log₂(charset_size)`; charset grows as classes appear: 26/52/62/94 |

**`src/main.rs`**: CLI wiring (the `clap` structs are already defined)

| Match arm | What to print |
|-----------|---------------|
| `Commands::Random { length, symbols }` | `Generated password: <password>`, then `Entropy: <bits> bits` using `calculate_entropy` |
| `Commands::Passphrase { words, separator }` | `Generated passphrase: <passphrase>` |
| `Commands::Pin { length }` | `Generated PIN: <pin>` |
| `Commands::Validate { password }` | `Password strength: <strength>` and `Entropy: <bits> bits`; if `check_common_patterns` returns `true`, also a line starting with `Warning:` |

The integration tests in `tests/cli.rs` run the program and check these line prefixes, so match them exactly.

Test the CLI once implemented:

```bash
cargo run -- random --length 20 --symbols
cargo run -- passphrase --words 4
cargo run -- pin --length 8
cargo run -- validate "MyStr0ng!Pass2024"
```

## Test locally

Run these from the `week14` directory:

```bash
cargo test
cargo fmt
cargo clippy -- -D warnings
```

CI runs `cargo fmt --check`, so run `cargo fmt` before you push.

## Submit

```bash
git add week14/
git commit -m "Complete Lab 14"
git push origin main
```

A green Week 14 badge earns 10 points.
