<script setup>
import { ref } from "vue";
import { loginRequest } from "../api";
import { useRouter } from "vue-router";

const login = ref("");
const password = ref("");
const rememberMe = ref(false);

const router = useRouter();

const handleLogin = async () => {
  let isAuthorized = await loginRequest(
    login.value,
    password.value,
    rememberMe.value,
    true
  );

  if (isAuthorized) {
    router.push({ name: "TimeTracking" });
  }
};
</script>

<template>
  <main
    class="flex flex-col items-center justify-center min-h-screen bg-gray-100"
  >
    <h1 class="text-3xl font-bold mb-8 text-center">{{ $t("title") }}</h1>
    <div class="w-full max-w-md p-6 bg-white rounded-lg shadow-md">
      <div class="mb-4">
        <InputText
          v-model="login"
          :placeholder="$t('login')"
          class="w-full p-3 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-400"
        />
      </div>
      <div class="mb-6">
        <Password
          v-model="password"
          :placeholder="$t('password')"
          :feedback="false"
          class="w-full"
          inputClass="w-full p-3 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-400"
          toggleMask
        />
      </div>
      <div class="flex items-center mb-6">
        <Checkbox
          v-model="rememberMe"
          inputId="rememberMe"
          name="rememberMe"
          :binary="true"
        />
        <label for="rememberMe" class="ml-2 cursor-pointer"> {{ $t("rememberMe") }} </label>
      </div>
      <Button
        @click="handleLogin"
        class="w-full py-3 text-white bg-blue-600 rounded-md hover:bg-blue-700 transition duration-200"
      >
        {{ $t("loginButton") }}
      </Button>
    </div>
  </main>
</template>