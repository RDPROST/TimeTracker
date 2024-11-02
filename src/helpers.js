export const sortByClicks = (items, clickData) => {
	return items.slice().sort((a, b) => {
		const clicksA = clickData[a.value] || 0;
		const clicksB = clickData[b.value] || 0;
		return clicksB - clicksA;
	});
};


export const highlightItem = (item, clickData) => {
	clickData = clickData == undefined ? {} : clickData;
	return clickData[item.value] > 0 ? "text-green-500 font-bold" : "";
};

export const calculateElapsedTime = (startDate, serverTime, timeZoneOffset) => {
	const start = new Date(startDate);
	const server = new Date(serverTime);

	if (isNaN(start.getTime()) || isNaN(server.getTime())) {
		console.error("Некорректные даты:", startDate, serverTime);
		return 0;
	}

	start.setHours(start.getHours() + timeZoneOffset);

	const elapsedMilliseconds = server.getTime() - start.getTime();
	return Math.floor(elapsedMilliseconds / 1000); // Переводим в секунды
};