// Type declarations for the following MIT-licensed library:
// BassoonTracker v0.4.0 by Steffest - build 2021-04-19 - Full source on https://github.com/steffest/BassoonTracker
declare namespace BassoonTracker {
  function init(initHost: boolean): void;
  function load(url: string, autoPlay: boolean, callback?: () => void): void;
  function isPlaying(): boolean;
  function stop(): void;
}
