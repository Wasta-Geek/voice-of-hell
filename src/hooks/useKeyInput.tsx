import { useEffect, useRef } from 'react';
import { listen } from '@tauri-apps/api/event';
import { KeyboardStateType } from '@/types/KeyboardState';


// Hook for using keyboard (callback given will be called for each keyboard state changes)
export function useKeyboard(callback: null | ((keyboardState: KeyboardStateType) => void)) {
    const keyboardState = useRef<KeyboardStateType>()

    useEffect(() => {
        const unlisten = listen<KeyboardStateType>('keyboard_state_changed', (event) => {
            const data = event.payload;

            if (data != keyboardState.current) {
                if (callback)
                    callback(data);
                keyboardState.current = data;
            }
        });

        return () => {
            // Unlisten is a promise that return the unlistener
            unlisten.then((fn) => fn());
        }
    }, [callback]);
};