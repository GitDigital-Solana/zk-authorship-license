src/components/LicenseVerifier.tsx

```tsx
import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export const LicenseVerifier: React.FC = () => {
  const [credJson, setCredJson] = useState("");
  const [proof, setProof] = useState<any>(null);

  const handleProve = async () => {
    try {
      const res = await invoke("prove_license", { credentialJson: credJson });
      setProof(res);
    } catch (e) {
      alert("Error: " + e);
    }
  };

  return (
    <div>
      <h2>Generate License Proof</h2>
      <textarea
        rows={6}
        placeholder="Paste signed credential JSON"
        value={credJson}
        onChange={(e) => setCredJson(e.target.value)}
      />
      <button onClick={handleProve}>Create ZK Proof</button>
      {proof && <pre>{JSON.stringify(proof, null, 2)}</pre>}
    </div>
  );
};
```
