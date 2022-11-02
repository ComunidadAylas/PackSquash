import { os } from "@tauri-apps/api";
import { OsType } from "@tauri-apps/api/os";

let osFamily: OsType | undefined;
try {
  osFamily = await os.type();
} catch {
  // Ignore error
}

export default osFamily;
