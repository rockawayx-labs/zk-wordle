import { useEffect, useState } from "react";
import { ethers } from "ethers";
import { Container, LoadingOverlay } from "@mantine/core";
import { showNotification } from "@mantine/notifications";
import { Footer } from "./components/Footer";
import { Game } from "./components/Game";
import { CONTRACT_ABI, CONTRACT_ADDRESS, PROVIDER_API_KEY } from "./constants";
import { extractErrorMessage } from "./utils";

export interface ContractData {
  commitment: string;
  imageId: string;
}

function App() {
  const [contractData, setContractData] = useState<ContractData>();

  useEffect(() => {
    const provider = ethers.getDefaultProvider(
      `https://polygon-mumbai.g.alchemy.com/v2/${PROVIDER_API_KEY}`
    );
    const contract = new ethers.Contract(
      CONTRACT_ADDRESS,
      CONTRACT_ABI,
      provider
    );

    Promise.all([contract.commitment(), contract.imageId()])
      .then(([commitment, imageId]) => {
        setContractData({ commitment, imageId });
      })
      .catch((e) => {
        showNotification({
          title: "Error",
          message: extractErrorMessage(e),
          color: "red",
        });
      });
  }, []);

  return (
    <Container py="4rem" size="xs">
      <LoadingOverlay visible={!contractData} />
      <Game contractData={contractData} />
      <Footer />
    </Container>
  );
}

export default App;
