import { For } from "solid-js";

export default function AudioVisualization(props: {
  downsampledData: number[];
}) {
  const maxAmplitude = () => Math.max(...props.downsampledData.map(Math.abs));

  const normalizedData = () =>
    props.downsampledData.map((x) => (Math.abs(x) / maxAmplitude()) * 100);

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
