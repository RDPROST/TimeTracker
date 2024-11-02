<script setup>
import { ref, onMounted, computed, watch } from "vue";
import { getProjects, getTasks, startTimer, stopTimer, getTimer } from "../api";
import { FilterMatchMode, FilterService } from "@primevue/core/api";
import { sortByClicks, highlightItem, calculateElapsedTime } from "../helpers";

const groupedProjects = ref([]);
const selectedProject = ref();
const filteredProjects = ref([]);
const disabledProject = ref(true);

const groupedTasks = ref([]);
const selectedTask = ref();
const filteredTasks = ref([]);
const disabledTask = ref(true);

const isTruncationEnabled = ref(true);
const truncateLength = ref(40);
const timeZoneOffset = ref(3);

// --- Функции работы с localStorage ---
const updateClickData = (type, id) => {
  const clickData = getClickData();
  if (!clickData[type]) {
    clickData[type] = {};
  }
  clickData[type][id] = (clickData[type][id] || 0) + 1;
  localStorage.setItem("clickData", JSON.stringify(clickData));
};

const getClickData = () => {
  return JSON.parse(localStorage.getItem("clickData")) || {};
};

// --- Функции для фильтрации проектов и задач ---
const searchProject = (event) => {
  let query = event.query;
  let newfilteredProjects = [];

  for (let project of groupedProjects.value) {
    let filteredItems = FilterService.filter(
      project.items,
      ["label"],
      query,
      FilterMatchMode.CONTAINS
    );
    if (filteredItems && filteredItems.length) {
      newfilteredProjects.push({ ...project, ...{ items: filteredItems } });
    }
  }

  filteredProjects.value = newfilteredProjects;
};

const searchTask = (event) => {
  let query = event.query;
  let newfilteredTasks = [];

  for (let task of groupedTasks.value) {
    let filteredItems = FilterService.filter(
      task.items,
      ["label"],
      query,
      FilterMatchMode.CONTAINS
    );
    if (filteredItems && filteredItems.length) {
      newfilteredTasks.push({ ...task, ...{ items: filteredItems } });
    }
  }

  filteredTasks.value = newfilteredTasks;
};

// --- Загрузка и сортировка проектов и задач ---
const loadProjects = async () => {
  try {
    disabledProject.value = true;

    const projects = await getProjects();
    const clickData = getClickData().projects || {};
    const grouped = {};

    projects.forEach((project) => {
      const orgId = project.org_id;
      const orgName = project.org_name;
      if (!grouped[orgName]) {
        grouped[orgName] = {
          label: orgName,
          code: orgId,
          items: [],
        };
      }
      grouped[orgName].items.push({
        label: project.name,
        value: project.id,
      });
    });

    groupedProjects.value = Object.values(grouped)
      .map((group) => ({
        ...group,
        items: sortByClicks(group.items, clickData),
      }))
      .sort((a, b) => {
        const maxClicksA = Math.max(
          ...a.items.map((item) => clickData[item.value] || 0)
        );
        const maxClicksB = Math.max(
          ...b.items.map((item) => clickData[item.value] || 0)
        );
        return maxClicksB - maxClicksA;
      });
  } catch (error) {
    console.error("Ошибка загрузки проектов:", error);
  }
};

const loadTasks = async () => {
  try {
    disabledTask.value = true;
    selectedTask.value = null;
    const tasks = await getTasks(selectedProject.value.value);
    const clickData = getClickData().tasks || {};
    const grouped = {};

    tasks.forEach((task) => {
      const phaseId = task.phace_id;
      const phaseName = task.phase_name;
      if (!grouped[phaseName]) {
        grouped[phaseName] = {
          label: phaseName,
          code: phaseId,
          items: [],
        };
      }
      grouped[phaseName].items.push({
        label: task.name,
        value: task.id,
      });
      grouped[phaseName].counter = grouped[phaseName].items.length;
    });

    groupedTasks.value = Object.values(grouped)
      .map((group) => ({
        ...group,
        items: sortByClicks(group.items, clickData),
      }))
      .sort((a, b) => {
        const maxClicksA = Math.max(
          ...a.items.map((item) => clickData[item.value] || 0)
        );
        const maxClicksB = Math.max(
          ...b.items.map((item) => clickData[item.value] || 0)
        );
        return maxClicksB - maxClicksA;
      });

    disabledTask.value = false;
  } catch (error) {
    console.error("Ошибка загрузки задач:", error);
  }
};

// --- Обработчик выбора проекта и задачи ---
const selectProject = (project) => {
  selectedProject.value = project.value;
  updateClickData("projects", project.value.value);
  loadTasks();
};

const selectTask = (task) => {
  selectedTask.value = task.value;
  updateClickData("tasks", task.value.value);
};

const truncateLabel = (label) => {
  if (isTruncationEnabled.value && label.length > truncateLength.value) {
    return label.substring(0, truncateLength.value) + "...";
  }
  return label;
};

let timer = null;

const isRunning = ref(false);
const elapsedTime = ref(0);

