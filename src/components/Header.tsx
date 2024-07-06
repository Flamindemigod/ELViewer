import {
  Button,
  Flex,
  Progress,
  Text,
  useThemeContext,
} from "@radix-ui/themes";
import LogoDark from "../assets/ELViewer.svg";
import LogoLight from "../assets/ELViewerLight.svg";
import { UploadIcon } from "@radix-ui/react-icons";
import { open } from "@tauri-apps/plugin-dialog";
import { useState } from "react";
import { taurpc } from "../contexts/Taurpc";

export default () => {
  const [progress, setProgess] = useState<number | null>(null);
  const theme = useThemeContext();
  let logo = LogoDark;
  if (theme.appearance === "light") {
    logo = LogoLight;
  }
  return (
    <Flex direction={"row"} gap={"2"} align={"center"} p="2">
      <div className="logo relative aspect-square h-16">
        <img
          draggable={false}
          src={logo}
          alt="EL Viewer Logo"
          className="absolute inset-0 object-fill animate-spin object-center origin-center"
        />
      </div>
      <Text className="text-3xl font-semibold font-sans">EL Viewer</Text>
      {!!progress ? (
        <Flex direction={"column"} p="2" gap="2" className="ml-auto ">
          <Text className="px-4">Parsing Log</Text>
          <Progress value={progress} max={1} />
        </Flex>
      ) : (
        <Button
          className="ml-auto relative"
          size={"3"}
          onClick={async () => {
            const file = await open({
              multiple: false,
              directory: false,
              title: "Select a Log File to View",
              filters: [
                {
                  name: "TESO Encounter Log",
                  mimeType: "text/plain",
                  extensions: ["log"],
                },
              ],
            });
            let handler: ReturnType<typeof setInterval> | null = null;
            if (!!file) {
              taurpc.upload(file.path).then(() => {
                if (!!handler) {
                  clearTimeout(handler);
                  handler = null;
                }
                setProgess(null);
              });
              handler = setInterval(async () => {
                let pol_res = await taurpc.poll_state();
                setProgess(
                  Math.min(
                    Math.max(
                      0,
                      pol_res === "None" || pol_res === "Processed"
                        ? 0
                        : pol_res.Processing,
                    ) ?? 0,
                    1,
                  ),
                );
              }, 500);
            }
          }}
        >
          <UploadIcon /> Select A Log
        </Button>
      )}
    </Flex>
  );
};
