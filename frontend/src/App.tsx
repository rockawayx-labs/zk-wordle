import { useState, useEffect } from "react";
import { ethers } from "ethers";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";

const CONTRACT_ADDRESS = "0x307B04Fd818eD3620847cE88fAfa73b80e090E79";
const CONTRACT_ABI = [
  {
    inputs: [],
    name: "commitment",
    outputs: [
      {
        internalType: "bytes32",
        name: "",
        type: "bytes32",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "imageId",
    outputs: [
      {
        internalType: "bytes32",
        name: "",
        type: "bytes32",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
];

function App() {
  const [count, setCount] = useState(0);
  const [loading, setLoading] = useState(false);
  const [commitment, setCommitment] = useState('');
  const [imageId, setImageId] = useState('');

  useEffect(() => {
    async function init() {
      const provider = ethers.getDefaultProvider(
        `https://polygon-mumbai.g.alchemy.com/v2/${import.meta.env.VITE_ALCHEMY_MUMBAI_API_KEY}`
      );
      const contract = new ethers.Contract(
        CONTRACT_ADDRESS,
        CONTRACT_ABI,
        provider
      );
      const commitmentResp = await contract.commitment();
      const imageIdResp = await contract.imageId();
      console.log({commitmentResp, imageIdResp});

      setCommitment(commitmentResp);
      setImageId(imageIdResp);
    }

    init();
  }, []);

  const handleFetchWithProxy = async () => {
    const response = await fetch("/api/healthcheck");
    console.log(await response.text());
  };

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

    const rsWorker = new Worker(new URL("./worker.ts", import.meta.url), {
      type: "module",
    });

    rsWorker.postMessage(receipt);
    rsWorker.onmessage = (e) => {
      console.log(e);
      setLoading(false);
      rsWorker.terminate();
    };
  };

  const handleTryCheck = async () => {
    setLoading(true);
    const guessResponse = await fetch("/api/guess", {
      method: "POST",
      body: JSON.stringify({ guess: "adept" }),
      headers: {
        "Content-Type": "application/json",
      },
    });
    const { receipt } = await guessResponse.json();

    const checkResponse = await fetch("/api/check", {
      method: "POST",
      body: JSON.stringify({ receipt }),
      headers: {
        "Content-Type": "application/json",
      },
    });

    console.log(await checkResponse.json());
    setLoading(false);
  };

  return (
    <div className="App">
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>{loading ? "Loading" : "Stale"}</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <button onClick={handleFetchWithProxy}>fetch with proxy</button>
        <button onClick={handleTryWorker}>try worker</button>
        <button onClick={handleTryCheck}>try check</button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </div>
  );
}

export default App;
