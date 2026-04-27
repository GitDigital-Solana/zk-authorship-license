src-tauri/src/commands/license.rs

```rust
use tauri::command;
use crate::models::*;
use sha2::{Sha256, Digest};
use chrono::{Utc, Duration};

#[command]
pub async fn issue_license(
    author_did: String,
    licensee_pubkey: String,
    policy_terms: String,
    expiry_days: u32,
) -> Result<LicenseOutput, String> {
    // 1. Policy commitment
    let mut hasher = Sha256::new();
    hasher.update(policy_terms.as_bytes());
    let policy_hash = hasher.finalize();

    let now = Utc::now();
    let credential = LicenseCredential {
        licensee_pubkey: licensee_pubkey.clone(),
        policy_commitment: hex::encode(policy_hash),
        issued_at: now.to_rfc3339(),
        expires: (now + Duration::days(expiry_days as i64)).to_rfc3339(),
    };

    // 2. Sign credential (mock sign; use actual key management)
    let signature = format!("mock_signature_{}", credential.licensee_pubkey);

    let signed = SignedCredential {
        author_public_key: author_did.clone(),
        credential,
        signature,
    };

    // 3. Optionally post policy hash on-chain (mock)
    let policy_on_chain = Some(format!("mock_policy_addr_{}", hex::encode(policy_hash)));

    Ok(LicenseOutput {
        signed_credential: signed,
        policy_on_chain_addr: policy_on_chain,
    })
}
```
