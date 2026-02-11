import { useState, useEffect, useCallback } from 'react';
import { check, Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

export type UpdateStatus = 'idle' | 'checking' | 'available' | 'downloading' | 'error';

interface UpdateState {
  status: UpdateStatus;
  version: string | null;
  progress: number;
  error: string | null;
}

export function useUpdateChecker() {
  const [state, setState] = useState<UpdateState>({
    status: 'idle',
    version: null,
    progress: 0,
    error: null,
  });
  const [update, setUpdate] = useState<Update | null>(null);

  useEffect(() => {
    const timer = setTimeout(async () => {
      setState(prev => ({ ...prev, status: 'checking' }));
      try {
        const result = await check();
        if (result) {
          setUpdate(result);
          setState(prev => ({
            ...prev,
            status: 'available',
            version: result.version,
          }));
        } else {
          setState(prev => ({ ...prev, status: 'idle' }));
        }
      } catch (err) {
        console.error('Update check failed:', err);
        // Silently fail on check — don't bother the user
        setState(prev => ({ ...prev, status: 'idle' }));
      }
    }, 3000);

    return () => clearTimeout(timer);
  }, []);

  const downloadAndInstall = useCallback(async () => {
    if (!update) return;
    setState(prev => ({ ...prev, status: 'downloading', progress: 0, error: null }));

    try {
      let totalLength = 0;
      let downloaded = 0;

      await update.downloadAndInstall((event) => {
        if (event.event === 'Started') {
          totalLength = event.data.contentLength ?? 0;
        } else if (event.event === 'Progress') {
          downloaded += event.data.chunkLength;
          if (totalLength > 0) {
            setState(prev => ({
              ...prev,
              progress: Math.round((downloaded / totalLength) * 100),
            }));
          }
        } else if (event.event === 'Finished') {
          setState(prev => ({ ...prev, progress: 100 }));
        }
      });

      await relaunch();
    } catch (err) {
      console.error('Update download failed:', err);
      setState(prev => ({
        ...prev,
        status: 'error',
        error: err instanceof Error ? err.message : String(err),
      }));
    }
  }, [update]);

  const dismiss = useCallback(() => {
    setState({ status: 'idle', version: null, progress: 0, error: null });
    setUpdate(null);
  }, []);

  const retry = useCallback(() => {
    downloadAndInstall();
  }, [downloadAndInstall]);

  return {
    ...state,
    downloadAndInstall,
    dismiss,
    retry,
  };
}
