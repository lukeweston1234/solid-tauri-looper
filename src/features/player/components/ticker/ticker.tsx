import { createEffect, createSignal, onCleanup, Show } from "solid-js";
import { useAppContext } from "../../../../core/app-state/app.context";

export default function Ticker() {
  const [state] = useAppContext();
  const [position, setPosition] = createSignal(0);
  const [loopStartInstant, setLoopStartInstant] = createSignal<number | null>(
    null
  );

  let tickerInterval: any;

  createEffect(() => {
    console.log("Effect triggered, state.status:", state.status);


    const loopTime =
      (state.timeInformation.beatsPerMeasure * state.timeInformation.bars * 60) /
      state.bpm;
    const loopInterval = (loopTime / 250) * 1000;

    setPosition(0);
    setLoopStartInstant(Date.now());

    if (state.status === "playing" || state.status === "recording") {
      tickerInterval = setInterval(() => {
        if (state.status !== "playing" && state.status !== "recording") return;

        const loopStartItem = loopStartInstant();

        if (!loopStartItem) {
          setLoopStartInstant(Date.now());
          setPosition(0);
        } else {
          const newPosition = (Date.now() - loopStartItem) / (loopTime * 1000);
          if (newPosition >= 1) {
            setLoopStartInstant(Date.now());
            setPosition(0);
          } else {
          }
          setPosition(newPosition);
        }
      }, loopInterval);
    }
  });

  onCleanup(() => tickerInterval && clearInterval(tickerInterval));

  return (
    <div id="ticker" class="w-full h-full relative z-[100]">
      <Show when={state.status !== "stopped"}>
        <div
          style={{ left: `calc(${position() * 100}% - 2px)` }}
          class="h-full w-[1px] border-l-app-primary border-l-[1px] border-app-marker absolute"
        ></div>
      </Show>
    </div>
  );
}
