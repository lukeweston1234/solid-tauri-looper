import { createSignal, For, Show } from "solid-js";
import { themes, Theme } from "./themes";
export default function ThemePicker() {
  const [isOpen, setIsOpen] = createSignal(false);
  const [activeThemeIndex, setActiveThemeIndex] = createSignal(0);
  let timeoutId: ReturnType<typeof setTimeout> | null = null;

  const handlePointerEnter = () => {
    if (timeoutId) {
      clearTimeout(timeoutId);
      timeoutId = null;
    }
    setIsOpen(true);
  };

  const handlePointerLeave = () => {
    timeoutId = setTimeout(() => {
      setIsOpen(false);
      timeoutId = null;
    }, 2000);
  };

  function setCSSVariables(theme: Theme, index: number) {
    console.log(theme, index);
    setActiveThemeIndex(index);
    const root = document.querySelector(":root") as any;
    root?.style!.setProperty("--primary", theme.primaryColor);
    root?.style!.setProperty("--background", theme.backgroundColor);
    root?.style!.setProperty("--light-left", theme.leftLight);
    root?.style!.setProperty("--light-right", theme.rightLight);
    root?.style!.setProperty("--time-marker", theme.timeMarker);
  }

  return (
    <div
      onPointerEnter={handlePointerEnter}
      onPointerLeave={handlePointerLeave}
      class="absolute bottom-4 left-6 flex gap-1 items-center z-50 cursor-pointer hover:brightness-150 transition-all duration-300"
    >
      <button class="flex gap-1">
        <span class="text-sm">{themes[activeThemeIndex()].name}</span>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="2"
          stroke="currentColor"
          class="size-4 mt-[3px]"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="m8.25 4.5 7.5 7.5-7.5 7.5"
          />
        </svg>
      </button>

      <Show when={isOpen()}>
        <For each={themes.filter((_, i) => i !== activeThemeIndex())}>
          {(theme, i) => (
            <div class="flex items-center gap-1">
              {i() !== 0 && (
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="2"
                  stroke="currentColor"
                  class="size-4 "
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="m8.25 4.5 7.5 7.5-7.5 7.5"
                  />
                </svg>
              )}

              <span
                class="text-sm"
                onClick={() =>
                  setCSSVariables(theme.theme, themes.indexOf(theme))
                }
              >
                {theme.name}
              </span>
            </div>
          )}
        </For>
      </Show>
    </div>
  );
}
