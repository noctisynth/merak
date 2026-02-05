# Merak — Copilot Instructions

This file provides guidance for GitHub Copilot (and other AI assistants) when contributing to the `merak` repository. It explains the repository layout, coding conventions, important files and commands, and gives example prompts you can use to generate or modify code in a way that fits the project.

Use these instructions whenever you ask Copilot to generate code, propose changes, or help with reviews for this repository.

---

## Quick repository summary

- Primary languages: TypeScript (React + Vite) and Rust (workspace crates).
- Package manager: `pnpm` (see `package.json` for `packageManager`).
- Build tools: `vite`, `tsc` (project references).
- Styling: `tailwindcss`.
- Monorepo structure roots:
  - `crates/` — Rust workspace crates (backend crates live here)
  - `src/` — front-end source (TypeScript/React). Frontend file and component names should use snake_case (for example `button.tsx`, `user_card.tsx`) rather than camelCase or PascalCase.
  - `.github/` — GitHub configuration
  - `.changes/` — version change documentation
- Version management: Create a `.md` file under `.changes` to document version changes.

Important high-level files:
- `package.json` — scripts: `dev`, `build`, `lint`, `preview`, `prepare`.
- `Cargo.toml` — Rust workspace configuration (if editing or adding Rust crates, keep consistent with workspace).
- `components.json` — Shadcn component registry metadata. When adding or updating UI components provided by Shadcn, prefer invoking the shadcn CLI to add components (for example: `pnpm dlx shadcn@latest add button`).

License: `AGPL-v3.0` (do not suggest relicensing or remove/alter license header).

Author contact is available in `package.json` under `author`.

---

## General guidance for generated code

When Copilot generates code for this repo, follow these rules:

- Target languages and frameworks:
  - Frontend: TypeScript + React (React 19), Vite.
  - Styling: Tailwind CSS (v4.x), use utility classes. When creating reusable components, provide `className` prop for styling overrides.
  - Component generation and registry: Use Shadcn components. When adding a Shadcn component, invoke the shadcn CLI (e.g., `pnpm dlx shadcn@latest add button`) and ensure `components.json` (registry metadata) is updated accordingly.
  - State management: Zustand, create Zustand store under `src/models`.
  - Rust: If modifying Rust crates under `crates/`, follow existing crate structure and keep Cargo workspace settings unchanged.
- Imports:
  - Use the path alias `@/*` for imports into frontend code (this maps to `./src/*` per `tsconfig.json`).
  - Keep imports explicit and minimal. Prefer named imports for named exports.
- Types and safety:
  - Use TypeScript strict typing where possible. Prefer explicit return types on exported functions/components.
  - Avoid `any`. If `any` is necessary temporarily, add a `// TODO` comment with rationale and a tracking issue/PR reference.
- Styling and component APIs:
  - Components should accept `className?: string` and `...props` forwarding to the root element when appropriate.
  - If using `class-variance-authority` (`cva`) patterns, follow existing conventions in the repo for variants and default behaviors.
- Tests and type checks:
  - Do not introduce untested code paths. When adding significant logic, include one or more unit tests (preferred) or a clear plan to test manually.
  - Generated TypeScript must pass `tsc -b` for the project or subtree you modify.
- Security and secrets:
  - Never include secrets, credentials, API keys, or personal data in generated code or examples.
  - Avoid adding telemetry, analytics, or network calls without explicit intent and approval.
- Dependencies:
  - Do not add new dependencies without a clear benefit. If a dependency is required, prefer lightweight and well-maintained packages, and update `package.json` with a rationale comment in the PR.
  - Respect existing package manager constraints (`pnpm` and version overrides in `package.json`).

---

## Formatting, linting, and pre-commit

- The repo uses `biome` as the linter/formatter (`biome check .`). Any generated code should conform to the project's linting rules.
- Pre-commit hooks are configured. Generated code should not rely on disabling hooks. If you must, clearly document why.
- Pre-commit based on `prek` instead of `pre-commit`, using `prek run` to execute all pre-commit hooks.

---

## Building and running locally

When generating instruction-help, reference these commands:

