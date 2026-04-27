src-tauri/src/db.rs

```rust
use rusqlite::{Connection, Result};
use std::path::Path;

pub fn initialize(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch(include_str!("../migrations/001_initial_schema.sql"))?;
    Ok(conn)
}
```
