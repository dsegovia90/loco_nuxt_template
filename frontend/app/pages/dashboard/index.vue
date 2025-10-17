<script setup lang="ts">
import { useUserStore } from "~/stores/user";

const userStore = useUserStore();
</script>

<template>
  <div>
    <h1>Welcome, {{ userStore.user?.name }}!</h1>
    <UButton @click="userStore.handleLogout">Logout</UButton>
  </div>
</template>
