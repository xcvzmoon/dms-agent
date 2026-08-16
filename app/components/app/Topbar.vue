<script setup lang="ts">
  const version = await useTauriAppGetVersion();
  const colorMode = useColorMode();
  const { isSidebarCollapsed, toggleSidebar } = useLayoutStore();
  const { isRunning, run, stop } = useWatchersStore();

  const isDark = computed({
    get: () => colorMode.value === 'dark',
    set: (_isDark) => (colorMode.preference = _isDark ? 'dark' : 'light'),
  });
</script>

<template>
  <div
    class="border-muted/50 flex h-9 items-center gap-0.5 border-b bg-zinc-50/80 p-1.5 dark:bg-zinc-950/20"
    data-tauri-drag-region
  >
    <div class="ml-18.25 flex items-center gap-2 text-xs">
      <h1>DMS Agent</h1>

      <UBadge
        :label="version"
        size="xs"
        variant="subtle"
      />
    </div>

    <UButton
      :icon="isRunning ? 'i-lucide-square' : 'i-lucide-play'"
      :label="isRunning ? 'Stop Watchers' : 'Run Watchers'"
      size="sm"
      color="neutral"
      variant="ghost"
      class="ml-auto p-1"
      @click="isRunning ? stop() : run()"
    />

    <UButton
      :icon="isSidebarCollapsed ? 'i-lucide-panel-left-open' : 'i-lucide-panel-left-close'"
      size="sm"
      color="neutral"
      variant="ghost"
      class="p-1 text-xs"
      @click="toggleSidebar"
    />

    <UButton
      :icon="isDark ? 'i-lucide-moon' : 'i-lucide-sun'"
      size="sm"
      color="neutral"
      variant="ghost"
      class="p-1"
      @click="isDark = !isDark"
    />
  </div>
</template>
