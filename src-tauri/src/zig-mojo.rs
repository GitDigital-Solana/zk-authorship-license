src-tauri/src/zig_mojo.rs

```rust
use std::process::{Command, Stdio};
use std::io::Write;
use serde_json::Value;
use crate::models::*;

pub fn run_zig_commitment(input: &Value) -> Result<ZigCommitmentOutput, String> {
    let mut child = Command::new("zig-commitment")
        .arg("--stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    let stdin = child.stdin.as_mut().unwrap();
    let input_str = input.to_string();
    stdin.write_all(input_str.as_bytes()).map_err(|e| e.to_string())?;
    drop(stdin);

    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(stdout.trim()).map_err(|e| format!("Invalid Zig output: {}", e))
}

pub fn run_mojo_prover(input: &Value) -> Result<Value, String> {
    let mut child = Command::new("mojo-prover")
        .arg("--stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    let stdin = child.stdin.as_mut().unwrap();
    let input_str = input.to_string();
    stdin.write_all(input_str.as_bytes()).map_err(|e| e.to_string())?;
    drop(stdin);

    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(stdout.trim()).map_err(|e| format!("Invalid Mojo output: {}", e))
}
```
