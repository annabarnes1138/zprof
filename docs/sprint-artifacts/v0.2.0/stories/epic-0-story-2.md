# Story 0.2: Create Base Application Window and Navigation

**Epic:** Epic 0 - GUI Foundation
**Priority:** P0 (Blocking)
**Status:** ready-for-dev

## Dev Agent Record

**Context Reference:**
- [epic-0-story-2.context.xml](epic-0-story-2.context.xml)

## User Story

**As a** user
**I want** a clean, intuitive application window with navigation
**So that** I can access different features of zprof

## Acceptance Criteria

- [ ] Create main application layout with:
  - Sidebar navigation (collapsible, 240px width)
  - Main content area
  - Header bar with app title and actions
- [ ] Implement navigation structure:
  - **Profiles** (list view) - default route
  - **Create Profile** (wizard placeholder)
  - **Settings** (placeholder)
  - **About** (version info)
- [ ] Add routing with `svelte-spa-router`:
  - `/` or `/profiles` - Profile list (default)
  - `/create` - Create wizard
  - `/settings` - Settings panel
  - `/about` - About/version info
- [ ] Create reusable UI components (shadcn/ui style):
  - `Sidebar.svelte` - Navigation sidebar with collapsible state
  - `Header.svelte` - Top header/title bar
  - `Button.svelte` - Styled button component (primary, secondary, ghost variants)
  - `Card.svelte` - Content card component
- [ ] Implement sidebar navigation:
  - Icons + labels (icons from Lucide Svelte)
  - Active route highlighting (blue left border + background tint)
  - Collapse/expand toggle button
  - Persists collapsed state to localStorage
- [ ] Implement light/dark mode toggle:
  - Respect system theme preference on first launch
  - Manual toggle button in sidebar
  - Persist preference to localStorage
  - CSS custom properties for theming (use design tokens from UX spec)
- [ ] Apply design system from UX spec:
  - Colors: Developer Dark theme (primary: #3b82f6 blue)
  - Typography: Inter for UI, JetBrains Mono for code
  - Spacing: 4px base unit, Tailwind scale
  - Components: shadcn/ui patterns
- [ ] Add keyboard shortcuts:
  - `Cmd/Ctrl + ,` - Navigate to Settings
  - `Cmd/Ctrl + N` - Navigate to Create Profile
  - `Cmd/Ctrl + 1-4` - Navigate to sidebar items
- [ ] Handle window events:
  - Restore window size/position on launch (if saved)
  - Save window size/position on close

## Technical Details

### Component Structure

```
src-ui/src/
├── App.svelte              # Root component with router
├── components/
│   ├── ui/                 # shadcn/ui base components
│   │   ├── button.svelte
│   │   ├── card.svelte
│   │   ├── separator.svelte
│   │   └── tooltip.svelte
│   ├── Sidebar.svelte      # Main navigation sidebar
│   ├── Header.svelte       # Top header bar
│   └── ThemeToggle.svelte  # Light/dark mode toggle
├── views/
│   ├── ProfileList.svelte  # Placeholder: "Profiles coming soon"
│   ├── CreateWizard.svelte # Placeholder: "Wizard coming soon"
│   ├── Settings.svelte     # Placeholder: "Settings coming soon"
│   └── About.svelte        # Version info, links
├── lib/
│   ├── stores.ts           # Svelte stores (theme, sidebar state)
│   └── router.ts           # Route definitions
└── styles/
    └── globals.css         # Tailwind + design tokens
```

### Design Tokens (globals.css)

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 224 71% 4%;
    --primary: 217 91% 60%;
    --primary-foreground: 222 47% 11%;
    /* ... (full token set from UX spec) */
  }

  .dark {
    --background: 224 71% 4%;
    --foreground: 213 31% 91%;
    --primary: 217 91% 60%;
    /* ... (dark mode tokens) */
  }
}
```

### Sidebar Component

```svelte
<script lang="ts">
  import { Link } from 'svelte-spa-router';
  import { Home, Plus, Settings, Info, ChevronLeft } from 'lucide-svelte';
  import { sidebarCollapsed } from '$lib/stores';

  const navItems = [
    { path: '/', icon: Home, label: 'Profiles' },
    { path: '/create', icon: Plus, label: 'Create Profile' },
    { path: '/settings', icon: Settings, label: 'Settings' },
    { path: '/about', icon: Info, label: 'About' },
  ];
