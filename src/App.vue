<script setup>
import FloatLabel from "primevue/floatlabel";
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n"; // Импортируем useI18n

const { t, locale } = useI18n();

const selectedLanguage = ref();

const smartListDisabled = ref(false);

const languages = [
  { name: "Русский", code: "ru" },
  { name: "English", code: "en" },
];

const visible = ref(false);

const saveSettings = () => {
  localStorage.setItem("language", JSON.stringify(selectedLanguage.value));
  localStorage.setItem(
    "smartListDisabled",
    JSON.stringify(smartListDisabled.value)
  );

  locale.value = selectedLanguage.value.code;
  window.notify("success", t("settingsSaved"));
  visible.value = false;
};

onMounted(() => {
  const storedLanguage = localStorage.getItem("language");
  const storedSmartListDisabled = localStorage.getItem("smartListDisabled");

  if (storedLanguage) {
    selectedLanguage.value = JSON.parse(storedLanguage);
  } else {
    localStorage.setItem("language", JSON.stringify(languages[0]));
    selectedLanguage.value = languages[0];
  }

  if (storedSmartListDisabled) {
    smartListDisabled.value = JSON.parse(storedSmartListDisabled);
  } else {
    localStorage.setItem("smartListDisabled", JSON.stringify(false));
    smartListDisabled.value = false;
  }
});
</script>

<template>
  <Toast ref="toast" />
  <button
    @click="visible = true"
    class="p-2 bg-gray-200 rounded-md absolute top-4 left-4"
  >
    <i class="pi pi-cog"></i>
  </button>
  <Dialog
    v-model:visible="visible"
    modal
    :header="$t('settings')"
	:style="{ width: '400px' }"
    @hide="resetSettings"
  >
    <div class="p-4">
      <div class="mb-4">
        <FloatLabel variant="on">
          <Select
            v-model="selectedLanguage"
            :options="languages"
            optionLabel="name"
            class="w-full"
          ></Select>
          <label for="language" class="text-gray-700">{{
            $t("language")
          }}</label>
        </FloatLabel>
      </div>

      <div class="flex items-center mb-4">
        <Checkbox
          v-model="smartListDisabled"
          inputId="smartListToggle"
          :binary="true"
        />
        <label for="smartListToggle" class="ml-2 cursor-pointer">
          {{ $t("smartList") }}
        </label>
      </div>
    </div>

    <template #footer>
      <Button :label="$t('save')" @click="saveSettings" />
      <Button
        :label="$t('cancel')"
        @click="visible = false"
        class="p-button-secondary"
      />
    </template>
  </Dialog>
  <router-view></router-view>
</template>