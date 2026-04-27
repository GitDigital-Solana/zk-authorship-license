src-tauri/src/commands/prove.rs

```rust
use tauri::command;
use crate::models::*;
use crate::zig_mojo;

#[command]
pub async fn prove_license(credential_json: String) -> Result<ProofResult, String> {
    let cred: SignedCredential = serde_json::from_str(&credential_json)
        .map_err(|e| e.to_string())?;

    // Build Mojo witness
    let mojo_input = serde_json::json!({
        "witness": {
            "credential": cred.credential,
            "signature": cred.signature,
            "licensee_private_key": "placeholder_licensee_privkey" // should be retrieved securely
        },
        "public_inputs": {
            "author_public_key": cred.author_public_key,
            "policy_commitment": cred.credential.policy_commitment,
            "licensee_public_key": cred.credential.licensee_pubkey
        }
    });

    let out = zig_mojo::run_mojo_prover(&mojo_input)?;
    Ok(ProofResult {
        proof: out["proof"].as_str().unwrap_or("").to_string(),
        public_inputs: out["public_inputs"]
            .as_array()
            .map(|a| a.iter().map(|v| v.as_str().unwrap_or("").to_string()).collect())
            .unwrap_or_default(),
    })
}
```
