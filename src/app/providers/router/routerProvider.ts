import { createMemoryHistory, createRouter } from 'vue-router'

import {HomePage} from '@/pages/HomePage'
import {MyGamesPage} from '@/pages/MyGamesPage'


const routes = [
    { path: '/', name: 'Home', component: HomePage },
    { path: '/my-games', name: 'MyGames', component: MyGamesPage },
]

export const router = createRouter({
    history: createMemoryHistory(),
    routes,
})