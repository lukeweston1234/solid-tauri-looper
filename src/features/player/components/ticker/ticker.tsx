import { createSignal } from "solid-js";

export default function Ticker() {
  const [position, setPosition] = createSignal(0);

  setInterval(() => {
    setPosition((prev) => (prev > 100 ? 0 : prev + 0.3));
  }, 20);

  return (
    <div class="w-full h-full relative">
      <div
        style={{ left: `${position()}%` }}
        class="h-full w-[1px] bg-white absolute"
      ></div>
    </div>
  );
}
