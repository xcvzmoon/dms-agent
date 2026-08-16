<script setup lang="ts">
  const { isSidebarCollapsed } = useLayoutStore();
  const route = useRoute();

  const links = [
    { label: 'Dashboard', icon: 'i-lucide-layout-dashboard', to: '/' },
    { label: 'Directories', icon: 'i-lucide-folder-cog', to: '/directories' },
    { label: 'Documents', icon: 'i-lucide-file-text', to: '/documents' },
    { label: 'About', icon: 'i-lucide-info', to: '/about' },
  ];
</script>

<template>
  <aside
    :class="isSidebarCollapsed ? 'w-17.25' : 'w-57.75'"
    class="border-muted/50 bg-muted/25 block border-r p-4 transition-[width]"
  >
    <ul class="space-y-2">
      <li v-for="link in links">
        <UTooltip
          :text="link.label"
          :disabled="!isSidebarCollapsed"
          :delay-duration="0"
          :content="{
            align: 'center',
            side: 'right',
            sideOffset: 8,
          }"
          arrow
        >
          <UButton
            :color="link.to === route.path ? 'primary' : 'neutral'"
            :variant="link.to === route.path ? 'solid' : 'ghost'"
            :to="link.to"
            :icon="link.icon"
            :label="link.label"
            class="w-full"
          />
        </UTooltip>
      </li>
    </ul>
  </aside>
</template>
