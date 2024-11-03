import { useAppContext } from "../../../core/app-state/app.context";
import { createEffect } from "solid-js";

export default function Header() {
  const [appState, { setVolume }] = useAppContext();
  createEffect(() => {
    console.log(appState.masterVolume);
  });
  return (
    <div class="w-full grid-cols-3 grid">
      <div class="flex gap-6">
        <svg
          width="60"
          height="40"
          viewBox="0 0 60 40"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          class="fill-app-primary"
        >
          <g clip-path="url(#clip0_36_738)">
            <path d="M56.25 1.25C56.25 1.94036 56.8096 2.5 57.5 2.5H58.75C59.4404 2.5 60 1.94036 60 1.25C60 0.559645 59.4404 0 58.75 0H57.5C56.8096 0 56.25 0.559645 56.25 1.25Z" />
            <path d="M20 40H26.0723L24.3045 38.2322C23.8357 37.7634 23.1998 37.5 22.5368 37.5H20C10.335 37.5 2.5 29.665 2.5 20C2.5 10.335 10.335 2.50001 20 2.5H40C49.665 2.5 57.5 10.335 57.5 20C57.5 29.665 49.665 37.5 40 37.5H32.5184C31.5238 37.5 30.57 37.1049 29.8667 36.4017L27.7957 34.3306C26.6236 33.1585 25.0338 32.5 23.3762 32.5H20C13.0964 32.5 7.5 26.9036 7.5 20C7.5 13.0964 13.0964 7.5 20 7.5H40C46.9036 7.5 52.5 13.0964 52.5 20C52.5 26.9036 46.9036 32.5 40 32.5H35.1961C34.2015 32.5 33.2477 32.1049 32.5444 31.4017L30.4733 29.3306C29.3012 28.1585 27.7115 27.5 26.0539 27.5H20C15.8579 27.5 12.5 24.1421 12.5 20C12.5 15.8579 15.8579 12.5 20 12.5H40C44.1421 12.5 47.5 15.8579 47.5 20C47.5 24.0916 44.2235 27.418 40.1512 27.4985L40.1504 27.5H38.3211C37.3265 27.5 36.3727 27.1049 35.6694 26.4017L33.5983 24.3306C32.6366 23.3688 31.3937 22.7529 30.0628 22.5628L30 22.5H20C18.6193 22.5 17.5 21.3807 17.5 20C17.5 18.6193 18.6193 17.5 20 17.5H40C41.3807 17.5 42.5 18.6193 42.5 20C42.5 21.3807 41.3807 22.5 40 22.5H35L36.7678 24.2678C37.2366 24.7366 37.8725 25 38.5355 25H40C42.7614 25 45 22.7614 45 20C45 17.2386 42.7614 15 40 15H20C17.2386 15 15 17.2386 15 20C15 22.7614 17.2386 25 20 25H29.1789C30.1735 25 31.1273 25.3951 31.8306 26.0983L33.9017 28.1694C35.0738 29.3415 36.6635 30 38.3211 30H40.625V29.9808C45.8567 29.6582 50 25.3129 50 20C50 14.4772 45.5228 10 40 10H20C14.4772 10 10 14.4772 10 20C10 25.5229 14.4772 30 20 30H26.0539C27.0485 30 28.0023 30.3951 28.7056 31.0983L30.7767 33.1694C31.9488 34.3415 33.5385 35 35.1961 35H40C48.2843 35 55 28.2843 55 20C55 11.7157 48.2843 5 40 5H20C11.7157 5 5 11.7157 5 20C5 28.2843 11.7157 35 20 35H23.3762C24.3708 35 25.3246 35.3951 26.0279 36.0983L28.099 38.1694C29.2711 39.3415 30.8608 40 32.5184 40H40C51.0457 40 60 31.0457 60 20C60 8.9543 51.0457 -9.65645e-07 40 0L20 4.13264e-06C8.9543 5.09829e-06 -9.65645e-07 8.95431 0 20C9.65645e-07 31.0457 8.95431 40 20 40Z" />
          </g>
          <defs>
            <clipPath id="clip0_36_738">
              <rect width="60" height="40" fill="white" />
            </clipPath>
          </defs>
        </svg>
        <div class="flex items-center gap-6">
          <span class="text-sm">MEM 32%</span>
          <span class="text-sm">CPU 24%</span>
          <span class="text-sm">TEMP 54°C</span>
        </div>
      </div>
      <div class="flex gap-6 items-center justify-center">
        <svg
          width="16"
          height="18"
          viewBox="0 0 16 18"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          class="stroke-app-primary"
        >
          <path
            d="M1 2.65308C1 1.79708 1.917 1.25508 2.667 1.66708L14.207 8.01408C14.3836 8.11112 14.531 8.25382 14.6336 8.42727C14.7362 8.60072 14.7903 8.79855 14.7903 9.00008C14.7903 9.20161 14.7362 9.39944 14.6336 9.57289C14.531 9.74634 14.3836 9.88904 14.207 9.98608L2.667 16.3331C2.49569 16.4273 2.30278 16.4752 2.10731 16.4721C1.91184 16.469 1.72054 16.4151 1.55227 16.3155C1.384 16.216 1.24457 16.0744 1.14773 15.9045C1.05089 15.7347 0.99997 15.5426 1 15.3471V2.65308Z"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
        <svg
          width="16"
          height="16"
          viewBox="0 0 16 16"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          class="stroke-app-primary"
        >
          <circle cx="8.29028" cy="7.97217" r="6.75" stroke-width="1.5" />
        </svg>
        <svg
          width="16"
          height="16"
          viewBox="0 0 16 16"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          class="stroke-app-primary"
        >
          <rect
            x="1.54028"
            y="1.22217"
            width="13.5"
            height="13.5"
            rx="1.25"
            stroke-width="1.5"
          />
        </svg>
      </div>
      <div class="flex gap-[72px] items-center justify-end">
        <input
          id="default-range"
          type="range"
          min={0}
          max={1}
          step={0.01}
          onInput={(e) => setVolume(Number(e.target.value))}
          value={appState.masterVolume}
          class="cursor-pointer"
        />
        <div class="flex gap-6">
          <span>{`${appState.bpm} BPM`}</span>
          <span>{`${appState.timeInformation.beatsPerMeasure}/${appState.timeInformation.beatValue}`}</span>
        </div>
      </div>
    </div>
  );
}
