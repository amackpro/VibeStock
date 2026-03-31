/**
 * toast.js — Global toast notification store
 *
 * Usage:
 *   import { toast } from '../stores/toast.js';
 *   toast.success('Product saved!');
 *   toast.error('API error: ' + err.message);
 */
import { writable } from 'svelte/store';

const { subscribe, update } = writable([]);

let _id = 0;
const timers = new Map(); // Track timeout IDs for cleanup

function add(message, type = 'success', duration = 3500) {
  const id = ++_id;
  update(list => [...list, { id, message, type }]);
  const timerId = setTimeout(() => remove(id), duration);
  timers.set(id, timerId);
}

function remove(id) {
  // Clear the timeout if it exists
  const timerId = timers.get(id);
  if (timerId !== undefined) {
    clearTimeout(timerId);
    timers.delete(id);
  }
  update(list => list.filter(t => t.id !== id));
}

export const toasts = { subscribe };

export const toast = {
  success: (msg) => add(msg, 'success'),
  error:   (msg) => add(msg, 'error',   4500),
  warning: (msg) => add(msg, 'warning', 4000),
};
