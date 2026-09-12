# Contributing to 4Forge

Thank you for your interest in contributing to **4Forge**! 4Forge is an open-source, trustworthy, local development environment manager for polyglot developers (PHP, Node.js, Python, Ruby) on Windows.

We are committed to maintaining a clean, secure, transparent, and auditable codebase under the **Apache-2.0** license.

---

## Developer Certificate of Origin (DCO)

All contributions to 4Forge must be accompanied by a Developer Certificate of Origin (DCO) sign-off line in the commit message. This certifies that you have the right to submit your contribution under the Apache-2.0 license.

To sign off your commit, use the `-s` or `--signoff` flag:

```bash
git commit -s -m "feat(supervisor): add windows job object handling"
```

The resulting commit message will contain:

```text
Signed-off-by: Your Name <your.email@example.com>
```

---

## Commit Guidelines

We enforce the [Conventional Commits](https://www.conventionalcommits.org/) standard.

### Format
```text
<type>(<scope>): <short summary>

[optional body]

[optional footer(s)]
Signed-off-by: Your Name <your.email@example.com>
```

### Types
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `chore`: Maintenance tasks, dependency updates, tooling
- `refactor`: Code changes that neither fix a bug nor add a feature
- `test`: Adding or correcting tests
- `ci`: CI/CD configuration changes

### Scopes
Examples: `supervisor`, `caddy`, `runtime`, `db`, `gui`, `deps`.

---

## Branching & Pull Request Process

1. Fork the repository and create a branch from `main`:
   - Feature branch: `feature/<description>`
   - Bug fix branch: `fix/<description>`
   - Documentation branch: `docs/<description>`
   - Maintenance branch: `chore/<description>`
2. Follow rustfmt, clippy, and code conventions.
3. Make sure all automated checks pass locally before opening a pull request:
   ```bash
   cargo fmt --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test
   cargo audit
   cargo deny check
   ```
4. Frontend checks (if applicable):
   ```bash
   pnpm run check
   pnpm run build
   ```
5. Open a Pull Request targeting `main`. Pull requests require review before merging. Merges into `main` use squash merges to maintain a clean linear history.

---

## Code Style

- **Rust**: Formatted with `cargo fmt`. No lints suppressed with `#[allow(...)]` without a clear explanatory comment. Prefer pushing control flow outward to callers.
- **Frontend**: Tailwind utility classes colocated with components. Keep components focused, reusable, and small.
- **Licensing**: All new source files must carry the standard Apache-2.0 header found in `docs/license-header.txt`.
