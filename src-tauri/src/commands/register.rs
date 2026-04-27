src-tauri/src/commands/register.rs

```rust
use tauri::command;
use crate::models::*;
use crate::ollama;
use crate::zig_mojo;
use crate::db;
use sha2::{Sha256, Digest};

#[command]
pub async fn analyze_and_register(
    file_path: String,
    author_did: String,
) -> Result<RegistrationResult, String> {
    // 1. Ollama style vector
    let style_vec = ollama::extract_style_vector(&file_path)?;

    // 2. Code hash
    let code_bytes = std::fs::read(&file_path).map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    hasher.update(&code_bytes);
    let code_hash = hasher.finalize();

    // 3. Get author's public key (for demo, assuming did = public key hex)
    let author_pubkey = hex::decode(&author_did).map_err(|e| e.to_string())?;

    // 4. Call Zig to generate commitment
    let zig_input = serde_json::json!({
        "action": "create_commitment",
        "code_hash": hex::encode(code_hash),
        "style_vector": style_vec.0,
        "author_public_key": hex::encode(&author_pubkey),
    });
    let zig_out = zig_mojo::run_zig_commitment(&zig_input)?;

    // 5. Post commitment hash to Solana (mocked, replace with real tx)
    //    For now, simulate an on-chain address
    let on_chain_addr = format!("mock_onchain_addr_{}", &zig_out.commitment[..8]);

    // 6. Store in SQLite (pseudo-code, use actual DB connection)
    //    save_authorship(&author_did, &file_path, &code_hash, &style_vec, &zig_out)?;

    Ok(RegistrationResult {
        commitment: zig_out.commitment,
        on_chain_addr,
    })
}
```
