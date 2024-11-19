import "./App.css";
import { AppStateProvider } from "./core/app-state/app.context";
import { Player } from "./features/player/player";
import Header from "./shared/ui/header/header";
import Lighting from "./shared/ui/lighting/lighting";
import ThemePicker from "./shared/ui/theme/themePicker";
import { onMount } from "solid-js";
import { emit } from "@tauri-apps/api/event";
import Toolbar from "./shared/ui/toolbar/Toolbar";

export default function App() {
  onMount(async () => {
    console.log("emitting app ready");
    setTimeout(async () => {
      await emit("app_ready");
    }, 500);
  });
  return (
    <AppStateProvider>
      <div class="w-screen h-screen ">
        <div class="fixed inset-0 z-0">
          <div class="w-full h-full relative">
            <div class="z-0 absolute inset-0 bg-app-background opacity-60 touch-none pointer-events-none" />
          </div>
        </div>

        <div class="fixed inset-0 z-50 p-8 pb-12 overflow-clip flex flex-col gap-6">
          <div class="w-full absolute left-0 top-0">
            <Toolbar />
          </div>
          <Lighting />
          <Header />
          <Player />
          <ThemePicker />
        </div>
      </div>
      <div
        id="container"
        class="touch-none pointer-events-none inset-0 z-[2000]"
      >
        <div class="scanlines "></div>
      </div>
    </AppStateProvider>
  );
}
