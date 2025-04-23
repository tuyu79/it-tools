<template>
  <div>
    <c-button @click='handleGenerateUUID'>
      UUID
    </c-button>
    <c-button @click='handleGenerateTimestamp'>
      Timestamp
    </c-button>
  </div>
</template>

<script lang='ts' setup>
import CButton from '@/ui/c-button/c-button.vue';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { v4 as generateUuidV4 } from 'uuid';
import { invoke } from '@tauri-apps/api/core';

defineOptions({ name: 'Shortcut.page.vuew' });

const handleGenerateUUID = async () => {
  await writeText(generateUuidV4());
  invoke('paste_at_cursor', { text: 'UUID copied to clipboard' }).catch((e) => {
    console.error("paste error",e);
  });
};

const handleGenerateTimestamp = () => {
  console.log('generate timestamp');
};

</script>

<style scoped>

</style>