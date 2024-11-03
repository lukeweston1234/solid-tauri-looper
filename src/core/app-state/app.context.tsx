import { JSXElement, createContext, createEffect, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { AppContextType, AppState, AppStatus } from "./app.context.model";

export const AppStateContext = createContext<AppContextType>();

export function AppStateProvider(props: { children: JSXElement }) {
  const [state, setState] = createStore<AppState>({
    bpm: 120,
    masterVolume: 1,
    status: "stopped",
    isMetronomeOn: false,
    timeSignature: {
      beatsPerMeasure: 4,
      beatValue: 4,
    },
  });

  function setTimeSignature(beatsPerMeasure: number, beatValue: number) {
    setState((prevState) => ({
      ...prevState,
      timeSignature: {
        beatsPerMeasure: beatsPerMeasure,
        beatValue: beatValue,
      },
    }));
  }

  function toggleMetronome() {
    setState((prevState) => ({
      ...prevState,
      isMetronomeOn: !prevState.isMetronomeOn,
    }));
  }

  function setStatus(status: AppStatus) {
    setState((prevState) => ({
      ...prevState,
      status: status,
    }));
  }

  function setBPM(bpm: number) {
    setState((prevState) => ({
      ...prevState,
      bpm: bpm,
    }));
  }

  function setVolume(volume: number) {
    setState((prevState) => ({
      ...prevState,
      masterVolume: Math.min(Math.max(volume, 0), 1),
    }));
  }

  const appState = [
    state,
    {
      setStatus,
      setBPM,
      toggleMetronome,
      setVolume,
      setTimeSignature,
    },
  ] as const;

  return (
    <AppStateContext.Provider value={appState}>
      {props.children}
    </AppStateContext.Provider>
  );
}

export function useAppContext() {
  const context = useContext(AppStateContext);
  if (!context) {
    throw new Error("AppContext was not provided");
  }
  return context;
}
