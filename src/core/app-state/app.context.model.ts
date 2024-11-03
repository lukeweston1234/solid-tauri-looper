export interface TimeSignature {
  beatsPerMeasure: number; // 6 in 6/8
  beatValue: number; // 2 in 3/2
}

export type AppStatus = "playing" | "stopped" | "recording" | "paused";

export type AppContextType = readonly [
  AppState,
  {
    readonly setStatus: (status: AppStatus) => void;
    readonly setBPM: (bpm: number) => void;
    readonly toggleMetronome: () => void;
    readonly setVolume: (volume: number) => void;
    readonly setTimeSignature: (
      beatsPerMeasure: number,
      beatValue: number
    ) => void;
  }
];

export interface AppState {
  bpm: number;
  masterVolume: number;
  status: AppStatus;
  timeSignature: TimeSignature;
  isMetronomeOn: boolean;
}
