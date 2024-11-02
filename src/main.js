import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from 'primevue/config';
import Material from '@primevue/themes/material';
import { createI18n } from 'vue-i18n';
import router from "./router";

import InputText from 'primevue/inputtext';
import FloatLabel from "primevue/floatlabel";
import Password from 'primevue/password';
import Button from "primevue/button";
import ToastService from 'primevue/toastservice';
import Toast from 'primevue/toast'; 
import Card from 'primevue/card';
import Checkbox from 'primevue/checkbox';
import AutoComplete from "primevue/autocomplete";
import Tooltip from 'primevue/tooltip';
import Dialog from "primevue/dialog";
import Select from 'primevue/select';

import 'primeicons/primeicons.css';

import en from './locales/en.json';
import ru from './locales/ru.json';

const i18n = createI18n({
	locale: 'ru', // язык по умолчанию
	fallbackLocale: 'ru',
	messages: {
		en,
		ru
	}
});

const app = createApp(App)
	.use(i18n)
	.use(router)
	.use(PrimeVue, {
		theme: {
			preset: Material
		}
	})
	.use(ToastService)
	.directive('tooltip', Tooltip)
	.component("InputText", InputText)
	.component("FloatLabel", FloatLabel)
	.component("Password", Password)
	.component("Button", Button)
	.component('Toast', Toast)
	.component('Card', Card)
	.component('Checkbox', Checkbox)
	.component('Dialog', Dialog)
	.component('Select', Select)
	.component('AutoComplete', AutoComplete);

app.config.globalProperties.$notify = (severity, summary, detail, life) => {
	// Проверяем, существует ли $toast
	if (app.config.globalProperties.$toast) {
		app.config.globalProperties.$toast.add({ severity, summary, detail, life });
	} else {
		console.error('$toast не доступен');
	}
};
window.notify = (severity, detail, life = 3000, summary) => {
	summary = summary || i18n.global.t(severity);

	app.config.globalProperties.$notify(severity, summary, detail, life);
};

app.mount("#app");
