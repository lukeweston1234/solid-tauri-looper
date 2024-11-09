import { createEffect, For } from "solid-js";
import { Track } from "./components/track/track";
import Clock from "./components/clock/clock";
import { useAppContext } from "../../core/app-state/app.context";

export function Player() {
  const [state] = useAppContext();

  const scrollToBottom = () => {
    let trackContainer = document.getElementById("track-container");
    if (!trackContainer) return;
    trackContainer.scrollTop = trackContainer.scrollHeight;
  };

  createEffect(() => {
    if (state.tracks) {
      scrollToBottom();
    }
  });

  return (
    <div class="w-full flex flex-1 flex-col gap-6 relative overflow-hidden">
      {/* <div class="absolute top-0 bottom-0 left-0 w-[calc(100%-216px)]">
        <Ticker />
      </div> */}
      <div class="h-full w-full flex flex-col gap-6 z-10 overflow-y-auto">
        <div class="w-[calc(100%-218px)]">
          <Clock />
        </div>
        <div id="track-container" class="h-auto flex flex-col overflow-y-auto">
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
