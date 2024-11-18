import { For } from "solid-js";
import { useAppContext } from "../../../../core/app-state/app.context";

export default function AudioVisualization(props: { index: number }) {
  const [state] = useAppContext();

  const downsampledData = state.tracks[props.index].displayBuffer.buffer;

  const maxAmplitude = () => Math.max(...downsampledData.map(Math.abs));

  const normalizedData = () =>
    downsampledData.map((x) => (Math.abs(x) / maxAmplitude()) * 100);

  return (
    <div class="flex h-[64px] w-full items-center justify-between p-3">
      <For each={normalizedData()}>
        {(chunk) => (
          <div
            class="w-[2px] rounded-md bg-app-primary"
            style={{ height: `${chunk}%` }}
          />
        )}
      </For>
    </div>
  );
}
