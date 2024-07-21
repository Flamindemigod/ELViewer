import { Flex, Text } from "@radix-ui/themes";
import { taurpc } from "../contexts/Taurpc";
import { useEffect, useState } from "react";
import { Log, Trial } from "../types";
import dayjs from "dayjs";
import duration from "dayjs/plugin/duration";
import relativeTime from "dayjs/plugin/relativeTime";
import se from "../assets/SE.gif";
dayjs.extend(duration);
dayjs.extend(relativeTime);
export default () => {
  // const [_logStartTime, setStartTime] = useState<Date | null>(null);
  const [trials, setTrials] = useState<Trial[]>([]);
  const [test, _setTest] = useState<Log[]>([]);
  useEffect(() => {
    (async () => {
      setTrials(await taurpc.get_trials());
      // setTest(await taurpc.test());
      // setStartTime(new Date((await taurpc.get_log_start()) * 1000));
    })();
  }, []);
  return (
    <Flex direction={"column"} p="2" gap={"2"}>
      <pre>{JSON.stringify(test, null, "\t")}</pre>
      {trials.map((t, i) => (
        <Flex
          direction={"column"}
          minHeight={"8rem"}
          p={"4"}
          gap={"2"}
          className="rounded-md overflow-clip backdrop-blur-md relative before:absolute"
          key={`${t.id} - ${i}`}
        >
          <Flex
            className="-z-10 absolute inset-0 bg-gradient-to-l from-white/10 to-white/10 via-white/5 bg-500% animate-bg-travel-y"
            direction={"row"}
            justify={"end"}
          >
            <img
              draggable={false}
              className="pointer-events-none touch-none h-full"
              src={se}
              style={{
                maskImage:
                  "linear-gradient(270deg, rgba(0,0,0,1) 50%, rgba(255,255,255,0) 100%)",
              }}
            />
          </Flex>
          <Text
            size={"6"}
            className="font-semibold bg-clip-text text-transparent bg-gradient-to-l from-pink-300 to-cyan-300 animate-bg-travel-y bg-500%"
          >
            {t.id} -{" "}
            {dayjs(
              JSON.parse(t.start_time_ms ?? "0"),
              "milliseconds",
            ).toString()}
          </Text>
          <>
            {`${t.success ? "Cleared in " : "Failed to Clear for "}
              ${dayjs
                .duration({ milliseconds: t.duration_ms ?? 0 })
                .humanize()} with ${t.final_score ?? "No Score"} and ${t.vitality ?? 0 / 1000}/36`}
          </>
        </Flex>
      ))}
    </Flex>
  );
};
