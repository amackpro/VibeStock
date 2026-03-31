/**
 * main.js — Svelte application entry point
 * Mounts App.svelte into #app and initialises Tauri config
 */
import './app.css';
import App from './App.svelte';

const app = new App({ target: document.getElementById('app') });

export default app;
