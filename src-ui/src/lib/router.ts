import ProfileList from '../views/ProfileList.svelte';
import CreateWizard from '../views/CreateWizard.svelte';
import Settings from '../views/Settings.svelte';
import About from '../views/About.svelte';

export const routes = {
  '/': ProfileList,
  '/profiles': ProfileList,
  '/create': CreateWizard,
  '/settings': Settings,
  '/about': About,
};
