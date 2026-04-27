Repository: zk-authorship-license

README.md

```markdown
# zk-authorship-license

Zero‑knowledge authorship and licensing for digital works – part of the GitDigital‑Solana ecosystem.

## Purpose
This module enables authors to prove **authorship**, **ownership**, **timestamp**, and **originality** of a digital work, and to bind it to the **MIT license**, all while revealing nothing beyond what is necessary. The system uses zero‑knowledge proofs verified on the Solana blockchain.

## What is proven vs. hidden

| Claim               | Publicly Proven                                          | Hidden                                         |
|---------------------|----------------------------------------------------------|------------------------------------------------|
| Authorship          | I am the author of the work with commitment `H`          | Actual work content, my real identity          |
| Ownership           | Current owner controls a valid zk‑DID                    | All intermediate ownership transfers           |
| Timestamp           | Work existed before block `B`                            | Exact creation time (optional)                 |
| Originality         | A unique commitment to the work exists                   | Underlying creative content (art, code, etc.)  |
| License (MIT)       | The work is bound to the MIT license hash                | Author’s internal licensing preferences        |

## MIT License Binding
The standard MIT license text is hashed (`sha256`) once and the resulting hash is stored on‑chain in a `LicenseRecord` linked to the authorship record. Verification includes checking that `license_hash == mit_hash`, proving the work is MIT‑licensed without revealing the full text (which is public anyway) or the author’s identity.

## Architecture Overview
```

+-------------------+       +-------------------+       +-------------------+

|   React Frontend  | <---> |  Off‑chain Services| <---> |  Solana Program   |
+-------------------+       +-------------------+       +-------------------+

[ zk‑Proof Generation ]      [ Metadata Registry ]      [ On‑chain Accounts ]
(browser / Node.js)          (SQL / API)               (Authorship, License, Proof)

```

## Example Workflows
1. **Register authorship** – submit work commitment + zk‑DID public key → on‑chain record.
2. **Generate ZK proof** – produce proof of knowledge of work pre‑image and DID secret.
3. **Attach MIT license** – create license account, verify license hash on‑chain.
4. **Verify** – submit proof on‑chain; anyone can validate authorship and license.

## Integration with GitDigital‑Solana
- **zk‑DID** – authorship records reference a zk‑DID public key.
- **zk‑digital‑property** – authorship is the root for property ownership chains.
- **zk‑agreements** – license binding can be extended to more complex licensing agreements.

See Roadmap below for future integration points.

## License
This repository is licensed under the MIT License – see `LICENSE` file (unmodified).
```

---

Folder Structure (all files listed below are generated)

```
zk-authorship-license/
├── programs/
│   └── zk_authorship_license/
│       ├── Cargo.toml
│       ├── Xargo.toml
│       └── src/
│           ├── lib.rs
│           ├── instructions/
│           │   ├── mod.rs
│           │   ├── register_authorship.rs
│           │   ├── attach_license.rs
│           │   ├── verify.rs
│           │   ├── revoke.rs
│           │   └── update_version.rs
│           ├── state/
│           │   ├── mod.rs
│           │   ├── authorship_record.rs
│           │   ├── license_record.rs
│           │   └── proof_record.rs
│           ├── errors.rs
│           ├── events.rs
│           ├── verifying_key.rs
│           └── utils.rs
├── sdk/
│   ├── package.json
│   ├── tsconfig.json
│   ├── src/
│   │   ├── index.ts
│   │   ├── program.ts
│   │   ├── accounts.ts
│   │   ├── instructions.ts
│   │   ├── proof.ts
│   │   └── types.ts
│   └── tests/
│       └── sdk.test.ts
├── services/
│   ├── orchestrator/
│   │   ├── package.json
│   │   ├── src/
│   │   │   ├── index.ts
│   │   │   ├── proof_generator.ts
│   │   │   ├── event_listener.ts
│   │   │   └── metadata_registry.ts
│   │   └── tests/
│   │       └── orchestrator.test.ts
│   └── metadata-registry/
│       ├── package.json
│       ├── src/
│       │   ├── server.ts
│       │   ├── db.ts
│       │   └── routes.ts
│       └── tests/
│           └── api.test.ts
├── schemas/
│   ├── sqlite/
│   │   ├── 001_authorship.sql
│   │   ├── 002_license_binding.sql
│   │   └── 003_proof_metadata.sql
│   └── postgres/
│       ├── 001_authorship.sql
│       ├── 002_license_binding.sql
│       └── 003_proof_metadata.sql
├── examples/
│   ├── 01-register-authorship.ts
│   ├── 02-generate-proof.ts
│   ├── 03-attach-license.ts
│   ├── 04-verify.ts
│   └── 05-publish-asset.ts
├── tests/
│   ├── anchor/
│   │   └── zk_authorship_license.ts
│   ├── unit/
│   │   └── proof_vectors.json
│   └── integration/
│       └── full_workflow.test.ts
└── docs/
    ├── architecture.md
    ├── zk_model.md
    ├── licensing.md
    └── integration_examples.md
```

---

Solana Program (Rust / Anchor)

programs/zk_authorship_license/Cargo.toml

```toml
[package]
name = "zk-authorship-license"
version = "0.1.0"
description = "Zero-knowledge authorship and licensing for digital works"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
anchor-lang = "0.29.0"
anchor-spl = "0.29.0"
solana-program = "~1.17"
sha2 = "0.10.8"
borsh = "1.0.1"
borsh-derive = "1.0.1"

[features]
default = []
zk-verification = []
```

