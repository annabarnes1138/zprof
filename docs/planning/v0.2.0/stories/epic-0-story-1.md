# Story 0.1: Install Tauri and Initialize Project

**Epic:** Epic 0 - GUI Foundation
**Priority:** P0 (Blocking)
**Status:** ready-for-dev

## Dev Agent Record

**Context Reference:**
- [epic-0-story-1.context.xml](epic-0-story-1.context.xml)

## User Story

**As a** developer
**I want** Tauri installed and configured in the zprof project
**So that** we can build GUI applications with Rust backend

## Acceptance Criteria

- [ ] Install Tauri CLI: `cargo install tauri-cli`
- [ ] Initialize Tauri in project: `cargo tauri init`
- [ ] Choose Svelte as frontend framework
- [ ] Configure project structure:
  - `src-tauri/` for Tauri Rust backend
  - `src-ui/` for Svelte frontend
  - Keep existing `src/` for core business logic
- [ ] Update `.gitignore` for Tauri artifacts:
  - `src-tauri/target/`
  - `src-ui/node_modules/`
  - `src-ui/dist/`
  - `src-tauri/Cargo.lock` (keep root Cargo.lock)
- [ ] Configure `tauri.conf.json`:
  - App name: "zprof"
  - Window title: "zprof - Zsh Profile Manager"
  - Window size: 1200x800 (resizable)
  - Minimum size: 960x600
  - macOS and Linux targets
  - App identifier: "com.zprof.app"
- [ ] Add dependencies to `src-tauri/Cargo.toml`:
  ```toml
  [dependencies]
  tauri = { version = "2.0", features = ["shell-open"] }
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"

  [build-dependencies]
  tauri-build = { version = "2.0" }
  ```
- [ ] Create basic `src-tauri/src/main.rs` entry point
- [ ] Set up Svelte project in `src-ui/`:
  - `npm create vite@latest src-ui -- --template svelte-ts`
  - Install Tailwind CSS: `npm install -D tailwindcss postcss autoprefixer`
  - Configure Tailwind in `tailwind.config.js`
- [ ] Verify build: `cargo tauri dev` launches empty window
- [ ] Verify production build: `cargo tauri build` succeeds
- [ ] Add build instructions to README.md:
  - Prerequisites (Rust, Node.js, Tauri dependencies)
  - Development build: `cargo tauri dev`
  - Production build: `cargo tauri build`
  - Platform-specific notes

## Technical Details

### Tauri Configuration (`tauri.conf.json`)

```json
{
  "productName": "zprof",
  "identifier": "com.zprof.app",
  "version": "0.2.0",
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:5173",
    "distDir": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "zprof - Zsh Profile Manager",
        "width": 1200,
        "height": 800,
        "minWidth": 960,
        "minHeight": 600,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": ["dmg", "deb", "appimage"],
    "identifier": "com.zprof.app",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

### Project Structure After Initialization

```
zprof/
├── src/                    # Existing CLI code (unchanged)
├── src-tauri/              # NEW
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   ├── icons/
│   └── src/
│       ├── main.rs
│       └── lib.rs
├── src-ui/                 # NEW
│   ├── package.json
│   ├── vite.config.ts
│   ├── tsconfig.json
│   ├── tailwind.config.js
│   ├── postcss.config.js
│   ├── index.html
│   ├── public/
│   └── src/
│       ├── main.ts
│       ├── App.svelte
│       └── app.css
├── Cargo.toml              # Root workspace
└── README.md
```

### Cargo Workspace Configuration

Update root `Cargo.toml` to include Tauri workspace:

```toml
[workspace]
members = [".", "src-tauri"]
```

## Files Created/Modified

**New Files:**
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `src-tauri/build.rs`
- `src-tauri/src/main.rs`
- `src-tauri/src/lib.rs`
- `src-ui/package.json`
- `src-ui/vite.config.ts`
- `src-ui/tsconfig.json`
- `src-ui/tailwind.config.js`
- `src-ui/postcss.config.js`
- `src-ui/index.html`
- `src-ui/src/main.ts`
- `src-ui/src/App.svelte`
- `src-ui/src/app.css`

**Modified Files:**
- `.gitignore` (add Tauri/Node artifacts)
- `Cargo.toml` (workspace configuration)
- `README.md` (build instructions)

## Dependencies

**None** - This is the foundational story

## Testing

**Manual Verification:**
1. Run `cargo tauri dev` - Window should open with Vite dev server
2. Run `cargo tauri build` - Should produce platform bundle (`.dmg` on macOS, `.deb`/`.appimage` on Linux)
3. Verify hot reload works (edit `App.svelte`, see changes instantly)
4. Verify existing CLI still works: `cargo run -- --help`

**Success Criteria:**
- Empty Tauri window opens with Svelte "Hello World"
- No build errors or warnings
- Existing CLI functionality unaffected
- Documentation clearly explains build process

## Notes

- Keep existing `src/` directory unchanged - GUI and CLI coexist
- Use `cargo tauri dev` for development (hot reload)
- Use `cargo tauri build` for production bundles
- Tauri bundles are platform-specific (`.dmg` for macOS, `.deb`/`.appimage` for Linux)
- First build takes longer (downloads platform dependencies)

## References

- Tauri Quick Start: https://tauri.app/start/create-project/
- Svelte + Tauri Guide: https://tauri.app/guides/frontend/svelte
- Epic 0: [docs/planning/v0.2.0/epic-0-gui-foundation.md](../epic-0-gui-foundation.md)
- UX Design Spec: [docs/ux-design-specification.md](../../../ux-design-specification.md)
