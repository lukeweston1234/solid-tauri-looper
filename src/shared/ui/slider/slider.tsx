export default function Slider({
  min,
  max,
}: {
  min: number;
  max: number;
  initialValue: number;
  direction: "x" | "y";
  onSlide: (num: Number) => void;
}) {
  return (
    <div class="-rotate-90">
      <label
        for="default-range"
        class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
      >
        Default range
      </label>
      <input
        id="default-range"
        type="range"
        min={min}
        max={max}
        onInput={(e) => console.log(e.target.value)}
        value="50"
      />
    </div>
  );
}