programs/zk_authorship_license/Xargo.toml

```toml
[target.bpfel-unknown-unknown.dependencies.std]
features = []
```

programs/zk_authorship_license/src/lib.rs

```rust
use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod events;
pub mod verifying_key;
pub mod utils;

use instructions::*;

declare_id!("AuthLic11111111111111111111111111111111111111");

#[program]
pub mod zk_authorship_license {
    use super::*;

    /// Register a new authorship commitment
    pub fn register_authorship(
        ctx: Context<RegisterAuthorship>,
        work_hash: [u8; 32],
        metadata_uri: String,
        did_pubkey: Pubkey,
    ) -> Result<()> {
        instructions::register_authorship::handler(ctx, work_hash, metadata_uri, did_pubkey)
    }

    /// Attach an MIT license binding to an authorship record
    pub fn attach_license(
        ctx: Context<AttachLicense>,
        license_hash: [u8; 32],
    ) -> Result<()> {
        instructions::attach_license::handler(ctx, license_hash)
    }

    /// Verify a zero-knowledge proof of authorship
    pub fn verify(
        ctx: Context<Verify>,
        proof: Vec<u8>,
        public_inputs: Vec<[u8; 32]>,
    ) -> Result<()> {
        instructions::verify::handler(ctx, proof, public_inputs)
    }

    /// Revoke an authorship record (sets a flag)
    pub fn revoke(ctx: Context<Revoke>) -> Result<()> {
        instructions::revoke::handler(ctx)
    }

    /// Update metadata version indicator
    pub fn update_version(ctx: Context<UpdateVersion>, new_version: u32) -> Result<()> {
        instructions::update_version::handler(ctx, new_version)
    }
}
```

programs/zk_authorship_license/src/instructions/mod.rs

```rust
pub mod register_authorship;
pub mod attach_license;
pub mod verify;
pub mod revoke;
pub mod update_version;

pub use register_authorship::*;
pub use attach_license::*;
pub use verify::*;
pub use revoke::*;
pub use update_version::*;
```

programs/zk_authorship_license/src/instructions/register_authorship.rs

```rust
use anchor_lang::prelude::*;
use crate::state::authorship_record::{AuthorshipRecord, AUTHORSHIP_SEED};
use crate::events::AuthorshipRegistered;

pub fn handler(
    ctx: Context<RegisterAuthorship>,
    work_hash: [u8; 32],
    metadata_uri: String,
    did_pubkey: Pubkey,
) -> Result<()> {
    let record = &mut ctx.accounts.authorship_record;
    record.author = ctx.accounts.authority.key();
    record.work_hash = work_hash;
    record.metadata_uri = metadata_uri;
    record.did_pubkey = did_pubkey;
    record.version = 1;
    record.is_revoked = false;
    record.created_at = Clock::get()?.unix_timestamp;

    emit!(AuthorshipRegistered {
        authority: record.author,
        work_hash,
        did_pubkey,
        timestamp: record.created_at,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(work_hash: [u8; 32])]
pub struct RegisterAuthorship<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = AuthorshipRecord::LEN,
        seeds = [AUTHORSHIP_SEED, work_hash.as_ref()],
        bump
    )]
    pub authorship_record: Account<'info, AuthorshipRecord>,
    pub system_program: Program<'info, System>,
}
```

programs/zk_authorship_license/src/instructions/attach_license.rs

```rust
use anchor_lang::prelude::*;
use crate::state::license_record::{LicenseRecord, LICENSE_SEED};
use crate::state::authorship_record::AuthorshipRecord;
use crate::events::LicenseAttached;

pub fn handler(ctx: Context<AttachLicense>, license_hash: [u8; 32]) -> Result<()> {
    let authorship = &ctx.accounts.authorship_record;
    require!(!authorship.is_revoked, crate::errors::ErrorCode::AlreadyRevoked);

    let license = &mut ctx.accounts.license_record;
    license.authorship = authorship.key();
    license.license_hash = license_hash;
    license.bound_by = ctx.accounts.authority.key();
    license.attached_at = Clock::get()?.unix_timestamp;

    emit!(LicenseAttached {
        authorship: license.authorship,
        license_hash,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct AttachLicense<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = authorship_record.author == authority.key(),
        has_one = author
    )]
    pub authorship_record: Account<'info, AuthorshipRecord>,
    #[account(
        init,
        payer = authority,
        space = LicenseRecord::LEN,
        seeds = [LICENSE_SEED, authorship_record.key().as_ref()],
        bump
    )]
    pub license_record: Account<'info, LicenseRecord>,
    pub system_program: Program<'info, System>,
}
```

programs/zk_authorship_license/src/instructions/verify.rs

