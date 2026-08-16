<script setup lang="ts">
  import { Result } from 'better-result';

  const { watchLogs } = useWatchLogsStore();
  const { syncLogs } = useSyncLogsStore();
  const toast = useToast();

  const uploadingIds = ref(new Set<string>());

  const unsyncedEntries = computed(() =>
    watchLogs.value
      .filter((entry) => !entry.isSync && (entry.action === 'ADD' || entry.action === 'CHANGE'))
      .toSorted((a, b) => b.createdAt - a.createdAt),
  );

  const recentSyncLogs = computed(() =>
    syncLogs.value.toSorted((a, b) => b.createdAt - a.createdAt).slice(0, 10),
  );

  function fileName(path: string): string {
    return path.split('/').pop() ?? path;
  }

  function formatDate(createdAt: number): string {
    return new Date(createdAt).toLocaleString();
  }

  async function uploadEntry(entry: (typeof unsyncedEntries.value)[number]): Promise<void> {
    uploadingIds.value.add(entry.id);

    const result = await Result.tryPromise({
      try: () =>
        useTauriCoreInvoke<UploadOutcome>('upload_document', {
          item: {
            id: entry.id,
            file: entry.file,
            directoryPath: entry.directoryPath,
            dmsEndpoint: entry.dmsEndpoint,
          },
        }),
      catch: (cause) =>
        new UploadError({ message: cause instanceof Error ? cause.message : String(cause) }),
    });

    uploadingIds.value.delete(entry.id);

    if (Result.isError(result)) {
      toast.add({ title: 'Upload failed', description: result.error.message, color: 'error' });
      return;
    }

    toast.add({ title: 'Success', description: 'File uploaded to DMS.' });
  }
</script>

<template>
  <AppPageContainer class="space-y-4 p-4">
    <div>
      <h1 class="text-2xl font-bold">Documents</h1>
      <h3 class="text-xs">Tracked files waiting to be synced to the DMS</h3>
    </div>

    <div class="space-y-2">
      <UCard
        v-for="entry in unsyncedEntries"
        :key="entry.id"
      >
        <div class="flex items-center justify-between">
          <div>
            <div class="text-sm font-bold">{{ fileName(entry.file) }}</div>
            <div class="text-muted w-96 truncate text-[10px]">{{ entry.file }}</div>
          </div>

          <UButton
            label="Upload"
            icon="i-heroicons-arrow-up-tray"
            size="sm"
            :loading="uploadingIds.has(entry.id)"
            :disabled="uploadingIds.has(entry.id)"
            @click="uploadEntry(entry)"
          />
        </div>
      </UCard>

      <UAlert
        v-show="unsyncedEntries.length === 0"
        color="neutral"
        variant="subtle"
        icon="i-lucide-info"
        title="Nothing to sync"
        description="All tracked files are up to date."
      />
    </div>

    <div class="space-y-2">
      <h2 class="text-sm font-bold">Recently Synced</h2>

      <UCard
        v-for="entry in recentSyncLogs"
        :key="entry.id"
      >
        <div class="flex items-center justify-between">
          <div>
            <div class="text-sm font-medium">{{ entry.fileName }}</div>
            <div class="text-muted text-[10px]">{{ formatDate(entry.createdAt) }}</div>
          </div>

          <UBadge
            color="success"
            label="Synced"
            size="sm"
            variant="soft"
          />
        </div>
      </UCard>

      <UAlert
        v-show="recentSyncLogs.length === 0"
        color="neutral"
        variant="subtle"
        icon="i-lucide-info"
        title="No uploads yet"
        description="Uploaded files will show up here."
      />
    </div>
  </AppPageContainer>
</template>
