src/components/AuthorDashboard.tsx

```tsx
import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export const AuthorDashboard: React.FC = () => {
  const [filePath, setFilePath] = useState("");
  const [did, setDid] = useState("");
  const [result, setResult] = useState<any>(null);

  const handleRegister = async () => {
    try {
      const res = await invoke("analyze_and_register", { filePath, authorDid: did });
      setResult(res);
    } catch (e) {
      alert("Error: " + e);
    }
  };

  return (
    <div>
      <h2>Register Code Ownership</h2>
      <input
        placeholder="File path"
        value={filePath}
        onChange={(e) => setFilePath(e.target.value)}
      />
      <input
        placeholder="Your DID (public key hex)"
        value={did}
        onChange={(e) => setDid(e.target.value)}
      />
      <button onClick={handleRegister}>Analyze & Register</button>
      {result && (
        <pre>{JSON.stringify(result, null, 2)}</pre>
      )}
    </div>
  );
};
```
