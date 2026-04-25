import { writable, get } from 'svelte/store';

export const currentRoute = writable('/');
export const routeParams = writable({});

const routes = {
  '/': 'Login',
  '/dashboard': 'Dashboard',
  '/products': 'Products',
  '/suppliers': 'Suppliers',
  '/movements': 'Movements',
  '/users': 'Users',
  '/reports': 'Reports'
};

export function navigate(path, params = {}) {
  currentRoute.set(path);
  routeParams.set(params);
}

export function getRouteName(path) {
  return routes[path] || 'Unknown';
}

export { routes };
