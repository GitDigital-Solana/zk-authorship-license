src/components/LicenseIssuer.tsx

```tsx
import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export const LicenseIssuer: React.FC = () => {
  const [authorDid, setAuthorDid] = useState("");
  const [licenseePubkey, setLicenseePubkey] = useState("");
  const [terms, setTerms] = useState("view");
  const [expiry, setExpiry] = useState(365);
  const [result, setResult] = useState<any>(null);

  const handleIssue = async () => {
    try {
      const res = await invoke("issue_license", {
        authorDid,
        licenseePubkey,
        policyTerms: terms,
        expiryDays: expiry,
      });
      setResult(res);
    } catch (e) {
      alert("Error: " + e);
    }
  };

  return (
    <div>
      <h2>Issue License</h2>
      <input placeholder="Author DID" value={authorDid} onChange={(e) => setAuthorDid(e.target.value)} />
      <input placeholder="Licensee Pubkey" value={licenseePubkey} onChange={(e) => setLicenseePubkey(e.target.value)} />
      <input placeholder="Policy terms (e.g. view, edit)" value={terms} onChange={(e) => setTerms(e.target.value)} />
      <input type="number" placeholder="Expiry days" value={expiry} onChange={(e) => setExpiry(Number(e.target.value))} />
      <button onClick={handleIssue}>Issue License</button>
      {result && <pre>{JSON.stringify(result, null, 2)}</pre>}
    </div>
  );
};
```
