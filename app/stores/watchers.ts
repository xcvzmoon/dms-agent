type WatchersStore = {
  isRunning: Readonly<Ref<boolean>>;
  run: () => Promise<void>;
  stop: () => Promise<void>;
};

const store = await useTauriStoreLoad('watchers.json', { autoSave: false });
const isRunning = ref<boolean>((await store.get<boolean>('isRunning')) ?? false);

async function persist(): Promise<void> {
  await store.set('isRunning', isRunning.value);
  await store.save();
}

await useTauriEventListen<boolean>('watchers-status-changed', (event) => {
  isRunning.value = event.payload;
  void persist();
});

export function useWatchersStore(): WatchersStore {
  async function run(): Promise<void> {
    await useTauriCoreInvoke('start_watchers');
    isRunning.value = true;
    await persist();
  }

  async function stop(): Promise<void> {
    await useTauriCoreInvoke('stop_watchers');
    isRunning.value = false;
    await persist();
  }

  return {
    isRunning: readonly(isRunning),
    run,
    stop,
  };
}
