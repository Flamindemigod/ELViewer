import { createTauRPCProxy, ParserState } from "../types.ts";

export const taurpc = await createTauRPCProxy();

export const getProcessingValue = (s: ParserState): number | null => {
  switch (s) {
    case "None":
      return null;
    case "Processed":
      return null;
    default:
      return Math.min(Math.max(0, s.Processing), 1);
  }
};
