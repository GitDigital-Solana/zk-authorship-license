src/App.tsx

```tsx
import React from "react";
import { AuthorDashboard } from "./components/AuthorDashboard";
import { LicenseIssuer } from "./components/LicenseIssuer";
import { LicenseVerifier } from "./components/LicenseVerifier";

const App: React.FC = () => {
  const [tab, setTab] = React.useState<"register" | "issue" | "verify">("register");

  return (
    <div style={{ padding: 20 }}>
      <h1>ZK Authorship & Licensing</h1>
      <div>
        <button onClick={() => setTab("register")}>Register Work</button>
        <button onClick={() => setTab("issue")}>Issue License</button>
        <button onClick={() => setTab("verify")}>Prove License</button>
      </div>
      <hr />
      {tab === "register" && <AuthorDashboard />}
      {tab === "issue" && <LicenseIssuer />}
      {tab === "verify" && <LicenseVerifier />}
    </div>
  );
};

export default App;
```
