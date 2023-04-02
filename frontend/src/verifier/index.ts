import type { VerifyResultType } from "wasm-verifier";

export class Verifier {
  public readonly worker: Worker;

  constructor() {
    this.worker = new Worker(new URL("./worker.ts", import.meta.url), {
      type: "module",
    });
  }

  public async verify(receipt: string, imageId: string, commitment: string) {
    return new Promise<VerifyResultType>((resolve, reject) => {
      // Set up a listener for the worker thread's response
      this.worker.onmessage = (event) => {
        try {
          // Parse the response as JSON
          const result: VerifyResultType = JSON.parse(event.data);
          console.log(result);
          resolve(result);
        } catch (error) {
          reject(error);
        }
      };

      this.worker.onerror = (event) => {
        reject(event.error);
      };

      this.worker.postMessage([receipt, imageId.slice(2), commitment.slice(2)]);
    });
  }
}
