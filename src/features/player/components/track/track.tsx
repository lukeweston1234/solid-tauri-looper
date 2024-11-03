import { createSignal } from "solid-js";
import { TrackItem } from "./track.model";
import AudioVisualization from "../audio-visualization/audio-visualization";

export function Track(props: TrackItem) {
  const [state, setState] = createSignal<TrackItem>({ ...props });
  const displayIndex = () =>
    state().index > 9 ? state().index : `0${state().index}`;

  return (
    <div class="w-full h-32 border-y-2 border-app-primary relative flex items-center">
      <span class="absolute left-3 top-3">{displayIndex()}</span>
      <AudioVisualization
        downsampledData={[
          44, 13, 82, 72, 26, 4, 22, 32, 86, 39, 98, 5, 6, 10, 86, 17, 28, 83,
          23, 35, 47, 78, 59, 79, 41, 54, 39, 47, 93, 11, 25, 4, 72, 8, 83, 28,
          17, 46, 48, 22, 24, 79, 49, 9, 67, 3, 76, 98, 7, 76, 96, 52, 91, 18,
          88, 33, 99, 48, 69, 8, 99, 70, 97, 80, 47, 86, 61, 64, 11, 1, 9, 1,
          66, 22, 98, 28, 44, 26, 88, 98, 11, 51, 59, 60, 5, 75, 5, 7, 10, 52,
          23, 24, 88, 45, 46, 75, 68, 43, 8, 58, 40, 20, 6, 33, 0, 49, 42, 77,
          91, 51, 62, 88, 2, 88, 88, 44, 58, 78, 10, 87, 31, 62, 31, 98, 57, 4,
        ]}
      ></AudioVisualization>

      <div class="h-full flex gap-6 justify-between items-center">
        <div class="w-[26px] h-full flex items-center justify-center">
          <input class="-rotate-90 w-[96px]" type="range"></input>
        </div>
        <div class="flex flex-col gap-3">
          <button class="w-14 h-6 border-2 border-app-primary">S</button>
          <button class="w-14 h-6 border-2 border-app-primary">M</button>
        </div>
        <button>test</button>
      </div>
    </div>
  );
}
