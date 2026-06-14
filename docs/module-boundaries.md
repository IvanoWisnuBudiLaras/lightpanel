# Module Boundaries

## Routes

Routes parse HTTP requests and call services. They should stay thin.

## Services

Services contain business logic, deployment logic, and system integration.

## Utils

Utils contain shared low-level helpers, including safe command execution and
path handling.

## Models

Models define internal data shapes and database-facing records.

## Frontend

Pages own view composition. Components should stay small. API calls belong in
`frontend/src/lib/api.ts`.

