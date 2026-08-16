import type { DeepReadonly } from 'vue';

type WatchLogsStore = {
  watchLogs: Readonly<Ref<DeepReadonly<WatchLogEntry[]>>>;
  removeWatchLog: (id: string) => Promise<void>;
};

const store = await useTauriStoreLoad('watch-logs.json', { autoSave: false });
const watchLogs = ref<WatchLogEntry[]>((await store.get<WatchLogEntry[]>('watchLogs')) ?? []);

async function persist(): Promise<void> {
  await store.set('watchLogs', watchLogs.value);
  await store.save();
}

await useTauriEventListen<WatchLogEntry>('watch-log-created', (event) => {
  watchLogs.value = [...watchLogs.value, event.payload];
});

export function useWatchLogsStore(): WatchLogsStore {
  async function removeWatchLog(id: string): Promise<void> {
    watchLogs.value = watchLogs.value.filter((entry) => entry.id !== id);
    await persist();
  }

  return {
    watchLogs: readonly(watchLogs),
    removeWatchLog,
  };
}
