# Nysh

**SSH identity manager for GitHub, built for the terminal.**

Nysh is a terminal application for managing multiple SSH identities used with GitHub. It helps developers create or import SSH keys, configure SSH host aliases, test authentication, and use the correct identity for each Git repository.

Nysh provides both a **CLI and an interactive TUI**, sharing the same core functionality.

## Problem

Working with multiple GitHub accounts can be tedious when SSH identities and configuration are managed manually. Common pain points include:

- Keeping personal, work, university, and project identities separate.
- Maintaining `~/.ssh/config` safely.
- Accidentally using the wrong GitHub account for a repository.
- Repeatedly troubleshooting SSH authentication failures.
- Manually changing Git remote URLs to use the correct SSH identity.
- Keeping track of which SSH key belongs to which account.

Nysh aims to simplify these workflows without hiding how SSH and Git work.

## Goals

- Manage multiple GitHub SSH identities from one terminal application.
- Generate new SSH keys or import existing ones.
- Create and maintain SSH host aliases for separate GitHub accounts.
- Test SSH authentication and provide useful diagnostics.
- Inspect the current Git repository and its remote.
- Help assign a repository to the intended SSH identity.
- Keep private keys in the user's normal SSH directory.
- Provide both interactive and scriptable workflows.

## Planned Features

### v0.1 — SSH Identity Management

- Detect existing SSH keys and configuration.
- Generate Ed25519 key pairs.
- Import existing keys.
- Display and copy public keys.
- Create SSH aliases for GitHub accounts.
- Test authentication with GitHub.
- Inspect existing SSH configuration.
- Avoid overwriting existing keys or unrelated SSH configuration.
- Provide clear errors and safe confirmations for destructive actions.

### v0.2 — Git Repository Integration

- Detect the current Git repository.
- Display its remote URL and SSH host alias.
- Associate a repository with a selected identity.
- Update a remote URL after showing the proposed change and requesting confirmation.
- Clone repositories using a selected identity.
- Diagnose common Git remote and SSH authentication issues.

### v0.3 — CLI, Configuration & Polish

- Provide CLI commands alongside the TUI.
- Add a user-editable TOML configuration file.
- Improve diagnostics and recovery guidance.
- Add automated tests.
- Improve documentation and installation instructions.
- Prepare packaging and distribution.

### Future Possibilities

- Support other Git hosting providers such as GitLab and Bitbucket.
- Optional SSH-agent inspection and integration.
- Additional Git workflows based on real-world usage.
- Support for more advanced identity and repository management.

## CLI & TUI

Nysh is designed around a shared application core with two interfaces.

The **CLI** is intended for direct operations, scripting, and automation:

```bash
nysh list
nysh test github-personal
nysh generate github-work
```

The **TUI** is intended for interactive identity management:

```bash
nysh
```

The exact commands and interface will evolve during development.

Both interfaces should use the same underlying logic rather than implementing separate versions of the application's behavior.

## Example: Multiple GitHub Accounts

Nysh can help configure separate SSH aliases in `~/.ssh/config`:

```sshconfig
Host github-personal
    HostName github.com
    User git
    IdentityFile ~/.ssh/id_github_personal
    IdentitiesOnly yes

Host github-work
    HostName github.com
    User git
    IdentityFile ~/.ssh/id_github_work
    IdentitiesOnly yes
```

The aliases can then be used when cloning repositories:

```bash
git clone git@github-personal:username/repository.git
git clone git@github-work:company/repository.git
```

For an existing repository, Nysh can help update its remote to use the intended alias. Before making the change, it should display the resulting URL and request confirmation.

SSH authentication and Git commit authorship are separate concerns. The SSH identity determines authentication, while the name and email recorded in commits are configured independently through Git.

## Interface Direction

Nysh follows Nyverune's visual identity: **dark, minimal, technical, and focused on usability**.

The TUI should prioritize:

- Clear presentation of available SSH identities.
- Key type and file path for the selected identity.
- Authentication status.
- Contextual actions and keyboard shortcuts.
- Explicit confirmations for destructive operations.
- Readable output in terminals with limited color support.

The interface should remain useful without relying entirely on color.

## Technology

The initial implementation is planned in Rust.

| Dependency | Purpose |
| --- | --- |
| `ratatui` | Terminal user interface |
| `crossterm` | Terminal input and control |
| `clap` | CLI arguments and subcommands |
| `serde` + `toml` | Configuration serialization |
| `anyhow` / `thiserror` | Error handling |
| `directories` | User configuration paths |

Tokio or other asynchronous infrastructure should only be introduced if a concrete use case requires it.

## Configuration

Nysh will have its own configuration, separate from the shared Nyverune theme:

```text
~/.config/nysh/
└── config.toml
```

Example:

```toml
[appearance]
theme = "nyverune"

[ssh]
key_type = "ed25519"
default_directory = "~/.ssh"
```

The configuration schema may evolve during development. Nysh should work with sensible defaults when no configuration file exists.

## Security Principles

Nysh is designed to manage SSH identities without compromising private key security.

- Never upload, transmit, or log private keys.
- Never store GitHub tokens or passwords in plaintext.
- Keep private keys in the user's normal SSH directory.
- Check file paths and permissions when importing keys.
- Never overwrite keys or unrelated SSH configuration without explicit consent.
- Never delete private-key files automatically when removing an identity.
- Show and confirm Git remote changes before applying them.
- Treat a configured identity and a successfully authenticated GitHub account as separate states.

Nysh does not require the GitHub API for its core SSH workflows. SSH and local Git configuration are sufficient for the initial release.

## Architecture

Nysh should remain independent from any Nyverune-specific shared framework during its initial development.

The application will be structured around a shared core that can be consumed by both the CLI and TUI:

```text
┌─────────────┐     ┌─────────────┐
│     CLI     │     │     TUI     │
└──────┬──────┘     └──────┬──────┘
       │                   │
       └─────────┬─────────┘
                 │
          ┌──────▼──────┐
          │    Core     │
          └──────┬──────┘
                 │
       ┌─────────┼─────────┐
       │         │         │
     SSH       Git      Config
```

Once multiple Nyverune applications reveal genuinely shared patterns, reusable components can be extracted into a separate crate such as `nyverune-tui`.

## Development Approach

Nysh should evolve incrementally from safe SSH identity management toward Git repository integration.

The initial implementation should prioritize correctness and safety over feature breadth. Operations that modify SSH configuration, keys, or Git remotes should always be explicit and reversible where possible.

## Contributing

Contributions, bug reports, and feature suggestions are welcome.

Before implementing a major feature, open an issue to discuss the use case and proposed behavior.

Never include private keys, access tokens, passwords, or other credentials in issues, pull requests, logs, or screenshots.

## License

Nysh is licensed under the **MIT License**.

See [`LICENSE`](LICENSE) for the full license text.

