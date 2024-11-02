import { createRouter, createWebHistory } from 'vue-router';
import AuthorizationPage from '@/views/AuthorizationPage.vue';
import TimeTrackingPage from '@/views/TimeTrackingPage.vue';
import {loginRequest} from '../api';

const routes = [
	{
		path: '/login',
		name: 'Authorization',
		component: AuthorizationPage
	},
	{
		path: '/time-tracking',
		name: 'TimeTracking',
		component: TimeTrackingPage,
		meta: { requiresAuth: true }
	},
	{
        path: '/',
        redirect: '/time-tracking'
    }
];

const router = createRouter({
	history: createWebHistory(),
	routes
});

// Проверка авторизации перед каждым переходом на защищенные маршруты
router.beforeEach(async (to, from, next) => {
	const username = localStorage.getItem('username') || sessionStorage.getItem('username');
	const password = localStorage.getItem('password') || sessionStorage.getItem('password');

	const isAuthenticated = username && password && await loginRequest(username, password);
	console.log(!isAuthenticated && to.meta.requiresAuth)
	
	if (to.meta.requiresAuth && !isAuthenticated) {
		next({ name: 'Authorization' });
	} else {
		next();
	}
});

export default router;