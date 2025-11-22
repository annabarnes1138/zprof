# zprof GUI - UX Design Specification

_Created on 2025-11-21 by Anna Barnes_
_Version: 1.0 - Initial GUI Pivot Design_

---

## Executive Summary

This UX Design Specification defines the visual and interaction design for zprof's GUI application, built with Tauri. The design pivots from a terminal-based TUI to a native desktop application that provides visual theme previews, intuitive profile management, and a delightful user experience while preserving all CLI functionality.

**Key Design Goals:**
- **Visual & Intuitive:** Enable theme preview and visual configuration
- **Efficient:** Fast profile switching and creation workflows
- **Accessible:** WCAG 2.1 AA compliance for inclusive use
- **Native Feel:** Platform-appropriate design for macOS and Linux
- **Complementary to CLI:** GUI enhances but doesn't replace CLI workflows

---

## 1. Design System Foundation

### 1.1 Design System Choice

**Selected: shadcn/ui with Tailwind CSS**

**Rationale:**
- **Modern & Customizable:** Built on Radix UI primitives, fully themeable
- **Accessible by Default:** WCAG 2.1 AA compliance built-in
- **Developer-Friendly:** Copy-paste components, no npm bloat
- **Svelte Compatible:** Works seamlessly with SvelteKit/Tauri
- **Rich Component Library:** 50+ components covering all zprof needs
- **Professional Look:** Clean, modern aesthetic appropriate for developer tools

**Alternative Considered:**
- **DaisyUI:** More opinionated styling, less customizable
- **Custom:** Too much effort for MVP, shadcn provides better foundation

**Components Provided:**
- Form elements (inputs, selects, checkboxes, radio groups)
- Navigation (tabs, menus, breadcrumbs)
- Overlays (dialogs, sheets, popovers, tooltips)
- Data display (tables, cards, badges)
- Feedback (alerts, toasts, progress indicators)
- Layout (separators, scrollable areas, resizable panels)

---

## 2. Visual Foundation

### 2.1 Color System

**Primary Theme: Developer Dark (with Light Mode Support)**

**Color Palette:**

```css
/* Dark Mode (Primary) */
--background: 224 71% 4%        /* #0a0d15 - Deep blue-black */
--foreground: 213 31% 91%       /* #e4e7ec - Off-white */

--primary: 217 91% 60%          /* #3b82f6 - Blue (trust, tech) */
--primary-foreground: 222 47% 11%

--secondary: 222 47% 11%        /* #151822 - Darker blue-gray */
--secondary-foreground: 213 31% 91%

--accent: 216 34% 17%           /* #1c2534 - Subtle blue-gray */
--accent-foreground: 213 31% 91%

--muted: 223 47% 11%            /* #151822 */
--muted-foreground: 215.4 16.3% 56.9%

--destructive: 0 63% 31%        /* #822727 - Red for delete */
--destructive-foreground: 213 31% 91%

--border: 216 34% 17%           /* #1c2534 */
--input: 216 34% 17%
--ring: 217 91% 60%             /* Focus ring matches primary */

/* Semantic Colors */
--success: 142 71% 45%          /* #22c55e - Green */
--warning: 38 92% 50%           /* #f59e0b - Amber */
--error: 0 72% 51%              /* #ef4444 - Red */
--info: 199 89% 48%             /* #0ea5e9 - Cyan */
```

**Light Mode:**
```css
--background: 0 0% 100%         /* #ffffff */
--foreground: 224 71% 4%        /* #0a0d15 */
--primary: 217 91% 60%          /* Same blue */
/* ... (inverted from dark) */
```

**Rationale:**
- **Dark-first:** Developer tools often used in low-light, dark mode preferred
- **Blue Primary:** Conveys trust, technology, professionalism
- **High Contrast:** Excellent readability, passes WCAG AA for text
- **Platform-Native:** Follows macOS Big Sur / modern Linux dark theme aesthetics
- **Consistent Semantics:** Success/warning/error colors follow universal conventions

### 2.2 Typography

**Font Families:**
```css
--font-sans: 'Inter var', system-ui, -apple-system, sans-serif;
--font-mono: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
```

