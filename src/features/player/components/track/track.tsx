import { TrackItem } from "./track.model";
import AudioVisualization from "../audio-visualization/audio-visualization";
import Knob from "../../../../shared/ui/knob/knob";

export function Track(props: TrackItem & { isLast: boolean }) {
  return (
    <div class="w-full h-32 flex gap-6 items-center flex-shrink-0">
      <div
        class={`w-full h-full relative flex flex-col justify-center border-t-2 border-app-primary ${
          props.isLast && "border-b-2"
        }`}
      >
        <span class="absolute left-3 top-3">{props.index}</span>
        <AudioVisualization
          downsampledData={props.displayBuffer.buffer}
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
        <Knob
          initialValue={0.33}
          debounceTime={200}
          onValueChange={(x) => console.log(x)}
        ></Knob>
      </div>
    </div>
  );
}
