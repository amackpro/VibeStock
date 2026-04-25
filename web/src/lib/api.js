/**
 * api.js — VibeStock API Client
 */

import { get } from 'svelte/store';
import { authStore } from '../stores/auth.js';

let BASE_URL = '/api';

export function checkConnection() {
  return fetch(`${BASE_URL}/health`, { 
    method: 'GET'
  })
    .then(res => res.ok)
    .catch(() => false);
}

function getToken() {
  return get(authStore).token;
}

function getTenantId() {
  return get(authStore).tenant?.id;
}

async function request(method, path, body = null) {
  const token = getToken();
  const tenantId = getTenantId();
  const headers = { 'Content-Type': 'application/json' };
  if (token) headers['Authorization'] = `Bearer ${token}`;
  if (tenantId) headers['X-Tenant-ID'] = tenantId;

  const opts = { method, headers };
  if (body) opts.body = JSON.stringify(body);

  try {
    const res = await fetch(`${BASE_URL}${path}`, opts);
    const data = await res.json().catch(() => ({}));

    if (!res.ok) {
      throw new Error(data.error || `HTTP ${res.status}`);
    }
    return data;
  } catch (err) {
    if (err.name === 'TypeError' && err.message.includes('fetch')) {
      throw new Error('Unable to connect to server. Please ensure the API server is running on port 3000.');
    }
    if (err.name === 'AbortError') {
      throw new Error('Request timed out. Please try again.');
    }
    throw err;
  }
}

export const api = {
  setBase: (url) => { BASE_URL = url; },

  auth: {
    login:    (creds) => request('POST', '/auth/login',    creds),
    register: (data)  => request('POST', '/auth/register', data),
  },

  dashboard: {
    stats: () => request('GET', '/dashboard/stats'),
  },

  products: {
    list:    (params = {}) => request('GET', `/products?${new URLSearchParams(params)}`),
    get:     (id)          => request('GET', `/products/${id}`),
    byBarcode: (code)      => request('GET', `/products/barcode/${code}`),
    create:  (data)        => request('POST', '/products', data),
    update:  (id, data)    => request('PUT', `/products/${id}`, data),
    delete:  (id)          => request('DELETE', `/products/${id}`),
  },

  categories: {
    list:    ()            => request('GET', '/categories'),
    create:  (data)        => request('POST', '/categories', data),
    update:  (id, data)    => request('PUT', `/categories/${id}`, data),
    delete:  (id)          => request('DELETE', `/categories/${id}`),
  },

  suppliers: {
    list:    (params = {}) => request('GET', `/suppliers?${new URLSearchParams(params)}`),
    create:  (data)        => request('POST', '/suppliers', data),
    update:  (id, data)    => request('PUT', `/suppliers/${id}`, data),
    delete:  (id)          => request('DELETE', `/suppliers/${id}`),
  },

  movements: {
    list:    (params = {}) => request('GET', `/movements?${new URLSearchParams(params)}`),
    create:  (data)        => request('POST', '/movements', data),
  },

  users: {
    list:         ()         => request('GET', '/users'),
    updateRole:   (id, role) => request('PATCH', `/users/${id}/role`, { role }),
    toggleStatus: (id, state) => request('PATCH', `/users/${id}/status`, { is_active: state }),
  },

  tenants: {
    list:    ()          => request('GET', '/tenants'),
    get:     (id)        => request('GET', `/tenants/${id}`),
    create:  (data)      => request('POST', '/tenants', data),
    update:  (id, data)  => request('PUT', `/tenants/${id}`, data),
    delete:  (id)        => request('DELETE', `/tenants/${id}`),
  },

  reports: {
    inventory: (params = {}) => request('GET', `/reports/inventory?${new URLSearchParams(params)}`),
    lowStock:  (params = {}) => request('GET', `/reports/low-stock?${new URLSearchParams(params)}`),
    movements: (params = {}) => request('GET', `/reports/movements?${new URLSearchParams(params)}`),
    valuation: (params = {}) => request('GET', `/reports/valuation?${new URLSearchParams(params)}`),
  },

  geography: {
    regions:           () => request('GET', '/geography/regions'),
    countriesByRegion: (id) => request('GET', `/geography/regions/${id}/countries`),
    citiesByCountry:   (id) => request('GET', `/geography/countries/${id}/cities`),
    city:              (id) => request('GET', `/geography/cities/${id}`),
    regionStats:       (id) => request('GET', `/geography/regions/${id}/stats`),
    countryStats:      (id) => request('GET', `/geography/countries/${id}/stats`),
    cityDashboard:     (id) => request('GET', `/geography/cities/${id}/dashboard`),
    citiesWithInventory: () => request('GET', '/geography/cities-with-inventory'),
  },
};

export function openWebSocket(onMessage) {
  let wsUrl;
  if (BASE_URL.startsWith('http')) {
    wsUrl = BASE_URL.replace(/^http/, 'ws') + '/ws';
  } else {
    // Relative URL like '/api'
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    wsUrl = `${protocol}//${window.location.host}${BASE_URL}/ws`;
  }
  
  const ws = new WebSocket(wsUrl);

  ws.onopen    = () => console.info('[WS] Connected');
  ws.onclose   = () => console.info('[WS] Closed');
  ws.onerror   = (e) => console.error('[WS] Error', e);
  ws.onmessage = (ev) => {
    try { onMessage(JSON.parse(ev.data)); }
    catch {}
  };

  return ws;
}
