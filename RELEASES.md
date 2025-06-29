# Version 0.1.2 (2025-06-28)
Bug Fixes:
- If stdin was empty, `ding` would panic. Whoops.

Chores:
- Update README with better zsh integration examples

# Version 0.1.1 (2025-06-28)

Ding is a project to help write curl commands based on an OpenAPI specification.

It does this by reading an OpenAPI specification, comparing that to a curl command (passed in from stdin), then re-writing the curl command to include Query, Header, and Body parameters.