- Start dev server (frontend): `pnpm dev` (runs the `dev` script that invokes `vite`).
- Build: `pnpm build` (runs `tsc -b` then `vite build`).
- Lint: `pnpm lint` (runs `biome check .`).
- Preview build: `pnpm preview`.
- Prepare: `pnpm prepare` (runs `prek install` as defined).

When adding scripts or changing behavior, make sure they integrate cleanly with `pnpm` workspace and the existing build pipeline.

---

## Working with Rust crates

- Respect the Rust workspace in `Cargo.toml`. If you add a crate in `crates/`, ensure workspace members are updated accordingly.
- Follow existing crate conventions (error handling, crate-level doc comments).
- All Rust code changes must compile cleanly and fully pass `cargo clippy` with no warnings or errors. Do not introduce any `unsafe` code anywhere in the workspace.
- Run `cargo build` and `cargo test` for crates you modify.

### merak-macros (crates/macros)

The repository contains a set of macros used by Merak Backend.

- Treat macros as high-risk infrastructure:
  - Modify them only with extreme caution, coordinate with maintainers
  - Ensure backward-compatible generated outputs whenever possible.

### merak (crates/merak)

Server-side Rust crate for Merak Backend.

- Keep the file structure simple and modular:
  - Place HTTP handlers in `routes/`, domain models in `models/`
  - Group related code into modules to increase maintainability and discoverability.
- Models should use `merak_macros::Model` to derive database operations.
- Route handlers should use `#[utoipa::path]` to generate OpenAPI documentation.

---

## Copilot prompt examples

Use the following example prompts when asking Copilot to generate or modify code. Tailor context to the specific file(s) and describe constraints.

- Add a new React component:
  - "Create a new React component `src/components/project_card.tsx`. This component should accept `className` and have default padding, rounded corners, and a subtle shadow."
  - "Add a new Shadcn component `card` via `pnpm dlx shadcn@latest add card`."

- Fix TypeScript types:
  - "Refactor `src/utils/parse-task.ts` to remove `any`. Replace with proper types `Task` and `TaskInput`. Preserve existing behavior and add a unit test covering the main parsing path."

- Add route or page:
  - "Add a new route `/projects` to the React Router setup in `src/main.tsx` and create `src/pages/Projects.tsx` that lists projects from a mocked API. Use `useEffect` and `useState`, keep the UI minimal, and use Tailwind for layout."

- Tailwind/CSS utility generation:
  - "Create a utility class for a responsive card in `src/components/ui/card.tsx`. The component should accept `className` and have default padding, rounded corners, and a subtle shadow."

- Rust crate change:
  - "Add a new API endpoint `POST /projects` that creates a new project. Follow existing error types and logging conventions. Add unit tests using the existing test harness."

Always include:
- File path(s) you intend to modify.
- Short rationale.
- Constraints (do not add dependencies, must pass `tsc -b`, must conform to `biome`).

---

## Examples of "do not" instructions

When prompting Copilot, explicitly forbid these actions unless you consciously choose them:

- Do not add or expose API keys, secrets, or credentials.
- Do not change the project's license or author metadata.
- Do not add heavy dependencies without a documented reason.
- Do not modify `packageManager` or version pinning without coordinating with maintainers.
- Do not make breaking changes to public crate APIs without a migration plan.

---

## Code review checklist (for generated or suggested changes)

When reviewing AI-generated code, check:

- Does TypeScript compile (`pnpm build` / `tsc -b`)?
- Does the change follow import alias usage (`@/*`)?
- Are props, types, and interfaces explicit where appropriate?
- Are UI components accessible (semantic HTML, labels)?
- Is Tailwind usage consistent and not overly verbose?
- Are tests added for non-trivial logic?
- Are there no hard-coded secrets or credentials?
- Is the change small and well-documented in the PR?

---

## Contact and escalation

- If Copilot suggests risky or wide-reaching changes (e.g., project-wide refactor, updating core build tool versions), ask a human maintainer to review before merging.
- Use the `author` field in `package.json` to find repository owner/contact for urgent questions.

---

## Maintainer notes

- Keep this file up to date as the repo evolves. If you add new top-level directories, build steps, or change the license, update this guidance.
- If Copilot repeatedly produces pattern-based errors for this repo, create a short FAQ here documenting those pitfalls and the correct patterns.
