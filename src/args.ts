export interface ParsedArgs {
  help: boolean;
  version: boolean;
  noLineNumbers: boolean;
  theme?: "dark" | "light" | "auto";
  syntaxTheme?: string;
  positional: string[];
}

export function parseArgs(argv: string[]): ParsedArgs {
  const out: ParsedArgs = {
    help: false,
    version: false,
    noLineNumbers: false,
    positional: [],
  };

  let i = 0;
  while (i < argv.length) {
    const arg = argv[i]!;

    switch (arg) {
      case "-h":
      case "--help":
        out.help = true;
        i++;
        break;
      case "-v":
      case "--version":
        out.version = true;
        i++;
        break;
      case "--no-line-numbers":
        out.noLineNumbers = true;
        i++;
        break;
      case "--theme": {
        const next = argv[i + 1];
        if (!next || next.startsWith("-")) {
          throw new Error("--theme requires a value (dark, light, or auto)");
        }
        if (next !== "dark" && next !== "light" && next !== "auto") {
          throw new Error(`--theme value must be one of: dark, light, auto (got: ${next})`);
        }
        out.theme = next;
        i += 2;
        break;
      }
      case "--syntax-theme":
      case "--shiki-theme": {
        const next = argv[i + 1];
        if (!next || next.startsWith("-")) {
          throw new Error(`${arg} requires a value (a Shiki theme id)`);
        }
        out.syntaxTheme = next;
        i += 2;
        break;
      }
      default:
        if (arg.startsWith("--") || (arg.startsWith("-") && arg.length > 1)) {
          throw new Error(`Unknown option: ${arg}`);
        }
        out.positional.push(arg);
        i++;
        break;
    }
  }

  return out;
}
