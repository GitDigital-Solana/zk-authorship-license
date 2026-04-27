src-tauri/src/models.rs

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StyleVector(pub Vec<f64>);

#[derive(Debug, Serialize, Deserialize)]
pub struct ZigCommitmentOutput {
    pub commitment: String,   // hex-encoded bytes
    pub signature: String,    // hex-encoded signature
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseCredential {
    pub licensee_pubkey: String,
    pub policy_commitment: String,
    pub issued_at: String,
    pub expires: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedCredential {
    pub author_public_key: String,
    pub credential: LicenseCredential,
    pub signature: String,
}

#[derive(Debug, Serialize)]
pub struct RegistrationResult {
    pub commitment: String,
    pub on_chain_addr: String,
}

#[derive(Debug, Serialize)]
pub struct LicenseOutput {
    pub signed_credential: SignedCredential,
    pub policy_on_chain_addr: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProofResult {
    pub proof: String,            // hex-encoded Groth16 proof
    pub public_inputs: Vec<String>,
}

// For internal use
pub struct Identity {
    pub id: i64,
    pub did: String,
    pub public_key: Vec<u8>,
    pub encrypted_private_key: Vec<u8>,
}
```
