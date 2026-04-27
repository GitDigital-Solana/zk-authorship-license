src-tauri/src/ollama.rs

```rust
use std::process::Command;
use serde_json::Value;
use crate::models::StyleVector;

pub fn extract_style_vector(file_path: &str) -> Result<StyleVector, String> {
    let output = Command::new("ollama")
        .args([
            "run",
            "codellama:7b",
            "--",
            &format!(
                "Extract code style features as JSON array of 256 floats. Output only JSON with key \"vector\". File: {}",
                file_path
            ),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: Value = serde_json::from_str(stdout.trim())
        .map_err(|e| format!("Ollama output not valid JSON: {}", e))?;

    let arr: Vec<f64> = parsed["vector"]
        .as_array()
        .ok_or("Missing 'vector' array in Ollama response")?
        .iter()
        .map(|v| v.as_f64().ok_or("Non-float element"))
        .collect::<Result<Vec<f64>, _>>()?;

    Ok(StyleVector(arr))
}
```
