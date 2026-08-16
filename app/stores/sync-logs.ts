import type { DeepReadonly } from 'vue';

type SyncLogsStore = {
  syncLogs: Readonly<Ref<DeepReadonly<SyncLogEntry[]>>>;
};

const store = await useTauriStoreLoad('sync-logs.json', { autoSave: false });
const syncLogs = ref<SyncLogEntry[]>((await store.get<SyncLogEntry[]>('syncLogs')) ?? []);

await useTauriEventListen<SyncLogEntry>('sync-log-created', (event) => {
  syncLogs.value = [...syncLogs.value, event.payload];
});

export function useSyncLogsStore(): SyncLogsStore {
  return {
    syncLogs: readonly(syncLogs),
  };
}