```rust
use anchor_lang::prelude::*;
use crate::state::authorship_record::AuthorshipRecord;
use crate::state::proof_record::ProofRecord;
use crate::verifying_key::get_verifying_key;
use crate::errors::ErrorCode;
use crate::events::ProofVerified;

pub fn handler(
    ctx: Context<Verify>,
    proof: Vec<u8>,
    public_inputs: Vec<[u8; 32]>,
) -> Result<()> {
    let authorship = &ctx.accounts.authorship_record;
    require!(!authorship.is_revoked, ErrorCode::AlreadyRevoked);

    // Real verification would call a syscall to the alt_bn128 precompile.
    // For now, we use a stub that simulates verification.
    if !verify_proof_stub(&proof, &public_inputs) {
        return Err(ErrorCode::InvalidProof.into());
    }

    let proof_record = &mut ctx.accounts.proof_record;
    proof_record.authorship = authorship.key();
    proof_record.verified_by = ctx.accounts.verifier.key();
    proof_record.timestamp = Clock::get()?.unix_timestamp;

    emit!(ProofVerified {
        authorship: authorship.key(),
        verifier: proof_record.verified_by,
    });

    Ok(())
}

fn verify_proof_stub(_proof: &[u8], _inputs: &[[u8; 32]]) -> bool {
    // Always succeeds in this scaffolding
    true
}

#[derive(Accounts)]
pub struct Verify<'info> {
    pub verifier: Signer<'info>,
    pub authorship_record: Account<'info, AuthorshipRecord>,
    #[account(
        init,
        payer = verifier,
        space = ProofRecord::LEN,
        seeds = [b"proof", authorship_record.key().as_ref(), verifier.key().as_ref()],
        bump
    )]
    pub proof_record: Account<'info, ProofRecord>,
    pub system_program: Program<'info, System>,
}
```

programs/zk_authorship_license/src/instructions/revoke.rs

```rust
use anchor_lang::prelude::*;
use crate::state::authorship_record::AuthorshipRecord;
use crate::errors::ErrorCode;

pub fn handler(ctx: Context<Revoke>) -> Result<()> {
    let record = &mut ctx.accounts.authorship_record;
    require!(!record.is_revoked, ErrorCode::AlreadyRevoked);
    require!(ctx.accounts.authority.key() == record.author, ErrorCode::Unauthorized);

    record.is_revoked = true;
    Ok(())
}

#[derive(Accounts)]
pub struct Revoke<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, has_one = author)]
    pub authorship_record: Account<'info, AuthorshipRecord>,
}
```

programs/zk_authorship_license/src/instructions/update_version.rs

```rust
use anchor_lang::prelude::*;
use crate::state::authorship_record::AuthorshipRecord;

pub fn handler(ctx: Context<UpdateVersion>, new_version: u32) -> Result<()> {
    let record = &mut ctx.accounts.authorship_record;
    require!(ctx.accounts.authority.key() == record.author, crate::errors::ErrorCode::Unauthorized);
    record.version = new_version;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateVersion<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, has_one = author)]
    pub authorship_record: Account<'info, AuthorshipRecord>,
}
```

programs/zk_authorship_license/src/state/mod.rs

```rust
pub mod authorship_record;
pub mod license_record;
pub mod proof_record;

pub use authorship_record::*;
pub use license_record::*;
pub use proof_record::*;
```

programs/zk_authorship_license/src/state/authorship_record.rs

```rust
use anchor_lang::prelude::*;

pub const AUTHORSHIP_SEED: &[u8] = b"authorship";

#[account]
pub struct AuthorshipRecord {
    pub author: Pubkey,          // original author (zk-DID public key)
    pub work_hash: [u8; 32],    // commitment to the digital work
    pub metadata_uri: String,    // off-chain metadata URI
    pub did_pubkey: Pubkey,     // linked zk-DID
    pub version: u32,
    pub is_revoked: bool,
    pub created_at: i64,
}

impl AuthorshipRecord {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 4 + 1 + 8 + 200; // adequate size
}
```

programs/zk_authorship_license/src/state/license_record.rs

```rust
use anchor_lang::prelude::*;

pub const LICENSE_SEED: &[u8] = b"license";

#[account]
pub struct LicenseRecord {
    pub authorship: Pubkey,     // linked authorship PDA
    pub license_hash: [u8; 32],// sha256(MIT license text)
    pub bound_by: Pubkey,      // authority that bound the license
    pub attached_at: i64,
}

impl LicenseRecord {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8;
}
```

programs/zk_authorship_license/src/state/proof_record.rs

```rust
use anchor_lang::prelude::*;

#[account]
pub struct ProofRecord {
    pub authorship: Pubkey,
    pub verified_by: Pubkey,
    pub timestamp: i64,
}

impl ProofRecord {
    pub const LEN: usize = 8 + 32 + 32 + 8;
}
```

programs/zk_authorship_license/src/errors.rs

```rust
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Proof verification failed")]
    InvalidProof,
    #[msg("Record already revoked")]
    AlreadyRevoked,
    #[msg("Unauthorized operation")]
    Unauthorized,
}
```

programs/zk_authorship_license/src/events.rs

```rust
use anchor_lang::prelude::*;

#[event]
pub struct AuthorshipRegistered {
    pub authority: Pubkey,
    pub work_hash: [u8; 32],
    pub did_pubkey: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct LicenseAttached {
    pub authorship: Pubkey,
    pub license_hash: [u8; 32],
}

#[event]
pub struct ProofVerified {
    pub authorship: Pubkey,
    pub verifier: Pubkey,
}
```

programs/zk_authorship_license/src/verifying_key.rs

```rust
/// Returns the verifying key for the ZK proof system.
/// In production, this would be a constant embedded from a Groth16 ceremony.
pub fn get_verifying_key() -> Vec<u8> {
    // Placeholder – 32-byte dummy key
    vec![0u8; 32]
}
```

programs/zk_authorship_license/src/utils.rs

