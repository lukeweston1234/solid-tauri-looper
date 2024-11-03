import { createEffect, createSignal } from "solid-js";
import { TrackItem } from "./track.model";
import AudioVisualization from "../audio-visualization/audio-visualization";

export function Track(props: TrackItem & { isLast: boolean }) {
  const [state, setState] = createSignal<TrackItem>({ ...props });
  const displayIndex = () =>
    state().index > 9 ? state().index : `0${state().index}`;

  return (
    <div class="w-full h-32 flex gap-6 items-center">
      <div
        class={`w-full h-full relative flex flex-col justify-center border-t-2 border-app-primary ${
          props.isLast && "border-b-2"
        }`}
      >
        <span class="absolute left-3 top-3">{displayIndex()}</span>
        <AudioVisualization
          downsampledData={[
            44, 13, 82, 72, 26, 4, 22, 32, 86, 39, 98, 5, 6, 10, 86, 17, 28, 83,
            23, 35, 47, 78, 59, 79, 41, 54, 39, 47, 93, 11, 25, 4, 72, 8, 83,
            28, 17, 46, 48, 22, 24, 79, 49, 9, 67, 3, 76, 98, 7, 76, 96, 52, 91,
            18, 88, 33, 99, 48, 69, 8, 99, 70, 97, 80, 47, 86, 61, 64, 11, 1, 9,
            1, 66, 22, 98, 28, 44, 26, 88, 98, 11, 51, 59, 60, 5, 75, 5, 7, 10,
            52, 23, 24, 88, 45, 46, 75, 68, 43, 8, 58, 40, 20, 6, 33, 0, 49, 42,
            77, 91, 51, 62, 88, 2, 88, 88, 44, 58, 78, 10, 87, 31, 62, 31, 98,
            57, 4,
          ]}
        ></AudioVisualization>
        <button class="absolute right-3 top-3">
          <svg
            width="20"
            height="20"
            viewBox="0 0 20 20"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            class="stroke-app-primary"
          >
            <path
              d="M13 10H7M19 10C19 11.1819 18.7672 12.3522 18.3149 13.4442C17.8626 14.5361 17.1997 15.5282 16.364 16.364C15.5282 17.1997 14.5361 17.8626 13.4442 18.3149C12.3522 18.7672 11.1819 19 10 19C8.8181 19 7.64778 18.7672 6.55585 18.3149C5.46392 17.8626 4.47177 17.1997 3.63604 16.364C2.80031 15.5282 2.13738 14.5361 1.68508 13.4442C1.23279 12.3522 1 11.1819 1 10C1 7.61305 1.94821 5.32387 3.63604 3.63604C5.32387 1.94821 7.61305 1 10 1C12.3869 1 14.6761 1.94821 16.364 3.63604C18.0518 5.32387 19 7.61305 19 10Z"
              stroke-width="1.5"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
        </button>
      </div>
      <div class="h-full flex gap-6 justify-between items-center">
        <div class="w-[26px] h-full flex items-center justify-center">
          <input class="-rotate-90 w-[96px]" type="range"></input>
        </div>
        <div class="flex flex-col gap-3">
          <button class="w-14 h-6 border-2 border-app-primary flex items-center justify-center">
            S
          </button>
          <button class="w-14 h-6 border-2 border-app-primary flex items-center justify-center">
            M
          </button>
        </div>
        <button>test</button>
      </div>
    </div>
  );
}
