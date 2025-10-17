<script setup lang="ts">
import type { AuthFormField, FormSubmitEvent } from "@nuxt/ui";
import { z } from "zod";
import { useUserStore } from "~/stores/user";

const userStore = useUserStore();

const fields = ref<AuthFormField[]>([
  {
    name: "email",
    type: "text",
    label: "Email",
  },
  {
    name: "password",
    type: "password",
    label: "Password",
  },
]);

const schema = z.object({
  email: z.email("Invalid email address."),
  password: z
    .string("Password is required.")
    .min(6, "Minimum 6 characters.")
    .max(100),
});

type Schema = z.output<typeof schema>;

const onSubmit = async (values: FormSubmitEvent<Schema>) => {
  // Handle form submission
  userStore.handleLogin(values.data);
};
</script>

<template>
  <main class="flex items-center min-h-dvh">
    <UPageCard class="max-w-md sm:min-w-md mx-auto">
      <UAuthForm
        title="Login"
        description="Enter your credentials to access your account."
        icon="i-lucide-user"
        :fields="fields"
        :schema="schema"
        @submit="onSubmit"
      >
        <template #footer>
          Not registered yet?
          <ULink to="/register" class="text-primary font-medium">Register</ULink
          >.
        </template>
      </UAuthForm>
    </UPageCard>
  </main>
</template>
