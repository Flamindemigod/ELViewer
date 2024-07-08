import { Button, Flex, Text, useThemeContext } from "@radix-ui/themes";
import LogoDark from "../assets/ELViewer.svg";
import LogoLight from "../assets/ELViewerLight.svg";
import { UploadIcon } from "@radix-ui/react-icons";
import { open } from "@tauri-apps/plugin-dialog";
import { getProcessingValue, taurpc } from "../contexts/Taurpc";
import { useParserContext } from "../contexts/Parser";

export default () => {
  const theme = useThemeContext();
  const processing = getProcessingValue(useParserContext().state) !== null;
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
      <Button
        className="ml-auto relative"
        size={"3"}
        disabled={processing}
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
          if (!!file) {
            taurpc.upload(file.path);
          }
        }}
      >
        <UploadIcon /> Select A Log
      </Button>
    </Flex>
  );
};
