import { createSignal, onCleanup } from "solid-js";
import { useAppContext } from "../../../../core/app-state/app.context";

export default function Ticker() {
  const [state] = useAppContext();
  const [position, setPosition] = createSignal(0);

  const tickerInterval = setInterval(() => {
    if (state.status === "playing" || state.status === "recording") {
      setPosition((prev) => (prev >= 100 ? 0 : prev + 0.2));
    } else if (state.status === "stopped") {
      setPosition(0);
    }
  }, 10);

  onCleanup(() => clearInterval(tickerInterval));

  return (
    <div class="w-full h-full relative">
      <div
        style={{ left: `calc(${position()}% - 2px)` }}
        class="h-full w-[1px] border-l-white border-l-[1px] border-white absolute"
      ></div>
    </div>
  );
}
