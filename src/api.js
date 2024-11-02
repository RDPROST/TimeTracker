import { invoke } from "@tauri-apps/api/core";

const handleError = (error) => {
	window.notify('error', error);
	console.error('Ошибка:', JSON.stringify(error));
	return null;
};

export const loginRequest = async (username, password, remember = false, notify = false) => {
	console.log(username, password)
	const response = await invoke('authorize', { username, password }).catch(handleError);
	if (!response) return false;

	console.log(localStorage.getItem('username'));
	console.log(localStorage.getItem('password'));

	if (notify) {
		if (remember) {
			localStorage.setItem('username', username);
			localStorage.setItem('password', password);
		}
		else {
			sessionStorage.setItem('username', username);
			sessionStorage.setItem('password', password);
		}

		window.notify('success', 'Успешно авторизован');
	}


	return true;
};


export const getProjects = async () => {
	const username = localStorage.getItem('username') || sessionStorage.getItem('username');
	const password = localStorage.getItem('password') || sessionStorage.getItem('password');

	const response = await invoke('get_projects', { username, password }).catch(handleError);
	return response;
}

export const getTasks = async (projectId) => {
	const username = localStorage.getItem('username') || sessionStorage.getItem('username');
	const password = localStorage.getItem('password') || sessionStorage.getItem('password');

	const response = await invoke('get_tasks', { username, password, projectId }).catch(handleError);
	return response;
}

export const startTimer = async (taskId) => {
	const username = localStorage.getItem('username') || sessionStorage.getItem('username');
	const password = localStorage.getItem('password') || sessionStorage.getItem('password');

	const response = await invoke('start_timer', { username, password, taskId }).catch(handleError);
	if (!response) return false;
	
	return true;
}

export const stopTimer = async () => {
	const username = localStorage.getItem('username') || sessionStorage.getItem('username');
	const password = localStorage.getItem('password') || sessionStorage.getItem('password');

	const response = await invoke('stop_timer', { username, password }).catch(handleError);
	if (!response) return false;

	return true;
}

export const getTimer = async () => {
	const username = localStorage.getItem('username') || sessionStorage.getItem('username');
	const password = localStorage.getItem('password') || sessionStorage.getItem('password');

	const response = await invoke('get_timer', { username, password }).catch(handleError);
	if (!response) return false;

	return response;
}