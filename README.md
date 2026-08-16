# Create Tauri Nuxt

[![CI](https://github.com/xcvzmoon/dms-agent/actions/workflows/ci.yaml/badge.svg)](https://github.com/xcvzmoon/dms-agent/actions/workflows/ci.yaml)
[![Release](https://github.com/xcvzmoon/dms-agent/actions/workflows/release.yaml/badge.svg)](https://github.com/xcvzmoon/dms-agent/actions/workflows/release.yaml)

Opinionated template for creating a Tauri app with Nuxt.

## Stack

- [Tauri v2](https://v2.tauri.app) — Rust-backed desktop shell
- [Nuxt v4](https://nuxt.com) (SPA mode) — Vue frontend, rendered fully client-side for the Tauri webview
- [Nuxt UI v4](https://ui.nuxt.com) + Tailwind CSS v4 — component library and styling
- [VueUse](https://vueuse.org) — composables
- [Valibot](https://valibot.dev) — schema validation
- [Vite+](https://viteplus.dev) (`vp`) — unified dev/lint/format/test toolchain
- [Golar](https://github.com/auvred/golar) — Embedded language tooling orchestrator based on typescript-go

## Features

- **Auto-imported Tauri APIs** — a thin Nuxt module (`app/modules/tauri.ts`) auto-imports the `@tauri-apps/api/app`, `@tauri-apps/api/core`, and `@tauri-apps/plugin-store` APIs as `useTauriApp*`, `useTauriCore*`, and `useTauriStore*` composables, so there's no manual import needed to call Tauri from Vue components.

## Prerequisites

- [Vite+](https://viteplus.dev) (`vp`), the unified toolchain this project uses for installs, dev, build, lint, format, and test — see the [Vite+ guide](https://viteplus.dev/guide/) for install instructions
- [pnpm](https://pnpm.io) (version pinned via `devEngines` in `package.json`, currently `11.21.0`), the package manager Vite+ drives under the hood
- [Rust](https://www.rust-lang.org/tools/install) toolchain, required by Tauri
- Platform build dependencies for Tauri — see the [Tauri prerequisites guide](https://v2.tauri.app/start/prerequisites/)

## Getting Started

Install dependencies:

```bash
vp install
```

Run the app in dev mode (starts Nuxt and Tauri together, picking a free port if `3000` is busy):

```bash
vp run dev
```

Build the app for production:

```bash
vp run build
```

> `dev` and `build` are `package.json` scripts, not Vite+ built-ins — `vp dev` and `vp build` run Vite+'s own dev server/bundler instead, so use `vp run <script>` (or the `vpr` shorthand) for anything defined in `package.json`.

## Scripts

| Script                            | Description                                                                                                                                                       |
| --------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `vp run dev`                      | Run Nuxt + Tauri in development, auto-selecting an available port                                                                                                 |
| `vp run build`                    | Build the Tauri app for production                                                                                                                                |
| `vp run release`                  | Determine the next version from Conventional Commits, bump `package.json`/`tauri.conf.json`/`Cargo.toml`/`Cargo.lock`, update `CHANGELOG.md`, and tag the release |
| `vp run fmt` / `vp run fmt:check` | Format (or check formatting of) the frontend and Rust code                                                                                                        |
| `vp run lint`                     | Lint the frontend and Rust code                                                                                                                                   |
| `vp run test`                     | Run frontend and Rust test suites                                                                                                                                 |
| `vp run clean`                    | Clean Nuxt and Cargo build artifacts                                                                                                                              |

Frontend-only and Rust-only variants of these are also available, prefixed `vp:*`, `nuxt:*`, and `cargo:*` — see `package.json` for the full list.

## Releasing

```bash
vp run release [--type <major|premajor|minor|preminor|patch|prepatch|prerelease>] [--preid <id>] [--dry-run] [--push] [-y]
```

This keeps `package.json`, `src-tauri/tauri.conf.json`, and `src-tauri/Cargo.toml`/`Cargo.lock` versions in sync, then creates a release commit and an annotated `vX.Y.Z` tag. Pushing that tag triggers the [Release workflow](.github/workflows/release.yaml), which drafts a GitHub release and builds/uploads the Tauri app for macOS (Windows and Linux targets are scaffolded but currently disabled in the matrix).

## Project Structure

```
app/                  Nuxt app (pages, layouts, components, stores, modules)
src-tauri/            Tauri Rust backend, config, capabilities, and icons
scripts/              dev.ts and release.ts helper scripts
```

## Environment Variables

See `.env.example`:

- `NUXT_PORT` — preferred dev server port (falls back to an available one)
- `TAURI_HOST` — host the dev server binds to, used for mobile/device testing

## License

[MIT](LICENSE)
