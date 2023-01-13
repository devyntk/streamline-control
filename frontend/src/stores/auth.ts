import { create } from "zustand";

import { persist, createJSONStorage, devtools } from "zustand/middleware";

interface AuthState {
  sessionKey: string | null;
  logout: () => void;
  login: (key: string) => void;
}

export const useAuthStore = create<AuthState>()(
  devtools(
    persist(
      (set) => ({
        sessionKey: null,
        logout: () => set((state) => ({ sessionKey: null })),
        login: (key: string) => set(() => ({ sessionKey: key })),
      }),
      { name: "sc-auth-storage" }
    )
  )
);