const formattedTime = computed(() => {
  const hours = String(Math.floor(elapsedTime.value / 3600)).padStart(2, "0");
  const minutes = String(Math.floor((elapsedTime.value % 3600) / 60)).padStart(
    2,
    "0"
  );
  const seconds = String(elapsedTime.value % 60).padStart(2, "0");
  return `${hours}:${minutes}:${seconds}`;
});

const toggleTimer = async () => {
  if (isRunning.value) {
    const stopTimerResult = await stopTimer();
    if (stopTimerResult) {
      clearInterval(timer);
      isRunning.value = false;
    }
  } else {
    const startTimerResult = await startTimer(selectedTask.value.value);
    if (startTimerResult) {
      disabledProject.value = true;
      disabledTask.value = true;
      timer = setInterval(() => {
        elapsedTime.value++;
      }, 1000);
      isRunning.value = true;
    }
  }
};

const handleGetTimer = async () => {
  const { timer: timerData, servertime } = await getTimer();

  if (timerData && timerData.task_id) {
    elapsedTime.value = calculateElapsedTime(timerData.start_date, servertime, timeZoneOffset.value); // Вычисляем прошедшее время
    isRunning.value = true;
    timer = setInterval(() => {
      elapsedTime.value++;
    }, 1000);

    const project = groupedProjects.value
      .find((project) => project.code == timerData.org_id)
      .items.find((project) => project.value == timerData.project_id);

    selectedProject.value = project;
    disabledProject.value = true;

    await loadTasks();
    disabledTask.value = true;

    const task = groupedTasks.value
      .flatMap((taskGroup) => taskGroup.items) // Разворачиваем группы задач в единый массив
      .find((task) => task.value === timerData.task_id);

    selectedTask.value = task;
  } else {
    disabledProject.value = false;
  }
};

// Остановка таймера при уничтожении компонента
watch(isRunning, (newValue) => {
  if (!newValue) {
    clearInterval(timer);
    elapsedTime.value = 0; // Сбрасываем таймер, если остановлен

    disabledProject.value = false;
    disabledTask.value = false;
  }
});

onMounted(async () => {
  await loadProjects();
  handleGetTimer();
});
</script>

<template>
  <main
    class="flex flex-col items-center min-h-screen justify-center p-6"
  >
  <slot></slot>
    <div class="w-full max-w-md">
      <h1 class="text-5xl font-semibold text-gray-800 mb-16 text-center">
        {{ $t("timer") }}
      </h1>
      <div class="mb-4">
        <FloatLabel variant="on">
          <AutoComplete
            v-model="selectedProject"
            :suggestions="filteredProjects"
            :disabled="disabledProject"
            @complete="searchProject"
            optionLabel="label"
            optionGroupLabel="label"
            optionGroupChildren="items"
            inputId="projectAutocomplete"
            dropdown
            @option-select="selectProject"
            class="w-full border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 p-2"
            :emptySearchMessage="$t('emptySearchMessage')"
          >
            <template #option="slotProps">
              <p
                v-tooltip="slotProps.option.label"
                :class="
                  highlightItem(slotProps.option, getClickData().projects)
                "
              >
                {{ truncateLabel(slotProps.option.label) }}
              </p>
            </template>
            <template #optiongroup="slotProps">
              <p v-tooltip="slotProps.option.label">
                {{ truncateLabel(slotProps.option.label) }}
              </p>
            </template>
          </AutoComplete>
          <label for="projectAutocomplete" class="text-gray-700">{{
            $t("project")
          }}</label>
        </FloatLabel>
      </div>
      <div class="mb-4">
        <FloatLabel variant="on">
          <AutoComplete
            v-model="selectedTask"
            :disabled="disabledTask"
            :suggestions="filteredTasks"
            @complete="searchTask"
            optionLabel="label"
            optionGroupLabel="label"
            optionGroupChildren="items"
            inputId="taskAutocomplete"
            dropdown
            @option-select="selectTask"
            class="w-full border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 p-2"
            :emptySearchMessage="$t('emptySearchMessage')"
          >
            <template #option="slotProps">
              <p
                v-tooltip="slotProps.option.label"
                :class="highlightItem(slotProps.option, getClickData().tasks)"
              >
                {{ truncateLabel(slotProps.option.label) }}
              </p>
            </template>
            <template #optiongroup="slotProps">
              <p
                v-tooltip="
                  `${slotProps.option.label} (${slotProps.option.counter})`
                "
              >
                {{ truncateLabel(slotProps.option.label) }}
              </p>
            </template>
          </AutoComplete>
          <label for="taskAutocomplete" class="text-gray-700">{{
            $t("task")
          }}</label>
        </FloatLabel>
      </div>

      <div
        class="flex items-center justify-between mt-4 bg-gray-100 rounded-lg p-4"
      >
        <p class="text-3xl font-semibold text-gray-800">{{ formattedTime }}</p>
        <Button
          @click="toggleTimer"
          class="py-2 px-4 flex items-center text-white bg-blue-600 rounded-md hover:bg-blue-700 transition duration-200"
        >
          <i :class="isRunning ? 'pi pi-stop' : 'pi pi-play'" class="mr-2"></i>
          {{ isRunning ? $t("stop") : $t("start") }}
        </Button>
      </div>
    </div>
  </main>
</template>