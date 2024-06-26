import { CompletionContext } from "../CompletionContext";
import { PostprocessFilter, logger } from "./base";
import { splitLines } from "../utils";

export function trimMultiLineInSingleLineMode(): PostprocessFilter {
  return (input: string, context: CompletionContext) => {
    const inputLines = splitLines(input);
    if (context.mode === "fill-in-line" && inputLines.length > 1) {
      const suffix = context.currentLineSuffix.trimEnd();
      const inputLine = inputLines[0]!.trimEnd();
      if (inputLine.endsWith(suffix)) {
        const trimmedInputLine = inputLine.slice(0, -suffix.length);
        if (trimmedInputLine.length > 0) {
          logger.trace("Trim content with multiple lines.", { inputLines, trimmedInputLine });
          return trimmedInputLine;
        }
      }
      logger.trace("Drop content with multiple lines.", { inputLines });
      return null;
    }
    return input;
  };
}
