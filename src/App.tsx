import "./App.css";
import Lighting from "./shared/ui/lighting/lighting";

export default function App() {
  return (
    <div class="w-screen h-screen bg-app-background">
      <Lighting />
      <h1 class="text-3xl font-bold underline text-app-primary">
        Hello world!
      </h1>
    </div>
  );
}
