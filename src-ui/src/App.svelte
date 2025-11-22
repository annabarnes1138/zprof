<script lang="ts">
  import Router from 'svelte-spa-router';
  import { onMount } from 'svelte';
  import { push } from 'svelte-spa-router';
  import { routes } from './lib/router';
  import { theme } from './lib/stores';
  import Sidebar from './components/Sidebar.svelte';
  import Header from './components/Header.svelte';

  // Initialize theme on mount
  onMount(() => {
    // Apply initial theme class to document
    const currentTheme = localStorage.getItem('theme');
    if (currentTheme === 'dark' || (!currentTheme && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
      document.documentElement.classList.add('dark');
      theme.set('dark');
    } else {
      document.documentElement.classList.remove('dark');
      theme.set('light');
    }

    // Set up keyboard shortcuts
    function handleKeyboard(e: KeyboardEvent) {
      const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
      const modKey = isMac ? e.metaKey : e.ctrlKey;

      if (!modKey) return;

      switch (e.key) {
        case ',':
          e.preventDefault();
          push('/settings');
          break;
        case 'n':
          e.preventDefault();
          push('/create');
          break;
        case '1':
          e.preventDefault();
          push('/');
          break;
        case '2':
          e.preventDefault();
          push('/create');
          break;
        case '3':
          e.preventDefault();
          push('/settings');
          break;
        case '4':
          e.preventDefault();
          push('/about');
          break;
      }
    }

    window.addEventListener('keydown', handleKeyboard);

    return () => {
      window.removeEventListener('keydown', handleKeyboard);
    };
  });
</script>

<div class="flex h-screen overflow-hidden bg-background text-foreground">
  <!-- Sidebar -->
  <Sidebar />

  <!-- Main content area -->
  <div class="flex flex-col flex-1 overflow-hidden">
    <!-- Header -->
    <Header />

    <!-- Scrollable content -->
    <main class="flex-1 overflow-y-auto">
      <Router {routes} />
    </main>
  </div>
</div>
