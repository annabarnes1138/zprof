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

// Toast notifications store
export interface Toast {
  id: string;
  message: string;
  type: 'success' | 'error' | 'info';
  duration: number;
}

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);

  let idCounter = 0;

  function addToast(toast: Omit<Toast, 'id'>) {
    const id = `toast-${idCounter++}`;
    const newToast: Toast = { ...toast, id };

    update(toasts => [...toasts, newToast]);

    // Auto-dismiss based on type
    setTimeout(() => {
      removeToast(id);
    }, toast.duration);

    return id;
  }

  function removeToast(id: string) {
    update(toasts => toasts.filter(t => t.id !== id));
  }

  return {
    subscribe,
    success: (message: string) => addToast({ message, type: 'success', duration: 3000 }),
    error: (message: string) => addToast({ message, type: 'error', duration: 5000 }),
    info: (message: string) => addToast({ message, type: 'info', duration: 4000 }),
    remove: removeToast,
  };
}

// Wizard state store
export interface WizardState {
  step: number;
  framework: string | null;
  promptMode: 'engine' | 'theme' | null;
  promptEngine: string | null;
  frameworkTheme: string | null;
  plugins: string[];
  envVars: Record<string, string>;
}

const initialWizardState: WizardState = {
  step: 0,
  framework: null,
  promptMode: null,
  promptEngine: null,
  frameworkTheme: null,
  plugins: [],
  envVars: {},
};

function createWizardStore() {
  const { subscribe, set, update } = writable<WizardState>(initialWizardState);

  return {
    subscribe,
    set,
    update,
    reset: () => set(initialWizardState),
    nextStep: () => update(state => ({ ...state, step: state.step + 1 })),
    prevStep: () => update(state => ({ ...state, step: Math.max(0, state.step - 1) })),
  };
}

export const theme = createThemeStore();
export const sidebarCollapsed = createSidebarStore();
export const toast = createToastStore();
export const wizardState = createWizardStore();
