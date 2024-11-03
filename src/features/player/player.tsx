import { For } from "solid-js";
import { TrackItem } from "./components/track/track.model";
import { Track } from "./components/track/track";

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
    <div class="w-full h-full flex flex-col">
      <For each={testData}>
        {(child, i) => (
          <Track {...child} isLast={i() === testData.length - 1}></Track>
        )}
      </For>
    </div>
  );
}
