<script setup lang="ts">
  const { directories } = useDirectoriesStore();
  const { isRunning } = useWatchersStore();
  const { watchLogs, removeWatchLog } = useWatchLogsStore();

  const activeDirectoriesCount = computed(
    () => directories.value.filter((directory) => directory.isActive).length,
  );

  const recentWatchLogs = computed(() =>
    watchLogs.value.toSorted((a, b) => b.createdAt - a.createdAt).slice(0, 10),
  );

  const actionLabel: Record<WatchAction, string> = {
    ADD: 'Added',
    CHANGE: 'Changed',
    UNLINK: 'Removed',
  };

  const actionColor: Record<WatchAction, 'success' | 'info' | 'warning'> = {
    ADD: 'success',
    CHANGE: 'info',
    UNLINK: 'warning',
  };

  function formatDate(createdAt: number): string {
    return new Date(createdAt).toLocaleString();
  }
</script>

<template>
  <AppPageContainer class="space-y-4 p-4">
    <div>
      <h1 class="text-2xl font-bold">Dashboard</h1>
      <h3 class="text-xs">Overview of your tracked directories and file activity</h3>
    </div>

    <div class="grid grid-cols-3 gap-4">
      <UCard>
        <div class="text-muted text-xs">Directories</div>
        <div class="text-2xl font-bold">{{ directories.length }}</div>
      </UCard>

      <UCard>
        <div class="text-muted text-xs">Active Directories</div>
        <div class="text-2xl font-bold">{{ activeDirectoriesCount }}</div>
      </UCard>

      <UCard>
        <div class="text-muted text-xs">Watcher Status</div>
        <UBadge
          :color="isRunning ? 'success' : 'neutral'"
          :label="isRunning ? 'Running' : 'Stopped'"
          variant="soft"
          class="mt-1"
        />
      </UCard>
    </div>

    <div class="space-y-2">
      <h2 class="text-sm font-bold">Recent File Activity</h2>

      <UCard
        v-for="entry in recentWatchLogs"
        :key="entry.id"
      >
        <div class="flex items-center justify-between">
          <div>
            <div class="text-sm font-medium">
              {{ entry.file }}
              <UBadge
                :color="actionColor[entry.action]"
                :label="actionLabel[entry.action]"
                size="sm"
                variant="soft"
              />
            </div>

            <div class="text-muted text-[10px]">{{ formatDate(entry.createdAt) }}</div>
          </div>

          <UButton
            icon="i-heroicons-trash"
            size="sm"
            color="neutral"
            variant="outline"
            @click="removeWatchLog(entry.id)"
          />
        </div>
      </UCard>

      <UAlert
        v-show="recentWatchLogs.length === 0"
        color="neutral"
        variant="subtle"
        icon="i-lucide-info"
        title="No activity yet"
        description="Start watchers to begin tracking file changes."
      />
    </div>
  </AppPageContainer>
</template>
