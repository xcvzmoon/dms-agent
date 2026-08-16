type LayoutStore = {
  isSidebarCollapsed: Readonly<Ref<boolean>>;
  toggleSidebar: () => Promise<void>;
};

const store = await useTauriStoreLoad('layout.json', { autoSave: false });
const isSidebarCollapsed = ref<boolean>((await store.get<boolean>('isSidebarCollapsed')) ?? false);

export function useLayoutStore(): LayoutStore {
  async function toggleSidebar(): Promise<void> {
    isSidebarCollapsed.value = !isSidebarCollapsed.value;
    await store.set('isSidebarCollapsed', isSidebarCollapsed.value);
    await store.save();
  }

  return {
    isSidebarCollapsed: readonly(isSidebarCollapsed),
    toggleSidebar,
  };
}