</script>

<aside class={$sidebarCollapsed ? 'w-16' : 'w-60'}>
  <!-- Sidebar content -->
</aside>
```

### Route Configuration

```typescript
// src-ui/src/lib/router.ts
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
```

## Design Mockup

```
┌────────────────────────────────────────────────────────┐
│  [☰] zprof                           [🌙]  [⚙]  [─][□][×] │
├──────────┬─────────────────────────────────────────────┤
│          │                                              │
│  [📁]    │  Profile List (Coming in Story 0.4)         │
│  Profiles│                                              │
│          │                                              │
│  [+]     │                                              │
│  Create  │                                              │
│          │                                              │
│  [⚙]     │                                              │
│  Settings│                                              │
│          │                                              │
│  [ℹ]     │                                              │
│  About   │                                              │
│          │                                              │
│  [🌙]    │                                              │
│  Theme   │                                              │
│          │                                              │
│  [«]     │                                              │
│  Collapse│                                              │
│          │                                              │
└──────────┴─────────────────────────────────────────────┘
```

## Files Created/Modified

**New Files:**
- `src-ui/src/components/Sidebar.svelte`
- `src-ui/src/components/Header.svelte`
- `src-ui/src/components/ThemeToggle.svelte`
- `src-ui/src/components/ui/button.svelte`
- `src-ui/src/components/ui/card.svelte`
- `src-ui/src/components/ui/separator.svelte`
- `src-ui/src/components/ui/tooltip.svelte`
- `src-ui/src/views/ProfileList.svelte`
- `src-ui/src/views/CreateWizard.svelte`
- `src-ui/src/views/Settings.svelte`
- `src-ui/src/views/About.svelte`
- `src-ui/src/lib/stores.ts`
- `src-ui/src/lib/router.ts`
- `src-ui/src/styles/globals.css`

**Modified Files:**
- `src-ui/src/App.svelte` (add router + layout)
- `src-ui/package.json` (add svelte-spa-router, lucide-svelte)

## Dependencies

- **Blocks:** Story 0.1 (Tauri initialization)
- **npm packages:**
  - `svelte-spa-router` - Client-side routing
  - `lucide-svelte` - Icon library
  - `tailwindcss` - Already installed in 0.1

## Testing

**Manual Verification:**
1. Navigate between routes using sidebar
2. Verify active route highlighting
3. Collapse/expand sidebar, verify state persists
4. Toggle dark/light mode, verify:
   - Colors change correctly
   - Preference persists on reload
5. Test keyboard shortcuts (Cmd+N, Cmd+,, etc.)
6. Resize window, verify responsive layout
7. Restart app, verify window size/position restored

**Visual Testing:**
- Sidebar width: 240px (expanded), 60px (collapsed)
- Active route: blue left border (4px) + subtle background
- Colors match UX spec (use design tokens)
- Typography: Inter font, 14px base size
- Spacing: consistent 4px grid

## Notes

- Use shadcn/ui component patterns (copy/paste, don't install)
- Lucide icons: https://lucide.dev/icons/
- Placeholder views show "Coming soon" message + relevant info
- About view should show zprof version (read from package.json)
- Theme toggle respects `prefers-color-scheme` media query
- Sidebar collapse state saved to `localStorage.getItem('sidebarCollapsed')`

## References

- UX Design Spec: [docs/ux-design-specification.md](../../../ux-design-specification.md) (Section 3: Design Direction, Section 2: Visual Foundation)
- shadcn/ui Svelte: https://www.shadcn-svelte.com/
- Lucide Svelte: https://lucide.dev/guide/packages/lucide-svelte
- svelte-spa-router: https://github.com/ItalyPaleAle/svelte-spa-router
