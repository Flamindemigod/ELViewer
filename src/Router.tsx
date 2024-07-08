import { useParserContext } from "./contexts/Parser";
import ProcessingView from "./views/Processing.tsx";
import ProcessedView from "./views/Processed.tsx";

import { Flex, Text } from "@radix-ui/themes";

export default () => {
  const parserContext = useParserContext();
  if (parserContext.state === "None")
    return (
      <Flex
        direction={"column"}
        p="2"
        className="mx-auto w-10/12 max-w-md flex-grow items-center justify-center"
      >
        <Text>Select a Log to View</Text>
      </Flex>
    );
  if (parserContext.state === "Processed") return <ProcessedView />;
  if (parserContext.state.Processing !== undefined) return <ProcessingView />;
};
