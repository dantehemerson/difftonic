const ANSI_ESCAPE = /\x1b\[[0-9;]*[A-Za-z]/g;
const CR = /\r\n?/g;

export async function readPatch(): Promise<string> {
  const chunks: Buffer[] = [];
  for await (const chunk of Bun.stdin.stream()) {
    chunks.push(chunk as Buffer);
  }
  const raw = Buffer.concat(chunks).toString("utf8");
  return sanitize(raw);
}

export function sanitize(patch: string): string {
  return patch.replace(ANSI_ESCAPE, "").replace(CR, "\n");
}
