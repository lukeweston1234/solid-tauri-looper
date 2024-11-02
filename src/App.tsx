import "./App.css";
import Header from "./shared/ui/header/header";
import Lighting from "./shared/ui/lighting/lighting";

export default function App() {
  return (
    <div class="w-screen h-screen bg-app-background p-6 overflow-hidden">
      <Lighting />
      <Header />
      <h1 class="text-3xl font-bold underline text-app-primary">
        Hello world!
      </h1>
    </div>
  );
}
