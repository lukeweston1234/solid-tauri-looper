import "./App.css";
import { AppStateProvider } from "./core/app-state/app.context";
import { Player } from "./features/player/player";
import Header from "./shared/ui/header/header";
import Lighting from "./shared/ui/lighting/lighting";
import ThemePicker from "./shared/ui/theme/themePicker";

export default function App() {
  return (
    <AppStateProvider>
      <div class="w-screen relative h-screen bg-app-background p-8 overflow-clip flex flex-col gap-6">
        <Lighting />
        <Header />
        <Player />
        <ThemePicker />
      </div>
      <div id="container" class="touch-none pointer-events-none inset-0">
        <div class="scanlines"></div>
      </div>
    </AppStateProvider>
  );
}
