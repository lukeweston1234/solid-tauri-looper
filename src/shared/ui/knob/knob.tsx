import { createSignal } from "solid-js";

export default function Knob() {
  const SPEED = 0.03;

  const [percentage, setPercentage] = createSignal(0);
  const [initalY, setInitialY] = createSignal<number | null>(null);

  function onPointerDown(event: MouseEvent) {
    document.addEventListener("pointermove", onPointerMove);
    document.addEventListener("pointerup", onPointerUp);
    setInitialY(event.clientY);
  }

  function onPointerMove(event: MouseEvent) {
    let y = initalY();
    if (!y) return;
    let deltaY = y - event.clientY;
    setPercentage((prev) => Math.min(Math.max(0, prev + deltaY * SPEED), 1));
    setInitialY(event.clientY);
  }

  function onPointerUp(event: MouseEvent) {
    document.removeEventListener("pointermove", onPointerMove);
    document.removeEventListener("pointerup", onPointerUp);

    setInitialY(event.clientY);
  }

  return (
    <div
      onPointerDown={onPointerDown}
      class="relative w-16 h-16 flex items-center justify-center"
    >
      <svg
        class="stroke-app-primary pointer-events-none"
        width={60}
        height={60}
      >
        <circle
          cx={20}
          cy={20}
          r={24}
          stroke-width={8}
          stroke-dasharray={`${151 * percentage()} 151`}
          fill="none"
          transform="rotate(180, 25, 25)"
        />
      </svg>
      <span class="absolute left-[27px] top-[18px] pointer-events-none">R</span>
    </div>
  );
}
