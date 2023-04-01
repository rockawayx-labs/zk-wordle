import init, { verify_receipt as verify } from "wasm-verifier";
import wasm from "wasm-verifier/wasm_verifier_bg.wasm?url";

export async function verifyReceipt(receipt: string) {
  await init(wasm);
  return verify(receipt);
}

onmessage = async (e: MessageEvent<string>) => {
  const result = await verifyReceipt(e.data);
  postMessage(result);
};
