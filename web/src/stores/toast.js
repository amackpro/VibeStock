import { writable } from 'svelte/store';

function createToastStore() {
  const { subscribe, update } = writable([]);

  let id = 0;

  return {
    subscribe,
    
    show: (message, type = 'info', duration = 3000) => {
      const toast = { id: ++id, message, type };
      update(toasts => [...toasts, toast]);
      
      if (duration > 0) {
        setTimeout(() => {
          update(toasts => toasts.filter(t => t.id !== toast.id));
        }, duration);
      }
      
      return toast.id;
    },

    success: (message, duration) => {
      return createToastStore().show(message, 'success', duration);
    },

    error: (message, duration) => {
      return createToastStore().show(message, 'error', duration);
    },

    warning: (message, duration) => {
      return createToastStore().show(message, 'warning', duration);
    },

    dismiss: (id) => {
      update(toasts => toasts.filter(t => t.id !== id));
    },

    clear: () => {
      update(() => []);
    }
  };
}

export const toastStore = createToastStore();
