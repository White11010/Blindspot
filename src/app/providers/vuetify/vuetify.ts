import 'vuetify/styles';
import 'unfonts.css';
import '@mdi/font/css/materialdesignicons.css';

import { readStoredLocale } from '@/entities/locale';
import { createVuetify } from 'vuetify';
import type { LocaleMessages } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';
import { aliases, mdi } from 'vuetify/iconsets/mdi';
import { en, ru } from 'vuetify/locale';

const initialVuetifyLocale = readStoredLocale();

export const vuetify = createVuetify({
  components,
  directives,
  locale: {
    locale: initialVuetifyLocale,
    fallback: 'en',
    messages: { en: en as LocaleMessages, ru: ru as LocaleMessages },
  },
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi,
    },
  },
  theme: {
    defaultTheme: 'dark',
    themes: {
      light: {
        colors: {
          primary: '#111827',
          secondary: '#D97706',
        },
      },
      dark: {
        colors: {
          primary: '#F3F4F6',
          secondary: '#F59E0B',
        },
      },
    },
  },
});
