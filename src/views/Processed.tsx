import { Box, Flex, Text } from "@radix-ui/themes";
import { taurpc } from "../contexts/Taurpc";
import { useEffect, useState } from "react";
import { BeginTrial, EndTrial } from "../types";

export default () => {
  const [trials, setTrials] = useState<[BeginTrial, EndTrial | null][]>([]);
  useEffect(() => {
    (async () => setTrials(await taurpc.get_trials()))();
  }, []);
  return (
    <Flex direction={"column"} p="2" gap={"2"}>
      <Text>
        {trials.map((t, i) => (
          <Box p={"2"} key={`${t[0].id} - ${i}`}>
            {t[0].id} - {t[0].start_time_ms}
            {!!t[1] && (
              <>
                {JSON.stringify(t[1].success)} - {t[1].duration_ms} -{" "}
                {t[1].final_score} - {t[1].final_vitality_bonus / 1000}/36
              </>
            )}
          </Box>
        ))}
      </Text>
    </Flex>
  );
};
