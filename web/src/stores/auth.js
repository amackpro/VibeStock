import { writable, get } from 'svelte/store';

const STORAGE_KEY = 'vibestock_auth';

function createAuthStore() {
  const stored = typeof localStorage !== 'undefined' 
    ? localStorage.getItem(STORAGE_KEY) 
    : null;
  
  const initial = stored ? JSON.parse(stored) : {
    token: null,
    user: null,
    tenant: null,
    isAuthenticated: false
  };

  const { subscribe, set, update } = writable(initial);

  if (typeof localStorage !== 'undefined' && stored) {
    localStorage.setItem(STORAGE_KEY, stored);
  }

  return {
    subscribe,
    
    login: async (token, user, tenant = null) => {
      const state = { token, user, tenant, isAuthenticated: true };
      set(state);
      localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
    },

    logout: () => {
      const state = { token: null, user: null, tenant: null, isAuthenticated: false };
      set(state);
      localStorage.removeItem(STORAGE_KEY);
    },

    updateUser: (userData) => {
      update(s => {
        const newState = { ...s, user: { ...s.user, ...userData } };
        localStorage.setItem(STORAGE_KEY, JSON.stringify(newState));
        return newState;
      });
    },

    setTenant: (tenant) => {
      update(s => {
        const newState = { ...s, tenant };
        localStorage.setItem(STORAGE_KEY, JSON.stringify(newState));
        return newState;
      });
    }
  };
}

export const authStore = createAuthStore();
