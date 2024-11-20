import { For } from "solid-js";
import { useAppContext } from "../../../../core/app-state/app.context";

export default function Clock() {
  const [state] = useAppContext();
  return (
    <div class="w-full h-9 border-y-[1px] border-app-primary ">
      <div class="w-full h-full items-center grid grid-flow-col">
        <For each={new Array(state.timeInformation.bars)}>
          {(_, index) => (
            <>
              <For each={new Array(state.timeInformation.beatValue)}>
                {(_, childIndex) => (
                  <span class="text-xs">{`${index() + 1}.${
                    childIndex() + 1
                  }`}</span>
                )}
              </For>
            </>
          )}
        </For>
      </div>
    </div>
  );
}
