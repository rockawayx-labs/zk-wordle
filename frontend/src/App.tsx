import { Container } from "@mantine/core";
import { Footer } from "./components/Footer";
import { Game } from "./components/Game";

function App() {
  return (
    <Container py="4rem" size="xs">
      <Game />
      <Footer />
    </Container>
  );
}

export default App;
