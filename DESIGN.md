# LightPanel Design

## Philosophy

LightPanel should feel like a precise VPS control surface, not a generic admin
template. The UI is dark-first, compact, readable, and biased toward operational
clarity over decoration.

## Color System

Colors live in `frontend/src/styles/tokens.css`. Use semantic tokens such as
`--bg-panel`, `--border`, `--text-muted`, `--accent`, `--red`, and `--green`.
Avoid hardcoded colors in pages unless a new token is first justified.

## Typography

Use the system sans stack for UI text and a monospace stack only for logs,
commands, paths, ports, and code-like values. Headings should be restrained:
panel titles around 15-18px, not marketing-scale.

## Spacing

Use the spacing variables from `tokens.css`. The interface should maintain
medium-high information density: panels are compact, rows are scan-friendly, and
empty space should create grouping rather than a landing-page feel.

## Layout Rules

- Fixed sidebar on desktop.
- Sticky topbar.
- Main content scrolls independently.
- Keep laptop layouts useful before optimizing mobile.
- Prefer two-column operational layouts over wide blank pages.

## Component Rules

- Keep components small and focused.
- Use `SectionHeader` for panel titles and explanations.
- Use `MetricCard` and `StatusCard` for compact data.
- Use badges for runtime and app status.
- Use `LogViewer` for terminal-like output only.

## Anti Generic Admin Rules

- No oversized cards with vague icons.
- No Bootstrap-style blue primary everywhere.
- No fake SaaS hero sections.
- No heavy gradients, glassmorphism, or decorative blobs.
- No placeholder that looks like a working destructive action.

## Dashboard Hierarchy

The dashboard should show server identity first, then operational readiness:
resources, runtimes, active services, deployment health, security posture,
recent activity, and quick actions.

## Lightweight UI

Use Svelte, fetch, and plain CSS. Do not add chart libraries or UI frameworks
until a real interaction proves the need.
