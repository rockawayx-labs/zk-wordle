import init, { verify_receipt_with as verify } from "wasm-verifier";
import wasm from "wasm-verifier/wasm_verifier.wasm?url";

export async function verifyReceipt(receipt: string) {
  await init(wasm);
  return verify(receipt);
}

onmessage = async (e: MessageEvent<string>) => {
  const result = await verifyReceipt(e.data);
  postMessage(result);
};
