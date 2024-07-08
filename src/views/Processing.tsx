import { Flex, Progress, Text } from "@radix-ui/themes";
import { useParserContext } from "../contexts/Parser";
import { getProcessingValue } from "../contexts/Taurpc";

export default () => {
  const progress = getProcessingValue(useParserContext().state);
  return (
    <Flex
      direction={"column"}
      p="2"
      className="mx-auto w-10/12 max-w-md flex-grow items-center justify-center"
    >
      <Flex direction={"column"} gap="2" className="w-full">
        <Text>Parsing Log</Text>
        <Progress value={progress} max={1} />
      </Flex>
    </Flex>
  );
};