**Type Scale:**
- **h1:** 2.25rem (36px), font-weight: 700, line-height: 1.2
- **h2:** 1.875rem (30px), font-weight: 700, line-height: 1.3
- **h3:** 1.5rem (24px), font-weight: 600, line-height: 1.4
- **h4:** 1.25rem (20px), font-weight: 600, line-height: 1.4
- **body:** 0.875rem (14px), font-weight: 400, line-height: 1.5
- **small:** 0.75rem (12px), font-weight: 400, line-height: 1.5
- **code:** 0.875rem (14px), font-family: mono, line-height: 1.6

**Rationale:**
- **Inter:** Clean, highly legible, variable font for smooth scaling
- **JetBrains Mono:** Designed for developers, excellent for code/profile names
- **14px Base:** Readable at typical desktop viewing distances
- **System Fallbacks:** Respect user's system fonts if custom unavailable

### 2.3 Spacing & Layout

**Base Unit:** 4px (0.25rem in Tailwind)

**Spacing Scale:**
- **xs:** 4px (0.25rem)
- **sm:** 8px (0.5rem)
- **md:** 16px (1rem)
- **lg:** 24px (1.5rem)
- **xl:** 32px (2rem)
- **2xl:** 48px (3rem)
- **3xl:** 64px (4rem)

**Grid System:**
- **12-column responsive grid** (Tailwind default)
- **Container max-width:** 1280px (reasonable for desktop apps)
- **Sidebar width:** 240px (collapsed: 60px)
- **Content padding:** 24px (lg)

---

## 3. Design Direction

### 3.1 Chosen Design Approach

**Direction: "Professional Dashboard" - Clean, Efficient, Information-Dense**

**Layout Pattern:**
- **Left Sidebar Navigation** (collapsible)
- **Main Content Area** with header and scrollable content
- **Card-Based Organization** for profiles, settings sections
- **Floating Action Button** for primary "Create Profile" action

**Visual Hierarchy:**
- **Density Level:** Balanced (not too sparse, not cramped)
- **Visual Weight:** Minimal with subtle elevation
  - Flat design with 1px borders
  - Subtle shadows on cards (0 1px 3px rgba(0,0,0,0.12))
  - Hover states use subtle background color shift
- **Content Focus:** Text and data-driven with tasteful iconography

**Interaction Patterns:**
- **Primary Actions:** Prominent blue buttons, high contrast
- **Secondary Actions:** Ghost/outline buttons, lower visual weight
- **Inline Editing:** Click-to-edit profile names, inline forms
- **Modal Workflows:** Wizards use sheet/dialog overlays, not full-page
- **Progressive Disclosure:** Advanced options collapsed by default

**Navigation:**
- Sidebar sections:
  - **Profiles** (default view)
  - **Create Profile** (wizard)
  - **Settings**
  - **About**
- Active state: Blue left border + background tint
- Icon + label format (icon-only when collapsed)

**Rationale:**
- **Familiar Pattern:** Sidebar + content is standard for desktop apps (VS Code, Figma, etc.)
- **Efficient:** Power users can navigate quickly, beginners aren't overwhelmed
- **Scalable:** Easy to add new sections without redesigning
- **Platform-Appropriate:** Matches macOS/Linux desktop app conventions

---

## 4. Core User Experience

### 4.1 Defining Experience

**Core Experience:** "Effortless profile management with visual confidence"

**Primary User Task:** Create and switch between zsh profiles quickly and safely

**User Expectation:** When I describe my ideal zsh setup, I should:
1. **See** what themes look like before choosing
2. **Create** a profile in 2-3 minutes
3. **Switch** profiles instantly with visual confirmation
4. **Trust** that my configs won't break

**Emotional Goal:** **Confidence & Control**
- Users should feel empowered, not confused
- Every action should have clear, immediate feedback
- Mistakes should be easy to undo
- The interface should "get out of the way" for power users

### 4.2 Novel UX Patterns

**Theme Preview Component (Novel for Terminal Tools)**

**Challenge:** Users need to see what oh-my-zsh/zimfw themes actually look like before committing.

**Solution: Interactive Theme Preview Card**

