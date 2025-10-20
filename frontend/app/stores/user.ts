import type { CurrentResponse } from "~/bindings/CurrentResponse";
import type { MagicLinkParams } from "~/bindings/MagicLinkParams";
import type { LoginParams } from "~/bindings/LoginParams";
import type { LoginResponse } from "~/bindings/LoginResponse";
import type { RegisterParams } from "~/bindings/RegisterParams";
import { api } from "~/utils/api";
import { defineStore } from "pinia";

interface User {
  token: string;
  pid: string;
  name: string;
  email: string;
}

export const useUserStore = defineStore("user", () => {
  const route = useRoute();
  const router = useRouter();
  const user = ref<User | null>(null);
  const loading = ref(false);
  const toast = useToast();

  onMounted(() => {
    fetchCurrentUser();
  });

  const fetchCurrentUser = async () => {
    await api<CurrentResponse>("/api/auth/current")
      .then((response) => {
        user.value = {
          token: response.token,
          pid: response.pid,
          name: response.name,
          email: response.email,
        };
      })
      .catch((error) => {
        setUser(null);
        console.log(error);
      })
      .finally(() => {
        loading.value = true;
      });
  };

  const setUser = (newUser: User | null) => {
    user.value = newUser;
    window?.localStorage.setItem("token", newUser?.token || "");
  };

  const handleRegister = async (registerParams: RegisterParams) => {
    await api<LoginResponse, RegisterParams>("/api/auth/register", {
      method: "POST",
      body: registerParams,
    })
      .then(setUser)
      .catch((error) => {
        console.log(error);
      });
  };

  const handleLogin = async (loginParams: LoginParams) => {
    await api<LoginResponse, LoginParams>("/api/auth/login", {
      method: "POST",
      body: loginParams,
    })
      .then(setUser)
      .catch((error) => {
        console.log(error);
      });
  };

  const handleMagicLink = async (
    magicLinkParams: MagicLinkParams,
    sideEffect: () => void,
  ) => {
    await api<unknown, MagicLinkParams>("/api/auth/magic-link", {
      method: "POST",
      body: magicLinkParams,
    })
      .then(() => {
        toast.add({
          title: "Magic link sent!",
          icon: "lucide:mail",
        });
      })
      .catch((error) => {
        toast.add({
          title: "Error sending magic link!",
          icon: "lucide:mail-x",
        });
        console.log(error);
      })
      .finally(sideEffect);
  };

  const verifyMagicLink = async (token: string) => {
    await api<LoginResponse, MagicLinkParams>(`/api/auth/magic-link/${token}`)
      .then(setUser)
      .catch((error) => {
        toast.add({
          title: "Error verifying magic link!",
          description: "Please try again.",
          icon: "lucide:mail-x",
        });
        router.push("/");
        console.log(error);
      });
  };

  const handleLogout = async () => {
    await api("/api/auth/logout").catch((error) => {
      console.log(error);
    });
    setUser(null);
  };

  watch([user, loading], ([user, loading]) => {
    if (!loading) {
      return;
    }

    if (!user && route.path.startsWith("/dashboard")) {
      router.push("/");
    } else if (user && !route.path.startsWith("/dashboard")) {
      router.push("/dashboard");
    }
  });

  return {
    user,
    loading,
    fetchCurrentUser,
    handleLogin,
    handleMagicLink,
    handleLogout,
    handleRegister,
    verifyMagicLink,
  };
});
