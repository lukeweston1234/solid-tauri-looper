import { JSXElement, createContext, onCleanup, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { AppContextType, AppState, AppStatus } from "./app.context.model";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { TrackItem } from "../../features/player/components/track/track.model";

export const AppStateContext = createContext<AppContextType>();

const VISUALIZER_CHUNK_SIZE = 128;

export function AppStateProvider(props: { children: JSXElement }) {
  const [state, setState] = createStore<AppState>({
    recordingTrackIndex: 0,
    bpm: 120,
    masterVolume: 1,
    status: "stopped",
    isMetronomeOn: false,
    maxTracks: 6,
    timeInformation: {
      bars: 2,
      beatValue: 4,
      beatsPerMeasure: 4,
    },
    tracks: [
      // {
      //   index: 0,
      //   displayBuffer: {
      //     position: 0,
      //     buffer: Array(VISUALIZER_CHUNK_SIZE).fill(0),
      //   },
      //   isMuted: false,
      //   isSoloed: false,
      //   reverbWet: 0.5,
      //   volume: 1,
      // },
    ],
  });

  const trackAddedListener = listen("track_added", (event) => {
    const recordingIndex = event.payload as number;
    console.log("track index", recordingIndex);
    if (recordingIndex >= state.maxTracks) return;
    const tracks: TrackItem[] = [
      ...state.tracks,
      {
        index: recordingIndex,
        displayBuffer: {
          position: 0,
          buffer: Array(VISUALIZER_CHUNK_SIZE).fill(0),
        },
        reverbWet: 0,
        isMuted: false,
        isSoloed: false,
        volume: 1,
      },
    ];
    setState((prevState) => ({
      ...prevState,
      recordingTrackIndex: recordingIndex,
      tracks: tracks,
    }));
  });

  const visualizerSampleListener = listen("visualizer_sample", (event) => {
    const displaySample = event.payload as number;

    let recordingIndex = state.recordingTrackIndex;
    if (recordingIndex === null) throw new Error("No recording track index!");
    console.log("track index", recordingIndex);

    setState((prevState) => {
      const tracks = [...prevState.tracks];
      const updatedTrack = { ...tracks[recordingIndex] };

      const newPosition =
        (updatedTrack.displayBuffer.position + 1) % VISUALIZER_CHUNK_SIZE; // just in case

      const newBuffer = [...updatedTrack.displayBuffer.buffer];
      newBuffer[updatedTrack.displayBuffer.position] = displaySample;

      updatedTrack.displayBuffer = {
        position: newPosition,
        buffer: newBuffer,
      };

      tracks[recordingIndex] = updatedTrack;

      return { ...prevState, tracks };
    });
  });

  onCleanup(() => {
    trackAddedListener.then((unlisten) => unlisten());
    visualizerSampleListener.then((unlisten) => unlisten());
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

  async function toggleMetronome() {
    console.log(state.isMetronomeOn);
    if (state.isMetronomeOn) {
      await invoke("stop_metronome");
    } else {
      await invoke("start_metronome");
    }
    setState((prevState) => ({
      ...prevState,
      isMetronomeOn: !prevState.isMetronomeOn,
    }));
  }

  async function setStatus(status: AppStatus) {
    console.log(status);
    if (status === "playing") {
      await invoke("play");
    }
    if (status === "paused") {
      await invoke("pause");
    }
    if (status === "recording") {
      await invoke("start_looping");
    }
    if (status === "stopped") {
      await invoke("stop");
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

  async function reset() {
    setState((prevState) => ({
      ...prevState,
      status: "stopped",
      tracks: [],
    }));
    await invoke("reset");
  }

  const appState = [
    state,
    {
      setStatus,
      setBPM,
      toggleMetronome,
      setVolume,
      setTimeInformation,
      reset,
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
