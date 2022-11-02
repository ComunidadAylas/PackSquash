// Silence false-positive error messages for imports handled by Vite
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import songUrl from "../assets/konami/theme.xm";
import bassoonTrackerLibUrl from "../lib/bassoonplayer?url";
import { toast } from "solid-toast";

const konamiCodeKeys = [
  "arrowup",
  "arrowup",
  "arrowdown",
  "arrowdown",
  "arrowleft",
  "arrowright",
  "arrowleft",
  "arrowright",
  "b",
  "a"
];

const activationMessages = [
  "Qrzbfprar zbqr npgvingrq!",
  "Oevat vg ba!",
  "Fdhnfuvat bire 9000!",
  "Cngpuvat Zvarpensg jvgu orggre pbzcerffvba fpurzrf ivn Ybt4Furyy rkcybvgf!",
  "Yrg'f unir fbzr sha!",
  "Rnfgre rtt va yrff guna 80 XvO!",
  "Znk gur tnzr, zvk gur sha!",
  "Vg'f bayl bxnl gb purng sbe guvf!"
];

function toggleKonamiCode() {
  if (!BassoonTracker.isPlaying()) {
    // Oddly enough, reloading the song each time is faster than loading it once
    // and restarting playback
    BassoonTracker.load(songUrl, true);

    const now = new Date();
    const activationMessage =
      now.getDate() == 22 && now.getMonth() == 11
        ? "Happy birthday!"
        : decode(
            activationMessages[
              Math.floor(Math.random() * activationMessages.length)
            ]
          );

    toast(
      `${activationMessage}\n\n🎵 ${decode(
        "Dhnmne bs Fnakvba - Shaxl Fgnef (Uloevq fbat)"
      )}`
    );

    document.documentElement.classList.add("font-mono", "retro-color-scheme");
  } else {
    BassoonTracker.stop();

    document.documentElement.classList.remove(
      "font-mono",
      "retro-color-scheme"
    );
  }
}

export default () => {
  let keySequenceIndex = 0;

  document.addEventListener("keydown", (event) => {
    if (konamiCodeKeys[keySequenceIndex] == event.key.toLowerCase()) {
      if (++keySequenceIndex == konamiCodeKeys.length) {
        if (typeof BassoonTracker == "undefined") {
          // We could save DOM modifications by using indirect eval like this:
          // eval?.(await import("../lib/bassoonplayer?raw").default);
          // However, promises mess up the browser heuristics for allowing
          // automatic sound playback, as we will no longer execute in the
          // context of an interaction event, so it won't work properly
          const scriptElement = document.createElement("script");
          scriptElement.src = bassoonTrackerLibUrl;
          scriptElement.onload = () => {
            BassoonTracker.init(true);
            toggleKonamiCode();
            scriptElement.remove();
          };

          document.body.append(scriptElement);
        } else {
          toggleKonamiCode();
        }

        keySequenceIndex = 0;
      }
    } else {
      keySequenceIndex = 0;
    }
  });
};

function decode(message: string) {
  return message.replace(/[a-zA-Z]/g, (char) =>
    String.fromCharCode(
      char.charCodeAt(0) + (char.toLowerCase() <= "m" ? 13 : -13)
    )
  );
}
