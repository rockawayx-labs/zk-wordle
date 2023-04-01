import { useState, useEffect } from "react";
import { ethers } from "ethers";
import { Verifier } from "./verifier";
import { RECEIPT } from "./receipt";

const PROVIDER_API_KEY = "VDEtXZglGFw5AoR48KaAj-ngFWYUehMY";
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
  const [loading, setLoading] = useState(false);
  const [commitment, setCommitment] = useState("");
  const [imageId, setImageId] = useState("");

  useEffect(() => {
    async function init() {
      const provider = ethers.getDefaultProvider(
        `https://polygon-mumbai.g.alchemy.com/v2/${PROVIDER_API_KEY}`
      );
      const contract = new ethers.Contract(
        CONTRACT_ADDRESS,
        CONTRACT_ABI,
        provider
      );
      const commitmentResp = await contract.commitment();
      const imageIdResp = await contract.imageId();
      console.log({ commitmentResp, imageIdResp });

      setCommitment(commitmentResp);
      setImageId(imageIdResp);
    }

    init();
  }, []);

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
      const data = await verifier.verify(receipt, imageId, commitment);
      // const data = await verifier.verify(RECEIPT, imageId, commitment);
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
