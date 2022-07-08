# ZelLayGen (Zellij Layout Generator)

Easy to generate `layout.yaml` for [Zellij](https://zellij.dev)

## Usage

* Edit `template.toml`
    * `name` : Name of session & yaml file - `{name}.yaml`
    * `dir` : Directory to your project dir - `~/path/to/project`

## Example

Generated yaml file from default `template.toml` is as follow.

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
