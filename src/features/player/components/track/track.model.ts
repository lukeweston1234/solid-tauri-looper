export interface TrackItem {
  index: number;
  volume: number;
  isSoloed: boolean;
  isMuted: boolean;
  reverbWet: number;
  displayBuffer: { position: number; buffer: number[] };
}
