import { TrackItem } from "../../features/player/components/track/track.model";

export interface TimeInformation {
  bars: number;
  beatsPerMeasure: number; // 6 in 6/8
  beatValue: number; // 2 in 3/2
}

export type AppStatus = "playing" | "stopped" | "recording" | "paused";

export type AppContextType = readonly [
  AppState,
  {
    readonly setStatus: (status: AppStatus) => Promise<void>;
    readonly setBPM: (bpm: number) => void;
    readonly toggleMetronome: () => Promise<void>;
    readonly setVolume: (volume: number) => void;
    readonly setTimeInformation: (
      beatsPerMeasure: number,
      beatValue: number,
      bars: number
    ) => void;
    readonly reset: () => Promise<void>;
  }
];

export interface AppState {
  recordingTrackIndex: number | null;
  bpm: number;
  masterVolume: number;
  status: AppStatus;
  timeInformation: TimeInformation;
  isMetronomeOn: boolean;
  tracks: TrackItem[];
  maxTracks: number;
}
