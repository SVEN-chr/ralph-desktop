import { writable, derived } from 'svelte/store';
import type { Theme } from '../types';

// Current theme setting
export const themeSetting = writable<Theme>('system');

// Actual applied theme (resolved from system preference if needed)
export const appliedTheme = derived(themeSetting, ($setting) => {
  if ($setting === 'system') {
    if (typeof window !== 'undefined') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    return 'light';
  }
  return $setting;
});

// Initialize theme from config
export function initTheme(theme: Theme) {
  themeSetting.set(theme);
  applyTheme(theme);
}

// Set and apply theme
export function setTheme(theme: Theme) {
  themeSetting.set(theme);
  applyTheme(theme);
}

// Apply theme to document
function applyTheme(theme: Theme) {
  if (typeof document === 'undefined') return;

  const root = document.documentElement;

  if (theme === 'system') {
    const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    root.classList.toggle('dark', isDark);
  } else {
    root.classList.toggle('dark', theme === 'dark');
  }
}

// Listen for system theme changes
if (typeof window !== 'undefined') {
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    let currentSetting: Theme = 'system';
    themeSetting.subscribe(v => currentSetting = v)();

    if (currentSetting === 'system') {
      document.documentElement.classList.toggle('dark', e.matches);
    }
  });
}