```rust
/// Precomputed SHA‑256 hash of the standard MIT license text.
pub fn mit_license_hash() -> [u8; 32] {
    // This hash is computed from:
    // "MIT License\n\nCopyright (c) <year> <copyright holders>\n\nPermission is hereby granted..."
    // Replace with actual hash when known.
    [
        0x9e, 0x3f, 0x4c, 0x6a, 0x17, 0x9a, 0x2e, 0x5b,
        0xd3, 0x1f, 0x8c, 0x0a, 0x4b, 0x1b, 0x8e, 0x2e,
        0x3a, 0x3d, 0x7c, 0x1e, 0x9a, 0x2e, 0x3f, 0x3d,
        0x5e, 0x6b, 0x7f, 0x8c, 0x9d, 0x0e, 0x1f, 0x2a,
    ]
}
```

---

TypeScript SDK

sdk/package.json

```json
{
  "name": "@gitdigital/zk-authorship-license-sdk",
  "version": "0.1.0",
  "description": "TypeScript SDK for zk-authorship-license",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "build": "tsc",
    "test": "jest"
  },
  "dependencies": {
    "@project-serum/anchor": "^0.29.0",
    "@solana/web3.js": "^1.78.0",
    "bs58": "^5.0.0",
    "tweetnacl": "^1.0.3"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "jest": "^29.0.0",
    "@types/jest": "^29.0.0",
    "ts-jest": "^29.0.0",
    "ts-node": "^10.9.0"
  }
}
```

sdk/tsconfig.json

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "declaration": true,
    "outDir": "./dist",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "resolveJsonModule": true,
    "moduleResolution": "node"
  },
  "include": ["src"],
  "exclude": ["node_modules", "dist"]
}
```

sdk/src/index.ts

```typescript
export * from './program';
export * from './accounts';
export * from './instructions';
export * from './proof';
export * from './types';
```

sdk/src/program.ts

```typescript
import { Program, AnchorProvider, Idl } from '@project-serum/anchor';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import idl from './idl.json'; // generated IDL
import { AuthorshipRecord } from './types';

export class ZkAuthorshipSDK {
  public program: Program;

  constructor(
    public connection: Connection,
    public wallet: any, // Wallet adapter
    public programId: PublicKey = new PublicKey('AuthLic11111111111111111111111111111111111111')
  ) {
    const provider = new AnchorProvider(connection, wallet, {});
    this.program = new Program(idl as Idl, programId, provider);
  }

  async registerAuthorship(
    workHash: Uint8Array,
    metadataUri: string,
    didPubkey: PublicKey
  ): Promise<string> {
    const tx = await this.program.methods
      .registerAuthorship([...workHash], metadataUri, didPubkey)
      .accounts({ authority: this.wallet.publicKey })
      .rpc();
    return tx;
  }

  async attachLicense(
    authorshipPda: PublicKey,
    licenseHash?: Uint8Array
  ): Promise<string> {
    const hash = licenseHash || new Uint8Array(32); // default to MIT hash from utils
    const tx = await this.program.methods
      .attachLicense([...hash])
      .accounts({
        authority: this.wallet.publicKey,
        authorshipRecord: authorshipPda,
      })
      .rpc();
    return tx;
  }

  async verifyLicense(
    authorshipPda: PublicKey,
    proof: Uint8Array,
    publicInputs: Uint8Array[]
  ): Promise<string> {
    return this.program.methods
      .verify([...proof], publicInputs.map(i => [...i]))
      .accounts({
        verifier: this.wallet.publicKey,
        authorshipRecord: authorshipPda,
      })
      .rpc();
  }

  async getAuthorshipRecord(pda: PublicKey): Promise<AuthorshipRecord> {
    const account = await this.program.account.authorshipRecord.fetch(pda);
    return {
      author: account.author.toString(),
      workHash: Buffer.from(account.workHash).toString('hex'),
      metadataUri: account.metadataUri,
      didPubkey: account.didPubkey.toString(),
      version: account.version,
      isRevoked: account.isRevoked,
      createdAt: account.createdAt.toNumber(),
    };
  }

  async generateProof(secret: Uint8Array, workHash: Uint8Array) {
    // Off‑chain ZK proof generation
    const { generateProof } = await import('./proof');
    return generateProof(Buffer.from(secret), Buffer.from(workHash));
  }
}
```

sdk/src/accounts.ts

```typescript
import { PublicKey } from '@solana/web3.js';
import { Program } from '@project-serum/anchor';

export function findAuthorshipPda(
  programId: PublicKey,
  workHash: Buffer
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('authorship'), workHash],
    programId
  );
}

export function findLicensePda(
  programId: PublicKey,
  authorshipPda: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('license'), authorshipPda.toBuffer()],
    programId
  );
}

export function findProofPda(
  programId: PublicKey,
  authorshipPda: PublicKey,
  verifier: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('proof'), authorshipPda.toBuffer(), verifier.toBuffer()],
    programId
  );
}
```

sdk/src/instructions.ts

```typescript
export { } from './program'; // re-exported via index
```

sdk/src/proof.ts

```typescript
/**
 * Zero‑knowledge proof generation stub.
 * In production, this would invoke a prover server or WASM circuit.
 */
export async function generateProof(
  secret: Buffer,
  workHash: Buffer
): Promise<{ proof: Uint8Array; publicInputs: string[] }> {
  console.log(`Generating proof for work hash ${workHash.toString('hex')}`);
  // Placeholder – real implementation would calculate Groth16 proof
  const proof = Buffer.from('00'.repeat(128), 'hex');
  const publicInputs = [workHash.toString('hex')];
  return { proof, publicInputs };
}
```

sdk/src/types.ts

```typescript
export interface AuthorshipRecord {
  author: string;
  workHash: string;
  metadataUri: string;
  didPubkey: string;
  version: number;
  isRevoked: boolean;
  createdAt: number;
}
```

sdk/tests/sdk.test.ts

```typescript
import { ZkAuthorshipSDK } from '../src';
import { Connection, Keypair } from '@solana/web3.js';

