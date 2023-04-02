import init, { verify_receipt as verify } from "wasm-verifier";
import wasm from "wasm-verifier/wasm_verifier_bg.wasm?url";

export async function verifyReceipt([receipt, imageId, commitment]: [
  string,
  string,
  string
]) {
  await init(wasm);
  return verify(receipt, imageId, commitment);
}

onmessage = async (e: MessageEvent<[string, string, string]>) => {
  const result = await verifyReceipt(e.data);
  postMessage(result);
};
