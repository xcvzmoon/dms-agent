<script setup lang="ts">
  import type { FormSubmitEvent, TabsItem } from '@nuxt/ui';

  const { directories, addDirectory, toggleActive, removeDirectory } = useDirectoriesStore();
  const toast = useToast();

  const isOpenForm = ref(false);
  const searchKeyword = ref('');
  const filter = useRouteQuery<'ALL' | 'ACTIVE' | 'IN_ACTIVE'>('filter', 'ALL');

  const formPayload = reactive({
    path: '',
    dmsEndpoint: '',
    includePatterns: '',
    isActive: true,
    lastSync: null,
  });

  const tabItems: TabsItem[] = [
    { label: 'All', value: 'ALL' },
    { label: 'Active', value: 'ACTIVE' },
    { label: 'In-Active', value: 'IN_ACTIVE' },
  ];

  const filteredDirectories = computed(() =>
    directories.value.filter((directory) => {
      if (filter.value === 'ACTIVE' && !directory.isActive) return false;
      if (filter.value === 'IN_ACTIVE' && directory.isActive) return false;
      return directory.path.toLowerCase().includes(searchKeyword.value.toLowerCase());
    }),
  );

  async function pickFolder(): Promise<void> {
    const selected = await useTauriDialogOpen({ directory: true });
    if (typeof selected === 'string') formPayload.path = selected;
  }

  function resetForm(): void {
    formPayload.path = '';
    formPayload.dmsEndpoint = '';
    formPayload.includePatterns = '';
    formPayload.isActive = true;
    formPayload.lastSync = null;
  }

  async function submit(event: FormSubmitEvent<Directory>): Promise<void> {
    await addDirectory(event.data);
    isOpenForm.value = false;
    resetForm();
    toast.add({ title: 'Success', description: 'Directory has been registered!' });
  }

  async function onRemove(id: string): Promise<void> {
    await removeDirectory(id);
    toast.add({ title: 'Success', description: 'Directory has been removed!' });
  }
</script>

<template>
  <AppPageContainer class="space-y-4 p-4">
    <USlideover v-model:open="isOpenForm">
      <template #header> Add New Directory </template>

      <template #body>
        <UForm
          :schema="directorySchema"
          :state="formPayload"
          id="directory-form"
          class="space-y-4"
          @submit="submit"
        >
          <UFormField
            label="Select Directory"
            name="path"
          >
            <div class="flex items-center justify-between gap-2">
              <div class="flex-1 truncate text-xs">
                {{ formPayload.path || '---' }}
              </div>

              <UButton
                color="neutral"
                variant="outline"
                @click="pickFolder"
              >
                Select Folder
              </UButton>
            </div>
          </UFormField>

          <UFormField
            label="DMS Server Endpoint"
            name="dmsEndpoint"
          >
            <UInput
              v-model="formPayload.dmsEndpoint"
              class="w-full"
              placeholder="https://your.dms.endpoint/"
            />
          </UFormField>

          <UFormField
            label="File Types to Sync"
            name="includePatterns"
          >
            <UInput
              v-model="formPayload.includePatterns"
              class="w-full"
              placeholder="*.jpg, *.pdf"
            />
          </UFormField>
        </UForm>
      </template>

      <template #footer>
        <div class="flex w-full justify-end gap-2">
          <UButton
            color="neutral"
            variant="outline"
            label="Cancel"
            @click="isOpenForm = false"
          />

          <UButton
            label="Submit"
            type="submit"
            form="directory-form"
          />
        </div>
      </template>
    </USlideover>

    <div class="flex items-start justify-between">
      <div>
        <h1 class="text-2xl font-bold">Directory Management</h1>
        <h3 class="text-xs">Configure directories for monitoring</h3>
      </div>

      <UButton
        icon="i-heroicons-plus"
        label="Directory"
        @click="isOpenForm = true"
      />
    </div>

    <div class="flex items-end justify-between">
      <UInput
        v-model="searchKeyword"
        placeholder="Search Directory"
      />

      <UTabs
        v-model="filter"
        :items="tabItems"
        variant="link"
      />
    </div>

    <div class="space-y-2">
      <UCard
        v-for="directory in filteredDirectories"
        :key="directory.id"
      >
        <div class="item-center flex justify-between">
          <div>
            <div class="text-sm font-bold">
              <p>{{ directory.path }}</p>

              <UBadge
                :color="directory.isActive ? 'success' : 'warning'"
                :label="directory.isActive ? 'Active' : 'Inactive'"
                size="sm"
                variant="soft"
              />
            </div>

            <div class="w-96 truncate text-[10px]">
              Endpoint: {{ directory.dmsEndpoint || '-' }}
            </div>
          </div>

          <div class="flex items-center gap-2">
            <UButton
              v-if="!directory.isActive"
              size="sm"
              variant="outline"
              icon="i-heroicons-play"
              @click="toggleActive(directory.id ?? '', true)"
            />

            <UButton
              v-else
              size="sm"
              variant="outline"
              icon="i-heroicons-pause"
              @click="toggleActive(directory.id ?? '', false)"
            />

            <UButton
              size="sm"
              color="warning"
              variant="outline"
              icon="i-heroicons-trash"
              @click="onRemove(directory.id ?? '')"
            />
          </div>
        </div>
      </UCard>

      <UAlert
        v-show="filteredDirectories.length === 0"
        color="warning"
        variant="subtle"
        icon="i-lucide-info"
        title="Heads up!"
        description="No Data found."
      />
    </div>
  </AppPageContainer>
</template>