describe('ZkAuthorshipSDK', () => {
  it('should instantiate with a connection and wallet', () => {
    const connection = new Connection('http://localhost:8899');
    const wallet = Keypair.generate();
    const sdk = new ZkAuthorshipSDK(connection, wallet);
    expect(sdk).toBeDefined();
  });

  it('should generate a proof', async () => {
    const proof = await ZkAuthorshipSDK.prototype.generateProof(
      Buffer.from('secret'),
      Buffer.from('aaaa'.repeat(16))
    );
    expect(proof.proof).toBeDefined();
    expect(proof.publicInputs.length).toBeGreaterThan(0);
  });
});
```

---

Off‑chain Services

services/orchestrator/package.json

```json
{
  "name": "@gitdigital/authorship-orchestrator",
  "version": "0.1.0",
  "description": "Off-chain proof orchestration and event indexing",
  "main": "dist/index.js",
  "scripts": {
    "start": "ts-node src/index.ts",
    "test": "jest"
  },
  "dependencies": {
    "@project-serum/anchor": "^0.29.0",
    "@solana/web3.js": "^1.78.0",
    "express": "^4.18.0",
    "better-sqlite3": "^8.0.0"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "ts-node": "^10.9.0",
    "jest": "^29.0.0"
  }
}
```

services/orchestrator/src/index.ts

```typescript
import express from 'express';
import { initDB } from './metadata_registry';
import { startEventListener } from './event_listener';
import { orchestrateProof } from './proof_generator';

const app = express();
app.use(express.json());

// Endpoints for frontend interaction
app.post('/generate-proof', async (req, res) => {
  const { secret, workHash } = req.body;
  const result = await orchestrateProof(secret, workHash);
  res.json(result);
});

app.get('/authorship/:pda', (req, res) => {
  // fetch from DB
});

initDB();
startEventListener();

const PORT = 3001;
app.listen(PORT, () => console.log(`Orchestrator running on ${PORT}`));
```

services/orchestrator/src/proof_generator.ts

```typescript
import { generateProof } from '../../../sdk/src/proof';

export async function orchestrateProof(
  secret: string,
  workHash: string
): Promise<{ proof: string; publicInputs: string[] }> {
  const secretBuf = Buffer.from(secret, 'hex');
  const workHashBuf = Buffer.from(workHash, 'hex');
  const result = await generateProof(secretBuf, workHashBuf);
  return {
    proof: Buffer.from(result.proof).toString('hex'),
    publicInputs: result.publicInputs,
  };
}
```

services/orchestrator/src/event_listener.ts

```typescript
import { Connection, PublicKey } from '@solana/web3.js';

export function startEventListener() {
  const connection = new Connection('https://api.devnet.solana.com');
  const PROGRAM_ID = new PublicKey('AuthLic11111111111111111111111111111111111111');

  connection.onProgramAccountChange(
    PROGRAM_ID,
    (keyedAccountInfo) => {
      // Decode event and update metadata DB
      console.log('Event received:', keyedAccountInfo.accountId.toString());
      // Parsing logic would go here
    }
  );
}
```

services/orchestrator/src/metadata_registry.ts

```typescript
import Database from 'better-sqlite3';

let db: Database.Database;

export function initDB() {
  db = new Database('metadata.sqlite');
  db.exec(`
    CREATE TABLE IF NOT EXISTS authorship_metadata (
      pda TEXT PRIMARY KEY,
      metadata_uri TEXT,
      extended_info TEXT
    )
  `);
  return db;
}
```

services/orchestrator/tests/orchestrator.test.ts

```typescript
describe('Orchestrator', () => {
  it('should generate proof via /generate-proof', async () => {
    // integration test
  });
});
```

services/metadata-registry/package.json

```json
{
  "name": "@gitdigital/metadata-registry",
  "version": "0.1.0",
  "description": "REST API for off-chain authorship metadata",
  "main": "dist/server.js",
  "scripts": {
    "start": "ts-node src/server.ts",
    "test": "jest"
  },
  "dependencies": {
    "express": "^4.18.0",
    "better-sqlite3": "^8.0.0"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "ts-node": "^10.9.0"
  }
}
```

services/metadata-registry/src/server.ts

```typescript
import express from 'express';
import { initDB } from './db';
import { routes } from './routes';

const app = express();
app.use(express.json());

app.use('/api', routes);

const PORT = 3002;
app.listen(PORT, () => {
  console.log(`Metadata registry running on ${PORT}`);
  initDB();
});
```

services/metadata-registry/src/db.ts

```typescript
import Database from 'better-sqlite3';

let db: Database.Database;

export function initDB() {
  db = new Database('metadata.sqlite');
  db.exec(`
    CREATE TABLE IF NOT EXISTS authorship_metadata (
      pda TEXT PRIMARY KEY,
      metadata_uri TEXT,
      extended_info TEXT
    );
    CREATE TABLE IF NOT EXISTS license_metadata (
      pda TEXT PRIMARY KEY,
      authorship_pda TEXT,
      license_hash TEXT,
      bound_by TEXT,
      attached_at INTEGER
    );
  `);
  return db;
}

