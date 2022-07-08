# ZelLayGen (Zellij Layout Generator)

Easy to generate `layout.yaml` for [Zellij](https://zellij.dev)

## Usage

1. Copy `config_template.toml` to `config.toml`
* Edit `config.toml`
    * `name` : Name of session & yaml file - `{name}.yaml`
    * `dir` : Directory to your project dir - `~/path/to/project`
    * `editor_cmd` : Editor open dir command (e.g. `nvim`, `code .` and etc.)
    * `tree_tool` : Tool for file tree view (e.g. `broot`)
    * `monitor_tool` : Tool for monitor (e.g. `htop`, `btm`, `btop`, `ytop` and etc.)
    * `git_tool` : Tool for git (e.g. `gitui`)

## Example

Generated yaml file from default `config_template.toml` is as follow.

```yaml
---
session:
  name: "test"
  attach: true
template:
  direction: Horizontal
  parts:
    - direction: Vertical  # part 1
      borderless: true
      split_size:
        Fixed: 1
      run:
        plugin:
          location: "zellij:tab-bar"
    - direction: Vertical # part 2
      body: true
    - direction: Vertical # part 3
      borderless: true
      split_size:
        Fixed: 2
      run:
        plugin:
            location: "zellij:status-bar"
tabs:
  - name: "work"
    focus: true
    direction: Vertical
    parts:
      - direction: Horizontal
        run:
          command: { cmd: zsh, args: ["-c", "cd ~/path/to/project && nvim"] }
        focus: true
      - direction: Horizontal
        parts:
          - direction: Vertical
            run:
              command: { cmd: zsh, args: ["-c", "cd ~/path/to/project && broot"] }
          - direction: Vertical
            run:
              command: { cmd: zsh, args: ["-c", "cd ~/path/to/project && zsh"] }
  - name: "perf"
    direction: Vertical
    run:
      command: { cmd: btm }
  - name: "git"
    direction: Vertical
    run:
      command: { cmd: zsh, args: ["-c", "cd ~/path/to/project && gitui"] }
```
