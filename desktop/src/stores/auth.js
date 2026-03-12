/**
 * auth.js — Authentication Svelte Store
 *
 * Persists login state (token + user info) in localStorage so the
 * session survives app restarts within the Tauri window.
 */
import { writable } from 'svelte/store';

const STORAGE_KEY = 'vibestock_auth';

function loadFromStorage() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    return raw ? JSON.parse(raw) : { token: null, user: null };
  } catch {
    return { token: null, user: null };
  }
}

function createAuthStore() {
  const { subscribe, set, update } = writable(loadFromStorage());

  return {
    subscribe,

    /** Call after successful login API response */
    login(loginResponse) {
      const state = {
        token: loginResponse.token,
        user: {
          id:        loginResponse.user_id,
          username:  loginResponse.username,
          full_name: loginResponse.full_name,
          role:      loginResponse.role,
        },
      };
      localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
      set(state);
    },

    /** Clear session on logout */
    logout() {
      localStorage.removeItem(STORAGE_KEY);
      set({ token: null, user: null });
    },

    /** Check if currently authenticated */
    isLoggedIn() {
      return !!loadFromStorage().token;
    },
  };
}

export const authStore = createAuthStore();
