import { createApp } from "vue";
import App from "./App.vue";
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import 'unfonts.css'
import {router} from '@/app/providers/router'

const vuetify = createVuetify({
    components,
    directives,
})

createApp(App).use(router).use(vuetify).mount("#app");
