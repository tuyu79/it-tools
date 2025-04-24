<template>
  <div flex flex-col gap-2>
    <c-button @click='handleGenerateUUID'>
      UUID
    </c-button>
    <div flex flex-row gap-1>
      <c-button @click='handleGenerateTimestamp'>
        Time
      </c-button>
      <c-select
        v-model:value='timeFormat'
        :options='options'
        placeholder='Select a time format'
        w-300px
      />
    </div>
    <div flex flex-row gap-1>
      <c-button @click='handleCopyColor'>
        Color
      </c-button>
      <n-color-picker
        v-model:value='color'
        placement='bottom-end'
      />
    </div>
  </div>
</template>

<script lang='ts' setup>
import CButton from '@/ui/c-button/c-button.vue';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { v4 as generateUuidV4 } from 'uuid';
import { invoke } from '@tauri-apps/api/core';
import { withDefaultOnError } from '@/utils/defaults';
import { formatISO, formatISO9075, format } from 'date-fns';

defineOptions({ name: 'Shortcut.page.vuew' });

const color = ref('');
const timeFormat = ref('');
const options = ref([
  {
    label: 'ISO 8601',
    value: 'ISO 8601',
    formatter: formatISO,
  },
  {
    label: 'ISO 9075',
    value: 'ISO 9075',
    formatter: formatISO9075,
  },
  {
    label: 'yyyy-MM-dd',
    value: 'yyyy-MM-dd',
    formatter: (date: Date) => format(date, 'yyyy-MM-dd'),
  },
]);

const handleGenerateUUID = async () => {
  await writeText(generateUuidV4());
  invoke('paste_at_cursor', { text: 'UUID copied to clipboard' }).catch((e) => {
    console.error('paste error', e);
  });
};

const handleGenerateTimestamp = async () => {
  const selectedOption = options.value.find((option) => option.value === timeFormat.value);
  await writeText(withDefaultOnError(() => selectedOption?.formatter(new Date()), ''));
};

const handleCopyColor = async () => {
  await writeText(color.value);
};

</script>

<style scoped>

</style>