**Anatomy:**
- **Theme Name & Metadata Header:** Framework, popularity, Nerd Font requirement
- **Live Preview Canvas:**
  - Simulated terminal showing theme rendering git branch, directory, etc.
  - Dark background matching terminal aesthetics
  - Actual ANSI color rendering where possible
  - Screenshot fallback for complex themes
- **Actions:**
  - "Preview in Terminal" button → Opens actual terminal with temp theme
  - "Select" button → Adds to profile wizard
  - Heart icon → Favorite for later

**States:**
- **Default:** Theme preview visible, select button enabled
- **Hover:** Slight scale (1.02x), shadow deepens
- **Selected:** Blue border, checkmark badge
- **Loading:** Skeleton preview, shimmer effect
- **Error:** Gray placeholder with "Preview unavailable" message

**Interaction Flow:**
1. User scrolls through theme grid
2. Hover shows theme metadata tooltip
3. Click card → Enlarged preview modal with more details
4. Click "Select" → Theme added to wizard, modal closes
5. Visual feedback: Card marked selected, count updates

**Accessibility:**
- **Keyboard:** Arrow keys navigate, Enter selects, Esc closes modal
- **Screen Reader:** "Robby Russell theme. Minimal prompt with git integration. Selected."
- **Color Blind:** Don't rely only on color for selection (use checkmark icon)

**Technical Approach:**
- Fetch theme screenshots from curated library
- For dynamic preview: Mini shell renderer using ANSI escape codes
- Fallback: Static screenshots with note "Preview may vary"

---

## 5. User Journey Flows

### 5.1 Critical User Journeys

#### Journey 1: Create New Profile (First-Time User)

**User Goal:** Set up my first zprof profile

**Flow:**

1. **Landing / Empty State**
   - **User sees:** Large welcome message, "Create Your First Profile" CTA button
   - **User does:** Clicks "Create Profile" button
   - **System responds:** Opens Create Profile wizard (sheet overlay)

2. **Step 1: Choose Framework**
   - **User sees:** Grid of 5 framework cards (oh-my-zsh, zimfw, zinit, zap, prezto)
   - Each card shows: Logo, name, description, "Popular" or "Fast" badge
   - **User does:** Clicks a framework card (e.g., oh-my-zsh)
   - **System responds:** Card highlights, "Next" button activates

3. **Step 2: Choose Prompt Mode**
   - **User sees:** Two large option cards
     - "Standalone Prompt Engine" (Starship, P10k, etc.)
     - "Framework Built-in Theme" (robbyrussell, agnoster, etc.)
   - **User does:** Selects "Standalone Prompt Engine"
   - **System responds:** Next step loads

4. **Step 3: Choose Prompt Engine**
   - **User sees:** Engine cards with descriptions and requirements
   - **User does:** Selects "Starship"
   - **System responds:** Shows Nerd Font requirement note

5. **Step 4: Choose Plugins** (Optional - can skip)
   - **User sees:** Searchable plugin browser, categorized, recommended highlighted
   - **User does:** Selects 3-4 plugins via checkboxes
   - **System responds:** Selected count updates

6. **Step 5: Review & Create**
   - **User sees:** Summary of all choices, "Create Profile" button, profile name input
   - **User does:** Names profile "work", clicks "Create Profile"
   - **System responds:**
     - Progress modal appears: "Installing oh-my-zsh... ✓"
     - "Installing Starship... ✓"
     - "Installing plugins... ✓"
     - "Generating configuration... ✓"
   - **Success:** Modal shows "Profile 'work' created successfully! Activate now?"
   - **User does:** Clicks "Activate"
   - Returns to profile list with new profile active (badge shown)

**Decision Points:**
- Framework choice branches to appropriate theme/engine options
- Prompt mode determines if theme or engine selection shown
- Plugin selection is optional (can skip)

**Error Handling:**
- If installation fails: Show error with retry button and "Skip for now" option
- If profile name exists: Inline validation "Profile 'work' already exists. Choose another name."

**Estimated Steps to Value:** 5 steps, ~2-3 minutes

---

#### Journey 2: Switch Between Profiles (Power User)

**User Goal:** Quickly switch to a different profile

**Flow:**

