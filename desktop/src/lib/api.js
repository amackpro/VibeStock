/**
 * api.js — VibeStock API Client
 *
 * Centralizes all HTTP calls to the Rust/Axum backend.
 * Uses fetch() with bearer token from auth store.
 * All functions return parsed JSON or throw with error message.
 */

import { get } from 'svelte/store';
import { authStore }  from '../stores/auth.js';

/** Base URL — can be overridden via Tauri get_config command */
let BASE_URL = 'http://localhost:3000';

/** Read current JWT token from store */
function getToken() {
  return get(authStore).token;
}

/** Get current tenant ID from store */
function getTenantId() {
  return get(authStore).tenant?.id;
}

/** Core fetch wrapper — attaches auth header, throws on non-2xx */
async function request(method, path, body = null) {
  const token = getToken();
  const tenantId = getTenantId();
  const headers = { 'Content-Type': 'application/json' };
  if (token) headers['Authorization'] = `Bearer ${token}`;
  if (tenantId) headers['X-Tenant-ID'] = tenantId;

  const opts = { method, headers };
  if (body) opts.body = JSON.stringify(body);

  const res = await fetch(`${BASE_URL}${path}`, opts);
  const data = await res.json().catch(() => ({}));

  if (!res.ok) {
    throw new Error(data.error || `HTTP ${res.status}`);
  }
  return data;
}

// ── Auth ──────────────────────────────────────────────────────────────────────
export const api = {
  setBase: (url) => { BASE_URL = url; },

  auth: {
    login:    (creds) => request('POST', '/api/auth/login',    creds),
    register: (data)  => request('POST', '/api/auth/register', data),
  },

  // ── Dashboard ───────────────────────────────────────────────────────────────
  dashboard: {
    stats: () => request('GET', '/api/dashboard/stats'),
  },

  // ── Products ────────────────────────────────────────────────────────────────
  products: {
    list:         (params = {}) => request('GET',    `/api/products?${new URLSearchParams(params)}`),
    get:          (id)          => request('GET',    `/api/products/${id}`),
    byBarcode:    (code)        => request('GET',    `/api/products/barcode/${code}`),
    create:       (data)        => request('POST',   '/api/products',     data),
    update:       (id, data)    => request('PUT',    `/api/products/${id}`, data),
    delete:       (id)          => request('DELETE', `/api/products/${id}`),
  },

  // ── Categories ──────────────────────────────────────────────────────────────
  categories: {
    list:   ()          => request('GET',    '/api/categories'),
    create: (data)      => request('POST',   '/api/categories',     data),
    update: (id, data)  => request('PUT',    `/api/categories/${id}`, data),
    delete: (id)        => request('DELETE', `/api/categories/${id}`),
  },

  // ── Suppliers ───────────────────────────────────────────────────────────────
  suppliers: {
    list:   ()          => request('GET',    '/api/suppliers'),
    create: (data)      => request('POST',   '/api/suppliers',     data),
    update: (id, data)  => request('PUT',    `/api/suppliers/${id}`, data),
    delete: (id)        => request('DELETE', `/api/suppliers/${id}`),
  },

  // ── Stock Movements ─────────────────────────────────────────────────────────
  movements: {
    list:   (params = {}) => request('GET',  `/api/movements?${new URLSearchParams(params)}`),
    create: (data)        => request('POST', '/api/movements', data),
  },

  // ── Users (Admin Only) ──────────────────────────────────────────────────────
  users: {
    list:         ()          => request('GET',   '/api/users'),
    updateRole:   (id, role)  => request('PATCH', `/api/users/${id}/role`, { role }),
    toggleStatus: (id, state) => request('PATCH', `/api/users/${id}/status`, { is_active: state }),
  },

  // ── Tenants (Global Admin Only) ───────────────────────────────────────────────
  tenants: {
    list:   ()          => request('GET',   '/api/tenants'),
    get:    (id)        => request('GET',   `/api/tenants/${id}`),
    create: (data)      => request('POST',  '/api/tenants', data),
    update: (id, data)  => request('PUT',  `/api/tenants/${id}`, data),
    delete: (id)        => request('DELETE', `/api/tenants/${id}`),
  },

  // ── Reports ────────────────────────────────────────────────────────────────────
  reports: {
    inventory: (params = {}) => request('GET', `/api/reports/inventory?${new URLSearchParams(params)}`),
    lowStock:  (params = {}) => request('GET', `/api/reports/low-stock?${new URLSearchParams(params)}`),
    movements: (params = {}) => request('GET', `/api/reports/movements?${new URLSearchParams(params)}`),
    valuation: (params = {}) => request('GET', `/api/reports/valuation?${new URLSearchParams(params)}`),
  },
};

// ── WebSocket helper ──────────────────────────────────────────────────────────

/**
 * Opens a WebSocket connection to the API's real-time feed.
 * @param {(event: object) => void} onMessage - called on each parsed event
 * @returns {WebSocket} - caller can close() when done
 */
export function openWebSocket(onMessage) {
  const wsUrl = BASE_URL.replace(/^http/, 'ws') + '/api/ws';
  const ws = new WebSocket(wsUrl);

  ws.onopen    = () => console.info('[WS] VibeStock real-time feed connected');
  ws.onclose   = () => console.info('[WS] Connection closed');
  ws.onerror   = (e) => console.error('[WS] Error', e);
  ws.onmessage = (ev) => {
    try { onMessage(JSON.parse(ev.data)); }
    catch { /* ignore non-JSON */ }
  };

  return ws;
}
