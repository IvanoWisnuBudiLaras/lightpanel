# Security Model

The backend is the only component allowed to perform system operations.

System operations must be:

- Allowlisted.
- Validated.
- Audited when destructive.
- Isolated in service or command modules.
- Implemented without shell string concatenation.

Authentication, sessions, CSRF, and password hashing are planned but not
implemented in Step 1.

