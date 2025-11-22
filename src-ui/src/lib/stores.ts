import { writable } from 'svelte/store';

// Theme store - persists to localStorage
function createThemeStore() {
  const { subscribe, set, update } = writable<'light' | 'dark'>('dark');

  // Initialize from localStorage or system preference
  if (typeof window !== 'undefined') {
    const stored = localStorage.getItem('theme');
    if (stored === 'light' || stored === 'dark') {
      set(stored);
    } else {
      // Respect system preference
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      set(prefersDark ? 'dark' : 'light');
    }
  }

  return {
    subscribe,
    toggle: () => {
      update(current => {
        const newTheme = current === 'light' ? 'dark' : 'light';
        if (typeof window !== 'undefined') {
          localStorage.setItem('theme', newTheme);
          document.documentElement.classList.toggle('dark', newTheme === 'dark');
        }
        return newTheme;
      });
    },
    set: (value: 'light' | 'dark') => {
      if (typeof window !== 'undefined') {
        localStorage.setItem('theme', value);
        document.documentElement.classList.toggle('dark', value === 'dark');
      }
      set(value);
    }
  };
}

// Sidebar collapsed state - persists to localStorage
function createSidebarStore() {
  const { subscribe, set, update } = writable<boolean>(false);

  // Initialize from localStorage
  if (typeof window !== 'undefined') {
    const stored = localStorage.getItem('sidebarCollapsed');
    if (stored !== null) {
      set(stored === 'true');
    }
  }

  return {
    subscribe,
    toggle: () => {
      update(current => {
        const newValue = !current;
        if (typeof window !== 'undefined') {
          localStorage.setItem('sidebarCollapsed', String(newValue));
        }
        return newValue;
      });
    },
    set: (value: boolean) => {
      if (typeof window !== 'undefined') {
        localStorage.setItem('sidebarCollapsed', String(value));
      }
      set(value);
    }
  };
}

export const theme = createThemeStore();
export const sidebarCollapsed = createSidebarStore();