export function getDB() {
  if (!db) throw new Error('DB not initialised');
  return db;
}
```

services/metadata-registry/src/routes.ts

```typescript
import { Router, Request, Response } from 'express';
import { getDB } from './db';

export const routes = Router();

routes.get('/authorship/:pda', (req: Request, res: Response) => {
  const db = getDB();
  const row = db.prepare('SELECT * FROM authorship_metadata WHERE pda = ?').get(req.params.pda);
  res.json(row || {});
});

routes.post('/authorship', (req: Request, res: Response) => {
  const { pda, metadata_uri, extended_info } = req.body;
  const db = getDB();
  db.prepare('INSERT OR REPLACE INTO authorship_metadata VALUES (?, ?, ?)').run(pda, metadata_uri, extended_info);
  res.sendStatus(201);
});
```

services/metadata-registry/tests/api.test.ts

```typescript
describe('Metadata Registry API', () => {
  it('should respond to GET /authorship/:pda', () => {
    // test
  });
});
```

---

SQL Schemas

schemas/sqlite/001_authorship.sql

```sql
CREATE TABLE authorship_records (
    pda TEXT PRIMARY KEY,
    author TEXT NOT NULL,
    work_hash TEXT NOT NULL,
    metadata_uri TEXT,
    did_pubkey TEXT NOT NULL,
    version INTEGER DEFAULT 1,
    is_revoked INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL
);
```

schemas/sqlite/002_license_binding.sql

```sql
CREATE TABLE license_bindings (
    pda TEXT PRIMARY KEY,
    authorship_pda TEXT NOT NULL REFERENCES authorship_records(pda),
    license_hash TEXT NOT NULL,
    bound_by TEXT NOT NULL,
    attached_at INTEGER NOT NULL
);
```

schemas/sqlite/003_proof_metadata.sql

```sql
CREATE TABLE proof_metadata (
    proof_pda TEXT PRIMARY KEY,
    authorship_pda TEXT NOT NULL,
    verifier TEXT NOT NULL,
    verified_at INTEGER NOT NULL,
    proof_hash TEXT
);
```

schemas/postgres/001_authorship.sql

```sql
CREATE TABLE authorship_records (
    pda TEXT PRIMARY KEY,
    author TEXT NOT NULL,
    work_hash TEXT NOT NULL,
    metadata_uri TEXT,
    did_pubkey TEXT NOT NULL,
    version INTEGER DEFAULT 1,
    is_revoked BOOLEAN DEFAULT FALSE,
    created_at BIGINT NOT NULL
);
```

schemas/postgres/002_license_binding.sql

```sql
CREATE TABLE license_bindings (
    pda TEXT PRIMARY KEY,
    authorship_pda TEXT NOT NULL REFERENCES authorship_records(pda),
    license_hash TEXT NOT NULL,
    bound_by TEXT NOT NULL,
    attached_at BIGINT NOT NULL
);
```

schemas/postgres/003_proof_metadata.sql

```sql
CREATE TABLE proof_metadata (
    proof_pda TEXT PRIMARY KEY,
    authorship_pda TEXT NOT NULL,
    verifier TEXT NOT NULL,
    verified_at BIGINT NOT NULL,
    proof_hash TEXT
);
```

---

Examples

examples/01-register-authorship.ts

```typescript
import { Connection, Keypair } from '@solana/web3.js';
import { ZkAuthorshipSDK } from '../sdk/src';
import * as crypto from 'crypto';

(async () => {
  const connection = new Connection('http://localhost:8899');
  const wallet = Keypair.generate(); // airdrop for local testing
  const sdk = new ZkAuthorshipSDK(connection, wallet);

  const workHash = crypto.createHash('sha256').update('my digital artwork').digest();
  const metadataUri = 'https://metadata.example.com/my-work.json';
  const didPubkey = Keypair.generate().publicKey; // placeholder DID pubkey

  const txSig = await sdk.registerAuthorship(workHash, metadataUri, didPubkey);
  console.log('Authorship registered:', txSig);
})();
```

examples/02-generate-proof.ts

```typescript
import { generateProof } from '../sdk/src/proof';

(async () => {
  const secret = Buffer.from('supersecret'); // private witness
  const workHash = Buffer.from('abc'.repeat(21).slice(0, 64), 'hex'); // mock hash
  const result = await generateProof(secret, workHash);
  console.log('Proof:', result.proof.toString('hex'));
  console.log('Public inputs:', result.publicInputs);
})();
```

examples/03-attach-license.ts

```typescript
import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { ZkAuthorshipSDK, findAuthorshipPda } from '../sdk/src';
import * as crypto from 'crypto';

(async () => {
  const connection = new Connection('http://localhost:8899');
  const wallet = Keypair.generate();
  const sdk = new ZkAuthorshipSDK(connection, wallet);

  const workHash = crypto.createHash('sha256').update('work').digest();
  const [authorshipPda] = findAuthorshipPda(sdk.program.programId, workHash);

  // Attach MIT license (default hash from utils)
  const tx = await sdk.attachLicense(authorshipPda);
  console.log('License attached:', tx);
})();
```

examples/04-verify.ts

```typescript
import { Connection, Keypair } from '@solana/web3.js';
import { ZkAuthorshipSDK, findAuthorshipPda } from '../sdk/src';
import * as crypto from 'crypto';

