import { createEffect, createSignal, For } from "solid-js";
import { Track } from "./components/track/track";
import Clock from "./components/clock/clock";
import Ticker from "./components/ticker/ticker";
import { useAppContext } from "../../core/app-state/app.context";

export function Player() {
  const [state] = useAppContext();
  const [numTracks, setNumTracks] = createSignal(0);

  const scrollToBottom = () => {
    let trackContainer = document.getElementById("track-container");
    if (!trackContainer) return;
  
    // Check if container is overflowing
    if (trackContainer.scrollHeight > trackContainer.clientHeight) {
      trackContainer.scrollTop = trackContainer.scrollHeight;
    }
  };

  createEffect(() => {
    if (state.tracks) {
      if (state.tracks.length == 7){
        setNumTracks(0); // Rest at the end so we don't see bottom on new loop
      }
      if (state.tracks.length !== numTracks() && state.tracks.length > 4){
        setNumTracks(state.tracks.length);
        scrollToBottom();
      }
    }
  });

  return (
    <div class="w-full flex flex-1 flex-col gap-6 relative overflow-hidden">
      <div class="absolute top-0 bottom-0 left-0 w-[calc(100%-216px)]">
        <Ticker />
      </div>
      <div class="h-full w-full flex flex-col gap-6 z-10 overflow-y-auto">
        <div class="w-[calc(100%-218px)]">
          <Clock />
        </div>
        <div
          id="track-container"
          class="h-full flex flex-col overflow-y-auto relative"
        >
          <div class="absolute top-0 left-0 h-full w-[calc(100%-218px)] grid grid-flow-col">
            <For
              each={Array(
                state.timeInformation.bars *
                  state.timeInformation.beatsPerMeasure *
                  state.timeInformation.beatValue
              )}
            >
              {(_, i) => (
                <div
                  class={`h-[768px] w-[1px] bg-app-primary ${
                    i() % state.timeInformation.beatsPerMeasure == 0
                      ? "opacity-60"
                      : i() % 2 == 0
                      ? "opacity-30"
                      : "opacity-10"
                  } `}
                ></div>
              )}
            </For>
          </div>

          <For each={state.tracks}>
            {(child, i) => (
              <Track
                {...child}
                isLast={i() === state.tracks.length - 1}
              ></Track>
            )}
          </For>
        </div>
      </div>
    </div>
  );
}
