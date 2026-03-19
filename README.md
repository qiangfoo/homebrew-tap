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

```sh
work config set default_repo ~/code/myproject   # set a config value
work config get default_repo                     # print a config value
work config list                                 # print all settings
```

Config is stored in `~/.config/work/work.toml`.

### Options

| Key | Description |
|---|---|
| `default_repo` | Path to a git repo. Lets you run `work` from outside a git repo. |
