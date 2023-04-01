import init, { verify_receipt as verify } from "wasm-verifier";
import wasm from "wasm-verifier/wasm_verifier_bg.wasm?url";

export async function verifyReceipt(
  receipt: string,
  imageId: string,
  wordCommitment: string
) {
  await init(wasm);
  return verify(receipt, imageId, wordCommitment);
}

onmessage = async (e: MessageEvent<string>) => {
  console.log("e.data: ", e.data);
  const result = await verifyReceipt(e.data[0], e.data[1], e.data[2]);
  console.log("on message result: ", result);
  postMessage(result);
};
