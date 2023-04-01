export function extractErrorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

export function objectKeys<O extends object>(o: O) {
  return Object.keys(o) as Array<keyof O>;
}
