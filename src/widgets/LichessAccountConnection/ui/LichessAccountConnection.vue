<template>
  <connect-lichess-account />
</template>

<script setup lang="ts">
// Composite widget: presents a focused dashboard block; reads shared Pinia stores and Tauri invoke where needed.

import { ConnectLichessAccount } from '@/features/ConnectLichessAccount';
</script>