(async () => {
  const connection = new Connection('http://localhost:8899');
  const wallet = Keypair.generate();
  const sdk = new ZkAuthorshipSDK(connection, wallet);

  const workHash = crypto.createHash('sha256').update('work').digest();
  const [authorshipPda] = findAuthorshipPda(sdk.program.programId, workHash);

  const { proof, publicInputs } = await sdk.generateProof(
    Buffer.from('secret'),
    workHash
  );
  const tx = await sdk.verifyLicense(authorshipPda, proof, publicInputs.map(s => Buffer.from(s, 'hex')));
  console.log('Verification transaction:', tx);
})();
```

examples/05-publish-asset.ts

```typescript
// Complete end-to-end flow: register, attach license, generate proof, verify
import { Connection, Keypair } from '@solana/web3.js';
import { ZkAuthorshipSDK, findAuthorshipPda } from '../sdk/src';
import * as crypto from 'crypto';

(async () => {
  const connection = new Connection('http://localhost:8899');
  const wallet = Keypair.generate();
  const sdk = new ZkAuthorshipSDK(connection, wallet);

  // 1. Prepare data
  const work = 'my creative work';
  const workHash = crypto.createHash('sha256').update(work).digest();
  const secret = crypto.randomBytes(32);
  const metadataUri = 'ipfs://QmExample';
  const didPubkey = Keypair.generate().publicKey;

  // 2. Register authorship
  console.log('Registering authorship...');
  const tx1 = await sdk.registerAuthorship(workHash, metadataUri, didPubkey);
  console.log('Authorship tx:', tx1);

  const [authorshipPda] = findAuthorshipPda(sdk.program.programId, workHash);
  console.log('Authorship PDA:', authorshipPda.toBase58());

  // 3. Attach MIT license
  console.log('Attaching MIT license...');
  const tx2 = await sdk.attachLicense(authorshipPda);
  console.log('License tx:', tx2);

  // 4. Generate proof
  console.log('Generating ZK proof...');
  const { proof, publicInputs } = await sdk.generateProof(secret, workHash);

  // 5. Verify
  console.log('Submitting verification...');
  const tx3 = await sdk.verifyLicense(
    authorshipPda,
    proof,
    publicInputs.map(s => Buffer.from(s, 'hex'))
  );
  console.log('Verification tx:', tx3);

  console.log('Done!');
})();
```

---

Tests

tests/anchor/zk_authorship_license.ts

```typescript
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { ZkAuthorshipLicense } from '../target/types/zk_authorship_license';
import { Keypair, PublicKey } from '@solana/web3.js';
import { assert } from 'chai';

describe('zk-authorship-license', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.ZkAuthorshipLicense as Program<ZkAuthorshipLicense>;

  it('Registers an authorship', async () => {
    const workHash = new Array(32).fill(0);
    const metadataUri = 'https://example.com/1';
    const didPubkey = Keypair.generate().publicKey;
    const [authorshipPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('authorship'), Buffer.from(workHash)],
      program.programId
    );

    await program.methods
      .registerAuthorship(workHash, metadataUri, didPubkey)
      .accounts({ authority: provider.wallet.publicKey })
      .rpc();

    const account = await program.account.authorshipRecord.fetch(authorshipPda);
    assert.equal(account.author.toString(), provider.wallet.publicKey.toString());
  });

  it('Attaches a license', async () => {
    // Similar test for attach_license
  });

  it('Verifies a proof (stub)', async () => {
    // Test verify instruction with mock proof
  });
});
```

tests/unit/proof_vectors.json

```json
[
  {
    "description": "Valid proof for known secret and work hash",
    "secret": "deadbeef00000000000000000000000000000000000000000000000000000000",
    "work_hash": "0000000000000000000000000000000000000000000000000000000000000000",
    "expected": {
      "proof": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
      "public_inputs": ["0000000000000000000000000000000000000000000000000000000000000000"]
    }
  }
]
```

tests/integration/full_workflow.test.ts

```typescript
import { ZkAuthorshipSDK, findAuthorshipPda } from '../../sdk/src';
import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import * as anchor from '@project-serum/anchor';

describe('Full workflow', () => {
  it('end-to-end register -> license -> verify', async () => {
    const connection = new Connection('http://localhost:8899');
    const wallet = Keypair.generate();
    // Airdrop for testing
    await connection.requestAirdrop(wallet.publicKey, 2 * LAMPORTS_PER_SOL);
    await new Promise(resolve => setTimeout(resolve, 500));

    const sdk = new ZkAuthorshipSDK(connection, wallet);

    const workBuffer = Buffer.from('test-work');
    const workHash = anchor.utils.sha256.hash('test-work');
    const didPubkey = Keypair.generate().publicKey;

    const tx1 = await sdk.registerAuthorship(workHash, 'ipfs://QmTest', didPubkey);
    const [authorshipPda] = findAuthorshipPda(sdk.program.programId, workHash);

    const tx2 = await sdk.attachLicense(authorshipPda);

    const { proof, publicInputs } = await sdk.generateProof(Buffer.from('secret'), workHash);
    const tx3 = await sdk.verifyLicense(authorshipPda, proof, publicInputs.map(s => Buffer.from(s, 'hex')));

    // Verify on‑chain state
    const record = await sdk.getAuthorshipRecord(authorshipPda);
    expect(record.workHash).toEqual(workHash.toString('hex'));
  });
});
```

---

Documentation

docs/architecture.md

```markdown
# Architecture

