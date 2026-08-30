import { getFiletypeFromFileName } from "@pierre/diffs";
import type { SupportedLanguages } from "@pierre/diffs";

const OVERRIDES: Record<string, SupportedLanguages> = {
  ".mts": "typescript",
  ".cts": "typescript",
  Dockerfile: "dockerfile",
  Makefile: "makefile",
};

export function languageForPath(path: string | undefined): SupportedLanguages {
  if (!path) return "text";
  const basename = path.split("/").pop() ?? "";
  if (OVERRIDES[basename]) return OVERRIDES[basename]!;
  if (basename.startsWith(".") && OVERRIDES[basename.slice(1)]) {
    return OVERRIDES[basename.slice(1)]!;
  }
  return getFiletypeFromFileName(basename);
}
