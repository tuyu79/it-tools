<script setup lang='ts'>
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

import { RouterView, useRoute } from 'vue-router';
import { NGlobalStyle, NMessageProvider, NNotificationProvider, darkTheme } from 'naive-ui';
import { darkThemeOverrides, lightThemeOverrides } from './themes';
import { layouts } from './layouts';
import { useStyleStore } from './stores/style.store';
import CButton from '@/ui/c-button/c-button.vue';
import { listen } from '@tauri-apps/api/event';

const route = useRoute();
const layout = computed(() => route?.meta?.layout ?? layouts.base);
const styleStore = useStyleStore();

const theme = computed(() => (styleStore.isDarkTheme ? darkTheme : null));
const themeOverrides = computed(() => (styleStore.isDarkTheme ? darkThemeOverrides : lightThemeOverrides));

const { locale } = useI18n();

syncRef(
  locale,
  useStorage('locale', locale),
);

const handleCreateNewWindow = async () => {
  const existedOne = await WebviewWindow.getByLabel('my-label');
  if (existedOne) {
    console.log('existed one');
    await existedOne.setFocus();
    return;
  }

  const webview = new WebviewWindow('my-label', {
    url: '/shortcut',
    // x: 200,
    // y: 200,
    center: true,
    width: 300,
    height: 400,
    alwaysOnTop: true,
  });

  webview.once('tauri://created', function() {
    // webview successfully created
    webview.show();
  });
  webview.once('tauri://error', function(e) {
    // an error happened creating the webview
    console.error('error when creating webview', e);
  });

  const focusListener = await webview.listen('tauri://focus', function(e) {
    console.log('focus', e);
  });
  webview.onCloseRequested((e) => {
    focusListener();
    console.log('close requested', e);
  });
};

onMounted(() => {
  listen('open-shortcut', handleCreateNewWindow);
});
</script>

<template>
  <n-config-provider :theme='theme' :theme-overrides='themeOverrides'>
    <NGlobalStyle />
    <NMessageProvider placement='bottom'>
      <NNotificationProvider placement='bottom-right'>
        <component :is='layout'>
          <c-button @click='handleCreateNewWindow'>
            Create new window
          </c-button>
          <RouterView />
        </component>
      </NNotificationProvider>
    </NMessageProvider>
  </n-config-provider>
</template>

<style>
body {
  min-height: 100%;
  margin: 0;
  padding: 0;
}

html {
  height: 100%;
  margin: 0;
  padding: 0;
}

* {
  box-sizing: border-box;
}
</style>