1. **Profile List View**
   - **User sees:** Grid of profile cards, active profile has green "Active" badge
   - **User does:** Clicks "Activate" button on different profile card
   - **System responds:**
     - Button shows spinner briefly
     - Toast notification: "Activated profile 'personal'"
     - Badge moves to new active profile
     - Previous profile badge disappears

**Alternate Flow (Keyboard Shortcut):**
1. User presses `Cmd/Ctrl + P` (quick profile switcher)
2. Modal appears with profile list + search
3. User types profile name or arrows to select
4. Presses Enter → Profile activates, modal closes

**Estimated Time:** <5 seconds

---

#### Journey 3: Delete Profile (With Safety)

**User Goal:** Remove an old profile I don't use

**Flow:**

1. **Profile Card**
   - **User sees:** Profile card with "..." menu icon
   - **User does:** Clicks "..." → Dropdown menu appears
   - **User does:** Clicks "Delete profile"

2. **Confirmation Dialog**
   - **User sees:** Modal dialog:
     ```
     Delete Profile "old-work"?

     This profile and its configuration will be permanently deleted.
     Your backed-up configs will remain safe.

     [Cancel] [Delete Profile]
     ```
   - **User does:** Clicks "Delete Profile" (red, destructive button)
   - **System responds:**
     - Dialog closes
     - Profile card animates out (fade + slide)
     - Toast: "Profile 'old-work' deleted"

**Safety Net:**
- Can't delete active profile (button disabled with tooltip: "Deactivate this profile first")
- Confirmation required (no accidental deletion)
- Clear explanation of what's deleted

---

### 5.2 Secondary Journeys

**Profile Editing:** Click profile card → Edit button → Similar wizard but pre-filled (v0.3.0)

**Settings Configuration:** Sidebar → Settings → Tabs for preferences, appearance, advanced (v0.3.0)

**Theme Preview:** Click "Preview Themes" from create wizard → Opens theme browser with live previews (v0.2.0)

---

## 6. Component Library Strategy

### 6.1 Core Components (from shadcn/ui)

**Navigation:**
- `NavigationMenu` - Sidebar navigation
- `Tabs` - Settings panels, wizard steps
- `Breadcrumb` - Navigation context (if needed)

**Data Display:**
- `Card` - Profile cards, setting sections
- `Badge` - Active indicator, framework labels, plugin tags
- `Table` - Plugin list (optional advanced view)
- `Avatar` - Profile icons (optional)

**Inputs:**
- `Input` - Profile name, search
- `Select` - Dropdown selections
- `Checkbox` - Plugin selection
- `RadioGroup` - Framework/engine selection
- `Switch` - Boolean settings (dark mode toggle)

**Feedback:**
- `Toast` - Success/error notifications
- `Alert` - Important messages, warnings
- `Progress` - Installation progress
- `Skeleton` - Loading states

**Overlays:**
- `Dialog` - Confirmations, alerts
- `Sheet` - Create profile wizard (slide-in from right)
- `Popover` - Contextual help, tooltips
- `Tooltip` - Inline help

**Actions:**
- `Button` - All actions
- `DropdownMenu` - Profile actions menu

### 6.2 Custom Components

**Component: ProfileCard**
- **Purpose:** Display profile information with actions
- **Content:** Profile name, framework, prompt type, plugin count, created date
- **Actions:** Activate (if inactive), Edit (icon), Delete (menu), Duplicate (menu)
- **States:**
  - Default: White/gray card, subtle border
  - Active: Green "Active" badge, slightly elevated
  - Hover: Shadow deepens, actions appear
  - Loading: Skeleton content
- **Variants:** Grid view (card), List view (row)

**Component: ThemePreviewCard**
- **Purpose:** Show theme appearance before selection
- **Content:** Theme name, framework badge, preview canvas, description
- **Actions:** Select button, preview modal trigger
- **States:**
  - Default: Unselected, preview visible
  - Hover: Scale 102%, shadow
  - Selected: Blue border, checkmark
  - Loading: Skeleton preview
  - Error: Placeholder with message
- **Variants:** Small (grid), Large (modal view)

**Component: WizardSheet**
- **Purpose:** Multi-step profile creation workflow
- **Content:** Step indicator, current step content, prev/next buttons
- **Actions:** Previous, Next, Cancel, Finish
- **States:**
  - Step 1-5: Different content per step
  - Loading: Progress overlay during installation
  - Success: Completion screen
  - Error: Error screen with retry
