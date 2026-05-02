import 'vuetify/styles';
import 'unfonts.css';
import '@mdi/font/css/materialdesignicons.css'; // Ensure you are using css-loader

import { createApp } from 'vue';
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';
import { aliases, mdi } from 'vuetify/iconsets/mdi';

import { router } from '@/app/providers/router';

import App from './App.vue';
import { pinia } from './app/providers/store';

const vuetify = createVuetify({
  components,
  directives,
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi,
    },
  },
});

createApp(App).use(pinia).use(router).use(vuetify).mount('#app');

const loader = document.getElementById('preloader');
if (loader) loader.remove();
