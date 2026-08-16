import type { DeepReadonly } from 'vue';

type DirectoriesStore = {
  directories: Readonly<Ref<DeepReadonly<Directory[]>>>;
  addDirectory: (directory: Omit<Directory, 'id'>) => Promise<void>;
  updateDirectory: (id: string, patch: Partial<Directory>) => Promise<void>;
  toggleActive: (id: string, isActive: boolean) => Promise<void>;
  removeDirectory: (id: string) => Promise<void>;
};

const store = await useTauriStoreLoad('directories.json', { autoSave: false });
const directories = ref<Directory[]>((await store.get<Directory[]>('directories')) ?? []);

async function persist(): Promise<void> {
  await store.set('directories', directories.value);
  await store.save();
}

export function useDirectoriesStore(): DirectoriesStore {
  async function addDirectory(directory: Omit<Directory, 'id'>): Promise<void> {
    directories.value = [...directories.value, { ...directory, id: crypto.randomUUID() }];
    await persist();
  }

  async function updateDirectory(id: string, patch: Partial<Directory>): Promise<void> {
    directories.value = directories.value.map((directory) =>
      directory.id === id ? { ...directory, ...patch } : directory,
    );
    await persist();
  }

  async function toggleActive(id: string, isActive: boolean): Promise<void> {
    await updateDirectory(id, { isActive });
  }

  async function removeDirectory(id: string): Promise<void> {
    directories.value = directories.value.filter((directory) => directory.id !== id);
    await persist();
  }

  return {
    directories: readonly(directories),
    addDirectory,
    updateDirectory,
    toggleActive,
    removeDirectory,
  };
}