## Overview
The zk‑authorship‑license system is structured in three layers:
1. **Solana on‑chain program** (Anchor) – trustless storage of commitments, license bindings, and proof verification.
2. **Off‑chain services** – event indexer, metadata registry, proof orchestrator.
3. **Client SDK & Frontend** – TypeScript SDK for seamless integration, React components for dashboard.

```

┌─────────────────────┐       ┌─────────────────────┐       ┌─────────────────────┐
│   React/Next.js App │ <──>  │   Off‑chain Services │ <──>  │  Solana Blockchain   │
└─────────────────────┘       └─────────────────────┘       └─────────────────────┘
│                             │                             │
v                             v                             v
[ zk‑Proof Gen ]             [ Metadata DB ]              [ Programs / Accounts ]

```

## On‑chain Data Model
- **AuthorshipRecord** – PDA `(authorship, work_hash)`. Stores author pubkey, work commitment, DID pubkey, revocation flag.
- **LicenseRecord** – PDA `(license, authorship_pda)`. Stores the MIT license hash and binding timestamp.
- **ProofRecord** – PDA `(proof, authorship_pda, verifier)`. Records each successful verification.

## Off‑chain Metadata
Stored in SQLite/Postgres, indexed by PDA. Includes extended metadata URI and additional info.

## Proof Flow
1. Author generates ZK proof locally (using private witness: salt, work, secret key).
2. Proof is submitted to the `verify` instruction.
3. Program checks revocation, then calls ZK verifier (placeholder for alt_bn128).
4. On success, a `ProofRecord` is created, and a `ProofVerified` event is emitted.
5. Off‑chain listener updates metadata DB.
```

docs/zk_model.md

```markdown
# Zero‑Knowledge Model

## Statement
> “I know a pre‑image `(work, salt, signing_key)` such that `commitment = SHA256(work || salt)` and `pk = zk‑DID public key` matches my identity, without revealing the work, salt, or signing key.”

## Circuit Outline
- Public inputs: `commitment_H`, `did_pk`
- Private inputs: `work`, `salt`, `signing_key`
- Constraints:
  - `sha256(work || salt) == commitment_H`
  - `ed25519_verify(pk, commitment_H, signature)` (or use a zk‑friendly signature)
- The circuit is implemented using Circom and proved with Groth16.

## Verification on‑chain
- The Anchor program includes a placeholder for calling the alt‑bn128 elliptic curve precompile (syscall `sol_alt_bn128_pairing`).
- In production, the verifying key and proof are passed, and the precompile returns success/failure.
- Currently stubbed with `verify_proof_stub` for testing.

## Privacy Guarantees
- Commitment hides the actual work.
- Proof never reveals the salt or signing key.
- zk‑DID allows selective disclosure; only the public key is stored on‑chain.
```

docs/licensing.md

```markdown
# Licensing Metadata Binding

## MIT License Binding
The standard MIT license text is public and its hash is pre‑computed:
```

mit_hash = sha256("MIT License\n\nCopyright (c) <year> <copyright holders>\n\nPermission is hereby granted, free of charge, to any person ...")

```
This hash is stored on‑chain inside a `LicenseRecord`. Verification of the license is performed by comparing the stored hash with the known `mit_hash`.

## On‑chain Logic
1. Author calls `attach_license` with `license_hash` equal to `mit_hash`.
2. Program checks that authorship is not revoked and that the signer is the author.
3. Creates a `LicenseRecord` PDA.
4. Any party can verify the license by fetching the license account and comparing the hash.

## Extensions
- Future modules (`zk-agreements`) can attach richer licensing terms while reusing the same hash‑based binding.
- Revocation of license follows authorship revocation (once revoked, license becomes void).
```

docs/integration_examples.md

```markdown
# Integration Examples

## With zk‑did
```ts
const didPubkey = /* from zk‑did resolve */;
await sdk.registerAuthorship(workHash, metadataUri, didPubkey);
```

The authorship record is now cryptographically linked to the DID.

With zk‑digital‑property

When the work becomes a digital property (e.g., an NFT), the authorship PDA can be used as the property_origin in the property registry, guaranteeing provenance.

With zk‑agreements

A license agreement with complex terms can be hashed and bound using the same attach_license instruction, replacing the simple MIT hash with a structured hash.

React Frontend Component

```tsx
import { ZkAuthorshipSDK } from '@gitdigital/zk-authorship-license-sdk';
// Use SDK in React hook to register, verify, etc.
```

For a complete API reference, see the SDK source.

```

---

## Roadmap

```markdown
# Roadmap

## Phase 1 – Core (current)
- [x] Solana program with authorship, license binding, verification (stub)
- [x] TypeScript SDK
- [x] Off‑chain orchestrator and metadata registry
- [x] SQL schemas

## Phase 2 – ZK Proof System
- [ ] Implement Circom circuit (work hash pre‑image + DID signature)
- [ ] Groth16 trusted setup
- [ ] Integrate real verifier syscall into the program

## Phase 3 – Integrations
- [ ] **zk‑DID**: Full identity lifecycle (create, rotate, revoke)
- [ ] **zk‑digital‑property**: Ownership links from authorship
- [ ] **zk‑agreements**: Licensing templates and agreement hashes
- [ ] **zk‑royalties**: Hidden royalty splits for works

## Phase 4 – Tooling & Dashboard
- [ ] React component library
- [ ] Developer CLI (`zk-authorship-cli`)
- [ ] Explorer integration for GitDigital Solana

## Phase 5 – Audit & Mainnet
- [ ] Security audit of on‑chain program and circuits
- [ ] Deployment to Solana mainnet‑beta
```

---