- **Behavior:** Slides in from right, persists state if canceled and reopened

**Component: PluginBrowser**
- **Purpose:** Search and select plugins
- **Content:** Search bar, category filters, plugin cards with descriptions
- **Actions:** Select/deselect via checkbox, "Recommended" quick-add
- **States:**
  - Default: All plugins shown
  - Filtered: Search/category results
  - Empty: "No plugins found" with clear filters
- **Accessibility:**
  - ARIA role: `listbox` with `option` items
  - Keyboard: Type to search, arrow keys navigate, space to select

**Component: InstallationProgress**
- **Purpose:** Show real-time installation progress
- **Content:** Task list with checkmarks, current task spinner, overall progress bar
- **States:**
  - In Progress: Spinner on current task
  - Success: All tasks have checkmarks
  - Error: Failed task has X, error message, retry button
- **Animation:** Tasks check off sequentially, smooth transitions

---

## 7. UX Pattern Decisions

### 7.1 Button Hierarchy

- **Primary Action:** `bg-primary text-primary-foreground` (blue, high contrast)
  - Usage: "Create Profile", "Activate", "Save"
- **Secondary Action:** `bg-secondary text-secondary-foreground` (subtle)
  - Usage: "Cancel", "Back", navigation
- **Destructive Action:** `bg-destructive text-destructive-foreground` (red)
  - Usage: "Delete Profile", "Remove Plugin"
- **Ghost Action:** `variant="ghost"` (transparent, hover background)
  - Usage: Close buttons, icon buttons, tertiary actions

### 7.2 Feedback Patterns

- **Success:** Toast notification (top-right), green background, 3s auto-dismiss
  - "Profile 'work' created successfully!"
- **Error:** Toast notification (top-right), red background, 5s auto-dismiss + close button
  - "Failed to install Starship. [Retry]"
- **Warning:** Alert component (inline), yellow background
  - "Starship requires Nerd Font. Install font first."
- **Info:** Tooltip on hover, subtle blue background
  - (i) icon → "Framework themes are built into the framework"
- **Loading:**
  - Skeleton screens for initial loads
  - Spinners for actions (button shows spinner on click)
  - Progress bars for multi-step operations

### 7.3 Form Patterns

- **Label Position:** Above input (clearer for desktop)
- **Required Fields:** Red asterisk (*) after label
- **Validation Timing:** `onBlur` for individual fields, `onSubmit` for form
- **Error Display:** Inline below field, red text + red border
  - "Profile name is required"
- **Help Text:** Gray caption below input (before errors)
  - "Use lowercase letters, numbers, and hyphens"

### 7.4 Modal Patterns

- **Sizes:**
  - Small (400px): Confirmations, simple forms
  - Medium (600px): Default for most dialogs
  - Large (800px): Wizards, complex forms
  - Full: Not used (desktop app, not mobile)
- **Dismiss:** Click outside OR Escape key OR explicit close button
- **Focus Management:** Auto-focus first input on open, trap focus inside modal
- **Stacking:** Only one modal at a time (simplifies UX)

### 7.5 Navigation Patterns

- **Active State:** Blue left border (4px) + background tint
- **Breadcrumbs:** Not used (sidebar makes location clear)
- **Back Button:** Browser back not applicable (desktop app), wizard has "Previous" button
- **Deep Linking:** Profile URLs like `zprof://profile/work` (for future sharing)

### 7.6 Empty State Patterns

- **First Use (No Profiles):**
  - Large icon (folder/profile graphic)
  - Headline: "No profiles yet"
  - Body: "Create your first profile to get started managing zsh configurations."
  - CTA: Large "Create Profile" button
- **No Results (Search/Filter):**
  - "No profiles match your search"
  - Suggestion: "Try different keywords or clear filters"
  - Button: "Clear Filters"
