import ConfettiExplosion from "solid-confetti-explosion";

export default () => {
  return (
    <div class="pointer-events-none fixed z-50 flex h-screen w-screen justify-center">
      <ConfettiExplosion
        particlesShape="mix"
        stageWidth={document.documentElement.clientWidth}
        stageHeight={document.documentElement.clientHeight}
        duration={1500}
      />
    </div>
  );
};
