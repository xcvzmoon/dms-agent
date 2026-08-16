<script setup lang="ts">
  const { isSidebarCollapsed } = useLayoutStore();
</script>

<template>
  <UApp>
    <AppTopbar />

    <div class="flex h-[calc(100svh-2.25rem)]">
      <AppSidebar />

      <AppScrollArea
        :class="isSidebarCollapsed ? 'w-[calc(100svw-4.3125rem)]' : 'w-[calc(100svw-14.4375rem)]'"
        as="main"
        mount="both"
        class="h-[calc(100svh-2.25rem)] transition-[width]"
      >
        <NuxtPage />
      </AppScrollArea>
    </div>
  </UApp>
</template>
