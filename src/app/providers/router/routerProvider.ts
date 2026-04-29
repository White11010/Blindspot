import { createMemoryHistory, createRouter } from 'vue-router'

import {HomePage} from '@/pages/HomePage'
import {MyGamesPage} from '@/pages/MyGamesPage'


const routes = [
    { path: '/', component: HomePage },
    { path: '/my-games', component: MyGamesPage },
]

export const router = createRouter({
    history: createMemoryHistory(),
    routes,
})