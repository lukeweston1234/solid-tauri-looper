import { createSignal } from "solid-js";

export default function Ticker() {
  const [position, setPosition] = createSignal(0);

  setInterval(() => {
    setPosition((prev) => (prev >= 100 ? 0 : prev + 0.2));
  }, 10);

  return (
    <div class="w-full h-full relative">
      <div
        style={{ left: `calc(${position()}% - 2px)` }}
        class="h-full w-[1px] border-l-white border-l-[1px] border-white absolute"
      ></div>
    </div>
  );
}
