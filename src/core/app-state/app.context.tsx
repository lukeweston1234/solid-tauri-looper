import { JSXElement, createContext, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { AppContextType, AppState, AppStatus } from "./app.context.model";
import { invoke } from "@tauri-apps/api/core";

export const AppStateContext = createContext<AppContextType>();

export function AppStateProvider(props: { children: JSXElement }) {
  const [state, setState] = createStore<AppState>({
    bpm: 120,
    masterVolume: 1,
    status: "stopped",
    isMetronomeOn: false,
    timeInformation: {
      bars: 2,
      beatValue: 4,
      beatsPerMeasure: 4,
    },
  });

  function setTimeInformation(
    beatsPerMeasure: number,
    beatValue: number,
    bars: number
  ) {
    setState((prevState) => ({
      ...prevState,
      timeInformation: {
        beatsPerMeasure: beatsPerMeasure,
        beatValue: beatValue,
        bars: bars,
      },
    }));
  }

  function toggleMetronome() {
    setState((prevState) => ({
      ...prevState,
      isMetronomeOn: !prevState.isMetronomeOn,
    }));
  }

  async function setStatus(status: AppStatus) {
    if (status === 'playing'){
      await invoke('play')
    }
    if (status === 'paused'){
      await invoke('pause')
    }
    if (status === 'recording'){
      await invoke('start_looping');
    }
    if (status === 'stopped'){
      await invoke('stop')
    }
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
      setTimeSignature: setTimeInformation,
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
