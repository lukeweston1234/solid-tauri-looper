import { getCurrentWindow } from "@tauri-apps/api/window";

export default function Toolbar() {
  async function minimize() {
    console.log("minimize");
    await getCurrentWindow().minimize();
  }

  async function close() {
    await getCurrentWindow().close();
  }
  return (
    <div class="w-full flex justify-end gap-6 items-center p-3">
      <button onClick={minimize}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="size-6 text-app-primary"
        >
          <path stroke-linecap="round" stroke-linejoin="round" d="M5 12h14" />
        </svg>
      </button>

      <button onClick={close}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="size-6 text-app-primary"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M6 18 18 6M6 6l12 12"
          />
        </svg>
      </button>
    </div>
  );
}
