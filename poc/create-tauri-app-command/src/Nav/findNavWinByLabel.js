import { getAll } from '@tauri-apps/api/window';

export default function findNavWinByLabel() {
    const allWindows = getAll();
    // iterate all Windows and find by label
    for (const window of allWindows) {
        if (window.label === 'nav') {
            return window;
        }
    }
}