export interface Theme {
  name: string;
  metaBg: number;
  metaFg: number;
  hunkBg: number;
  hunkFg: number;
  oldLineFg: number;
  newLineFg: number;
  additionBg: number;
  deletionBg: number;
  additionAccent: number;
  deletionAccent: number;
}

export const DEFAULT_THEME: Theme = {
  name: "default",
  metaBg: 0x2a2d34,
  metaFg: 0x9ca0a6,
  hunkBg: 0x0e3a59,
  hunkFg: 0xe6f3ff,
  oldLineFg: 0x6f6f6f,
  newLineFg: 0x6f6f6f,
  additionBg: 0x0f3a18,
  deletionBg: 0x3f0d10,
  additionAccent: 0x7ad27a,
  deletionAccent: 0xe89090,
};

export const DARK_THEME: Theme = {
  name: "dark",
  metaBg: 0x1f2228,
  metaFg: 0x9da0a6,
  hunkBg: 0x0d2c45,
  hunkFg: 0xdceefb,
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
  oldLineFg: 0x9a9a9a,
  newLineFg: 0x9a9a9a,
  additionBg: 0xdbefdc,
  deletionBg: 0xf3d8d8,
  additionAccent: 0x2c7a2c,
  deletionAccent: 0xa93232,
};
