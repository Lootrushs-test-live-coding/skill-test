import { useState } from "react";
import { createRoot } from "react-dom/client";
import { ReleasePanel, ReleaseStatus, Role } from "./ReleasePanel";

function Harness() {
  const [status, setStatus] = useState<ReleaseStatus>("funded");
  const [role, setRole] = useState<Role>("buyer");
  const [mode, setMode] = useState<"ok" | "error" | "text">("ok");

  return (
    <main>
      <p>
        <label>
          role{" "}
          <select value={role} onChange={(event) => setRole(event.target.value as Role)}>
            <option value="buyer">buyer</option>
            <option value="seller">seller</option>
            <option value="arbiter">arbiter</option>
          </select>
        </label>{" "}
        <label>
          status{" "}
          <select value={status} onChange={(event) => setStatus(event.target.value as ReleaseStatus)}>
            <option value="funded">funded</option>
            <option value="released">released</option>
            <option value="refunded">refunded</option>
          </select>
        </label>{" "}
        <label>
          onRelease{" "}
          <select value={mode} onChange={(event) => setMode(event.target.value as "ok" | "error" | "text")}>
            <option value="ok">resolves</option>
            <option value="error">rejects with Error</option>
            <option value="text">rejects with a string</option>
          </select>
        </label>
      </p>
      <ReleasePanel
        seller="0xSeller"
        amount="500"
        status={status}
        role={role}
        onRelease={() =>
          new Promise((resolve, reject) => {
            setTimeout(() => {
              if (mode === "ok") resolve();
              else if (mode === "error") reject(new Error("network"));
              else reject("nope");
            }, 400);
          })
        }
      />
    </main>
  );
}

const root = document.getElementById("root");
if (root) {
  createRoot(root).render(<Harness />);
}
