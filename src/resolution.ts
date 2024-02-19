export enum Resolution {
  FHD = "FHD",
  HD = "HD",
}
type Size = { width: number; height: number };

export const resolutionConfig: Record<Resolution, Size> = {
  [Resolution.FHD]: { width: 1920, height: 1080 },
  [Resolution.HD]: { width: 1280, height: 720 },
};
