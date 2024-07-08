import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";
import { ParserState } from "../types";
import { taurpc } from "./Taurpc";

export interface ParserContextType {
  state: ParserState;
}

const ParserContext = createContext<ParserContextType>({ state: "None" });

export const ParserProvider = ({ children }: { children: ReactNode }) => {
  const [state, setState] = useState<ParserContextType>({ state: "None" });
  useEffect(() => {
    let handler = setInterval(async () => {
      const new_state = await taurpc.poll_state();
      setState((s) => ({ ...s, state: new_state }));
    }, 500);
    return () => {
      clearInterval(handler);
    };
  }, []);
  return (
    <ParserContext.Provider value={state}>{children}</ParserContext.Provider>
  );
};

export const useParserContext = () => useContext(ParserContext);
