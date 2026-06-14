# Architecture

LightPanel uses a native deployment model.

- Backend: Rust with axum.
- Frontend: Svelte SPA built by Vite.
- Internal database: SQLite.
- Process manager: systemd.
- Reverse proxy: Nginx.
- SSL: acme.sh integration later.

The backend owns system operations. The frontend only calls allowlisted HTTP
APIs and never executes commands directly.

