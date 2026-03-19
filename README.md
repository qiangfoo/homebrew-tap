# work

Interactive git worktree manager.

## Install

```sh
brew install qiangfoo/tap/work
```

Then add to your `~/.zshrc` or `~/.bashrc`:

```sh
eval "$(work init)"
```

## Usage

```sh
work          # select and switch to a worktree
work go       # same as above
work add      # create a new worktree (e.g. "feature" → 03-18-feature)
work remove   # remove a worktree
```

Worktrees created with `work add` are prefixed with the current date: `MM-DD-name`.

## Configuration

Create `~/.config/work/work.toml`:

```toml
default_repo = "~/code/myproject"
```

This lets you run `work` from outside a git repo and manage worktrees of the default repo.