- **Cleared Content:** Not applicable (can't delete all profiles while one is active)

### 7.7 Confirmation Patterns

- **Delete Profile:** Always confirm with dialog (destructive, can't undo easily)
- **Leave Unsaved:** Detect unsaved wizard changes, warn on close
  - "You have unsaved changes. Discard them?"
- **Irreversible Actions:** Explicit confirmation + clear explanation

### 7.8 Notification Patterns

- **Placement:** Top-right corner
- **Duration:**
  - Success: 3s auto-dismiss
  - Error: 5s auto-dismiss + manual close
  - Info: 4s auto-dismiss
- **Stacking:** Stack vertically, max 3 visible (oldest auto-dismisses)
- **Priority:** Not implemented (all same visual weight), could add in future

---

## 8. Responsive Design & Accessibility

### 8.1 Responsive Strategy

**Target Resolutions:**
- **Minimum Window Size:** 960x600 (enforced by Tauri config)
- **Optimal:** 1280x800 to 1920x1080
- **No mobile/tablet:** Desktop-only application

**Adaptation Patterns:**
- **Sidebar:**
  - **Large (>1200px):** Full sidebar (240px), icons + labels
  - **Medium (960-1200px):** Collapsed sidebar (60px), icons only
  - **User Toggle:** Button to manually collapse/expand
- **Profile Grid:**
  - **Large:** 3 columns
  - **Medium:** 2 columns
  - **Small (if window <960px):** 1 column (shouldn't happen with min-width enforced)
- **Modals:** Centered, responsive width (max 90% viewport width)
- **Tables:** Horizontal scroll if needed (rare case)

**Layout Grid:**
- 12-column grid for flexible layouts
- Responsive gap (16px → 24px on larger screens)

### 8.2 Accessibility Strategy

**WCAG Compliance Target:** WCAG 2.1 Level AA

**Key Requirements Met:**

**Color Contrast:**
- Text on background: 7:1 (AAA for normal text)
- UI components: 3:1 minimum (borders, icons)
- Tested with color contrast analyzers

**Keyboard Navigation:**
- All interactive elements accessible via Tab
- Logical tab order (left-to-right, top-to-bottom)
- Focus indicators: 2px blue outline on all focusable elements
- Shortcuts: `Cmd/Ctrl + P` (quick switcher), `Cmd/Ctrl + N` (new profile), `Cmd/Ctrl + ,` (settings)

**Screen Reader Support:**
- Semantic HTML: `<nav>`, `<main>`, `<article>`, `<button>`, `<input>`
- ARIA labels on icon-only buttons: `aria-label="Create new profile"`
- ARIA live regions for toast notifications: `aria-live="polite"`
- Form labels properly associated: `<label for="profile-name">`

**Focus Management:**
- Focus trapping in modals (can't tab outside)
- Return focus to trigger element on modal close
- Skip links: "Skip to main content" (hidden until focused)

**Alt Text:**
- All icons have descriptive labels
- Theme preview images have alt text: "Robby Russell theme preview showing minimal prompt"

**Touch Target Size:**
- Minimum 44x44px for all clickable elements (exceeds 24px minimum)
- Adequate spacing between interactive elements (8px minimum)

**Testing Strategy:**
- **Automated:** Lighthouse accessibility audits (score >95)
- **Manual:** Keyboard-only navigation testing
- **Screen Reader:** VoiceOver (macOS) and Orca (Linux) testing
- **Tools:** axe DevTools, WAVE browser extension

**Known Limitations:**
- Theme preview canvas may not be fully accessible (provide text description as fallback)
- Some advanced animations may need "Reduce motion" OS setting respect

---

## 9. Implementation Guidance

### 9.1 Technology Stack

**Frontend:**
- Svelte 4+ for component framework
- Tailwind CSS for styling
- shadcn/ui components (copy-paste, customized)
- Vite for bundling

**Desktop:**
- Tauri 2.0+ for native app wrapper
- IPC for frontend ↔ Rust backend communication

**Backend:**
- Existing Rust business logic (src/core, src/frameworks, etc.)
- Tauri commands for GUI-specific operations

### 9.2 Design Tokens (CSS Custom Properties)

All colors, spacing, typography defined as CSS variables in `src-ui/src/styles/globals.css`:

```css
@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 224 71% 4%;
    /* ... (all tokens from Section 2.1) */
  }

  .dark {
    --background: 224 71% 4%;
    --foreground: 213 31% 91%;
    /* ... (dark mode tokens) */
  }
}
```

### 9.3 Component Development Guidelines

1. **Start with shadcn/ui:** Copy component from shadcn, customize colors/spacing
2. **Use Tailwind Utilities:** Prefer `className` composition over custom CSS
3. **Responsive Classes:** Use `md:` and `lg:` prefixes for breakpoints
4. **Dark Mode:** Use `dark:` prefix for dark mode variants
5. **Accessibility First:** Include ARIA labels, keyboard handlers from start
6. **Svelte Stores:** Use stores for global state (active profile, theme preference)

### 9.4 Recommended Component Structure

```
src-ui/src/
├── components/
│   ├── ui/               # shadcn/ui components
│   │   ├── button.svelte
│   │   ├── card.svelte
│   │   └── ...
│   ├── ProfileCard.svelte
│   ├── ThemePreviewCard.svelte
│   ├── WizardSheet.svelte
│   └── ...
├── views/
│   ├── ProfileList.svelte
│   ├── CreateWizard.svelte
│   ├── Settings.svelte
│   └── About.svelte
├── lib/
│   ├── api.ts            # Tauri IPC client
│   ├── stores.ts         # Global Svelte stores
│   └── utils.ts          # Utilities
└── styles/
    └── globals.css       # Tailwind + design tokens
```

---

## 10. Next Steps

### 10.1 Immediate Actions (Epic 0 Implementation)

1. **Story 0.1:** Install Tauri, set up project structure ✅ (documented in epic)
2. **Story 0.2:** Build base application window with sidebar navigation
   - Implement Sidebar.svelte with navigation items
   - Apply chosen color theme and typography
3. **Story 0.3:** Implement IPC command layer
   - Create Tauri commands for profile operations
   - Build TypeScript API client
4. **Story 0.4:** Create ProfileList view with ProfileCard components
   - Use design direction from this spec
   - Implement activate/delete actions
5. **Story 0.5:** Ensure CLI compatibility with feature flags

### 10.2 Design Refinements Needed (During Implementation)

- [ ] **Finalize Iconography:** Choose icon set (Lucide, Heroicons, or Phosphor)
- [ ] **Theme Preview Technical Approach:** Determine how to render theme previews (screenshots vs. live render)
- [ ] **Animation Timing:** Define transition durations and easing functions
- [ ] **Error State Visuals:** Design specific error screens for common failures
- [ ] **Loading States:** Refine skeleton screens for each view

### 10.3 Future Enhancements (v0.3.0+)

- Interactive onboarding tour for first launch
- Profile templates/presets
- Theme preview animations
- Advanced plugin configuration UI
- Profile export/import with visual workflow
- Multi-window support for comparing profiles

---

## Appendix

### Related Documents

- **Sprint Change Proposal:** [docs/sprint-change-proposal-2025-11-21.md](../sprint-change-proposal-2025-11-21.md)
- **Epic 0 - GUI Foundation:** [docs/planning/v0.2.0/epic-0-gui-foundation.md](planning/v0.2.0/epic-0-gui-foundation.md)
- **Technical Decisions:** [docs/developer/technical-decisions.md](developer/technical-decisions.md) (AD-003)
- **Architecture Document:** [docs/developer/architecture.md](developer/architecture.md) (to be updated)

### Design References

**Inspiration Apps:**
- **VS Code:** Sidebar navigation, command palette, extension browsing
- **GitHub Desktop:** Profile management, clean cards
- **Docker Desktop:** Resource management, settings panels
- **Figma:** Modern UI, smooth interactions
- **Raycast:** Quick switcher, keyboard-first interactions

**Design Systems:**
- shadcn/ui Documentation: https://ui.shadcn.com/
- Tailwind CSS: https://tailwindcss.com/
- Radix UI Primitives: https://www.radix-ui.com/

### Version History

| Date       | Version | Changes                         | Author       |
| ---------- | ------- | ------------------------------- | ------------ |
| 2025-11-21 | 1.0     | Initial UX Design Specification | Anna Barnes  |

---

_This UX Design Specification was created to guide the Tauri GUI implementation for zprof. All design decisions are documented with rationale to support consistent implementation and future iterations._
