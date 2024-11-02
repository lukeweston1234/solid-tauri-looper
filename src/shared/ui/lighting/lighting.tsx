export default function Lighting() {
  return (
    <div class="w-screen h-screen inset-0 pointer-events-none z-50 absolute">
      <div class="w-full h-full relative pointer-events-none">
        <div class="lighting-left pointer-events-none" />
        <div class="lighting-right pointer-events-none" />
      </div>
    </div>
  );
}
