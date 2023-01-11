import { create } from "zustand";

import { persist, createJSONStorage, devtools } from "zustand/middleware";

interface AuthState {
  sessionKey: string | null;
}

export const useAuthStore = create<AuthState>()(
  devtools(
    persist(
      (set) => ({
        sessionKey: null,
        logout: () => set((state) => ({ sessionKey: null })),
      }),
      { name: "sc-auth-storage" }
    )
  )
);
