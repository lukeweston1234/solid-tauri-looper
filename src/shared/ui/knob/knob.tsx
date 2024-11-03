import { createSignal } from "solid-js";

export default function Knob(props: {
  onValueChange: (newPercentage: number) => any;
  debounceTime: number | null;
  initialValue: number;
}) {
  let debounceTime = props.debounceTime || 200;
  const SPEED = 0.03;

  const [percentage, setPercentage] = createSignal(
    Math.min(Math.max(props.initialValue, 0), 1)
  );
  const [initalY, setInitialY] = createSignal<number | null>(null);

  let debounceTimeout: any;

  function onPointerDown(event: MouseEvent) {
    document.addEventListener("pointermove", onPointerMove);
    document.addEventListener("pointerup", onPointerUp);
    setInitialY(event.clientY);
  }

  function onPointerMove(event: MouseEvent) {
    let y = initalY();
    if (!y) return;
    let deltaY = y - event.clientY;

    let newPercentage =
      Math.round(
        Math.min(Math.max(0, percentage() + deltaY * SPEED), 1) * 100
      ) / 100;

    setPercentage(newPercentage);
    setInitialY(event.clientY);

    if (debounceTimeout) clearTimeout(debounceTimeout);
    debounceTimeout = setTimeout(() => {
      props.onValueChange && props.onValueChange(newPercentage);
    }, debounceTime);
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
      <span class="absolute left-[27px] top-[20px] pointer-events-none">R</span>
    </div>
  );
}
