import "./App.css";
import { AppStateProvider } from "./core/app-state/app.context";
import { Player } from "./features/player/player";
import Header from "./shared/ui/header/header";
import Lighting from "./shared/ui/lighting/lighting";

export default function App() {
  return (
    <AppStateProvider>
      <div class="w-screen h-screen bg-app-background p-8 overflow-hidden flex flex-col gap-6">
        <Lighting />
        <Header />
        <Player />
      </div>
    </AppStateProvider>
  );
}
