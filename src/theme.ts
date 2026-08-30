export interface Theme {
  name: string;
  metaBg: number;
  metaFg: number;
  hunkBg: number;
  hunkFg: number;
  fileHeaderBg: number;
  fileHeaderFg: number;
  fileHeaderMutedFg: number;
  fileSeparatorFg: number;
  railContextFg: number;
  oldLineFg: number;
  newLineFg: number;
  additionBg: number;
  deletionBg: number;
  additionAccent: number;
  deletionAccent: number;
}

export const DARK_THEME: Theme = {
  name: "dark",
  metaBg: 0x1f2228,
  metaFg: 0x9da0a6,
  hunkBg: 0x0d2c45,
  hunkFg: 0xdceefb,
  fileHeaderBg: 0x2b3138,
  fileHeaderFg: 0xe6edf3,
  fileHeaderMutedFg: 0x8b949e,
  fileSeparatorFg: 0x4a4a4a,
  railContextFg: 0x4a4a4a,
  oldLineFg: 0x6a6a6a,
  newLineFg: 0x6a6a6a,
  additionBg: 0x0e3017,
  deletionBg: 0x350a0d,
  additionAccent: 0x86d687,
  deletionAccent: 0xed9b9b,
};

export const LIGHT_THEME: Theme = {
  name: "light",
  metaBg: 0xe6e6e6,
  metaFg: 0x555555,
  hunkBg: 0xb6dcf5,
  hunkFg: 0x073a5e,
  fileHeaderBg: 0xd9e1e8,
  fileHeaderFg: 0x0d1117,
  fileHeaderMutedFg: 0x57606a,
  fileSeparatorFg: 0xb0b0b0,
  railContextFg: 0xb0b0b0,
  oldLineFg: 0x9a9a9a,
  newLineFg: 0x9a9a9a,
  additionBg: 0xdbefdc,
  deletionBg: 0xf3d8d8,
  additionAccent: 0x2c7a2c,
  deletionAccent: 0xa93232,
};

export const DEFAULT_THEME = DARK_THEME;

/**
 * Names of well-known light Shiki themes. Used to pick a matching diff theme
 * when `--theme auto` is requested. The list is intentionally small; users
 * picking an unknown light theme should pass `--theme light` explicitly.
 */
const LIGHT_SYNTAX_THEME_NAMES = new Set([
  "github-light",
  "github-light-default",
  "github-light-high-contrast",
  "light-plus",
  "solarized-light",
  "min-light",
  "one-light",
  "rose-pine-dawn",
  "slack-ochin",
  "snazzy-light",
  "vitesse-light",
  "material-theme-lighter",
  "catppuccin-latte",
  "nord-light",
  "ayu-light",
  "kleur-light",
]);

/** Heuristic guess at whether a Shiki theme id belongs to a light scheme. */
export function isLightSyntaxTheme(themeId: string): boolean {
  return LIGHT_SYNTAX_THEME_NAMES.has(themeId);
}

/** Resolve a `--theme` value into a concrete `Theme`. */
export function resolveDiffTheme(
  requested: "dark" | "light" | "auto" | undefined,
  syntaxTheme: string,
): Theme {
  const value = requested ?? "auto";
  if (value === "dark") return DARK_THEME;
  if (value === "light") return LIGHT_THEME;
  return isLightSyntaxTheme(syntaxTheme) ? LIGHT_THEME : DARK_THEME;
}
