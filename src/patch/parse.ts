import { parsePatchFiles } from "@pierre/diffs";
import type { FileDiffMetadata, ParsedPatch } from "@pierre/diffs";

export interface ParsedDiff {
  files: FileDiffMetadata[];
  patchMetadata?: string;
}

export function parsePatch(text: string): ParsedPatch[] {
  if (text.trim().length === 0) return [];
  return parsePatchFiles(text, "diffview", true);
}

export function toParsedDiff(patches: ParsedPatch[]): ParsedDiff[] {
  return patches.map((p) => ({
    files: p.files,
    patchMetadata: p.patchMetadata,
  }));
}
