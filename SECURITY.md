# LightPanel Security Model

LightPanel is intended to manage VPS resources, so the default posture is
conservative.

## MVP Security Rules

- No arbitrary shell command execution.
- No browser terminal.
- No plugin system.
- No Docker socket access.
- No user-provided command executed directly.
- All system operations must be allowlisted.
- App names and domains must be strictly validated.
- Paths must be normalized and checked to prevent traversal.
- Destructive actions must write audit logs.
- Command execution must go through a safe command runner module.
- Never concatenate untrusted input into shell commands.

## Deferred Security Work

- Store passwords using Argon2id.
- Use secure HTTP-only cookies.
- Add CSRF protection for state-changing requests.
- Add rate limiting.
- Add role-based authorization if multi-user support is introduced.

## Firewall Safety

The first firewall module will inspect ufw state only. It must not enable,
disable, reset, or reload firewall rules automatically because that can lock out
SSH access.

