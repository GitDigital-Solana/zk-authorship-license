---

3. Database Migration SQL

src-tauri/migrations/001_initial_schema.sql

```sql
CREATE TABLE IF NOT EXISTS identities (
    id INTEGER PRIMARY KEY,
    alias TEXT NOT NULL,
    did TEXT NOT NULL UNIQUE,
    public_key BLOB NOT NULL,
    encrypted_private_key BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS authorship (
    id INTEGER PRIMARY KEY,
    identity_id INTEGER REFERENCES identities(id),
    file_path TEXT NOT NULL,
    code_hash BLOB NOT NULL,
    style_vector BLOB NOT NULL,   -- serialized JSON array of floats
    commitment BLOB NOT NULL,
    signature BLOB NOT NULL,
    on_chain_commitment_addr TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS licenses (
    id INTEGER PRIMARY KEY,
    author_identity_id INTEGER REFERENCES identities(id),
    licensee_pubkey BLOB NOT NULL,
    policy_commitment BLOB NOT NULL,
    credential_json TEXT NOT NULL,
    on_chain_policy_addr TEXT,
    issued_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```
