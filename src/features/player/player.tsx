import { For } from "solid-js";
import { TrackItem } from "./components/track/track.model";
import { Track } from "./components/track/track";
import Clock from "./components/clock/clock";
import Ticker from "./components/ticker/ticker";

export function Player() {
  const testData: TrackItem[] = [
    {
      index: 0,
      displayBuffer: [],
      isMuted: false,
      isSoloed: false,
      reverbWet: 1,
      volume: 1,
    },
    {
      index: 1,
      displayBuffer: [],
      isMuted: false,
      isSoloed: false,
      reverbWet: 1,
      volume: 1,
    },
    {
      index: 2,
      displayBuffer: [],
      isMuted: false,
      isSoloed: false,
      reverbWet: 1,
      volume: 1,
    },
    {
      index: 3,
      displayBuffer: [],
      isMuted: false,
      isSoloed: false,
      reverbWet: 1,
      volume: 1,
    },
  ];
  return (
    <div class="w-full flex flex-col gap-6 relative">
      <div class="absolute top-0 bottom-0 left-0 w-[calc(100%-216px)]">
        <Ticker />
      </div>
      <div class="flex flex-col gap-6 z-10">
        <div class="w-[calc(100%-216px)]">
          <Clock />
        </div>
        <div class="flex flex-col">
          <For each={testData}>
            {(child, i) => (
              <Track {...child} isLast={i() === testData.length - 1}></Track>
            )}
          </For>
        </div>
      </div>
    </div>
  );
}
