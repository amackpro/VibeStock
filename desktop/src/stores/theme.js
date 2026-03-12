import { writable } from 'svelte/store';

const THEME_KEY = 'vibestock_theme';

function createThemeStore() {
  // Check local storage or system preference
  let initial = 'dark';
  if (typeof window !== 'undefined') {
    const saved = localStorage.getItem(THEME_KEY);
    if (saved === 'light' || saved === 'dark') {
      initial = saved;
    } else if (window.matchMedia && window.matchMedia('(prefers-color-scheme: light)').matches) {
      // Default to light if system is light, else dark
      initial = 'light';
    }
  }

  const { subscribe, set, update } = writable(initial);

  return {
    subscribe,
    setTheme: (theme) => {
      if (typeof window !== 'undefined') {
        localStorage.setItem(THEME_KEY, theme);
        document.documentElement.setAttribute('data-theme', theme);
      }
      set(theme);
    },
    toggle: () => {
      update(current => {
        const next = current === 'dark' ? 'light' : 'dark';
        if (typeof window !== 'undefined') {
          localStorage.setItem(THEME_KEY, next);
          document.documentElement.setAttribute('data-theme', next);
        }
        return next;
      });
    },
    init: () => {
      if (typeof window !== 'undefined') {
        document.documentElement.setAttribute('data-theme', initial);
      }
    }
  };
}

export const themeStore = createThemeStore();
