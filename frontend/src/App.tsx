import { useState } from "react";
import { Verifier } from "./verifier";

function App() {
  const [loading, setLoading] = useState(false);

  const handleTryWorker = async () => {
    setLoading(true);
    const guessResponse = await fetch("/api/guess", {
      method: "POST",
      body: JSON.stringify({ guess: "adept" }),
      headers: {
        "Content-Type": "application/json",
      },
    });
    const { receipt } = await guessResponse.json();

    const verifier = new Verifier();
    try {
      const data = await verifier.verify(receipt);
      console.log(data);
    } catch (e) {
      console.log(e);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="App">
      <h1>{loading ? "Loading" : "Stale"}</h1>
      <div className="card">
        <button onClick={handleTryWorker}>try worker</button>
      </div>
    </div>
  );
}

export default App;
