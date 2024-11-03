import "./App.css";
import { AppStateProvider } from "./core/app-state/app.context";
import Header from "./shared/ui/header/header";
import Lighting from "./shared/ui/lighting/lighting";

export default function App() {
  return (
    <AppStateProvider>
      <div class="w-screen h-screen bg-app-background p-8 overflow-hidden flex flex-col gap-6">
        <Lighting />
        <Header />
        <h1 class="text-3xl font-bold underline text-app-primary">
          Hello world!
        </h1>
      </div>
    </AppStateProvider>
  );
}
