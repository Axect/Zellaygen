use std::fs::read_to_string;
use std::fs;
use indoc::formatdoc;
use serde_derive::Deserialize;

fn main() {
    let config = Template::parse_toml("config.toml");
    let yaml = formatdoc! {"
        ---
        session:
          name: \"{name}\"
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
                  location: \"zellij:tab-bar\"
            - direction: Vertical # part 2
              body: true
            - direction: Vertical # part 3
              borderless: true
              split_size:
                Fixed: 2
              run:
                plugin:
                    location: \"zellij:status-bar\"
        tabs:
          - name: \"work\"
            focus: true
            direction: Vertical
            parts:
              - direction: Horizontal
                run:
                  command: {{ cmd: zsh, args: [\"-c\", \"cd {dir} && {editor_cmd}\"] }}
                focus: true
              - direction: Horizontal
                parts:
                  - direction: Vertical
                    run:
                      command: {{ cmd: zsh, args: [\"-c\", \"cd {dir} && {tree}\"] }}
                  - direction: Vertical
                    run:
                      command: {{ cmd: zsh, args: [\"-c\", \"cd {dir} && zsh\"] }}
          - name: \"perf\"
            direction: Vertical
            run:
              command: {{ cmd: {monitor} }}
          - name: \"git\"
            direction: Vertical
            run:
              command: {{ cmd: zsh, args: [\"-c\", \"cd {dir} && {git}\"] }}
        ",
        name = config.get_name(),
        dir = config.get_dir(),
        editor_cmd = config.get_editor_cmd(),
        tree = config.get_tree_tool(),
        monitor = config.get_monitor_tool(),
        git = config.get_git_tool(),
    };

    let yaml_file_name = format!("{}.yaml", config.get_name());
    fs::write(&yaml_file_name, yaml).unwrap();

    println!("Generating {} completed!", yaml_file_name);
}

#[derive(Debug, Deserialize)]
struct Template {
    config: Config
}

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    dir: String,
    editor_cmd: String,
    tree_tool: String,
    monitor_tool: String,
    git_tool: String,
}

impl Template {
    fn parse_toml(toml_path: &str) -> Self {
        let config = read_to_string(toml_path).expect("Can't read config.toml");
        toml::from_str(&config).expect("Can't parse config.toml")
    }

    fn get_name(&self) -> &str {
        &self.config.name
    }

    fn get_dir(&self) -> &str {
        &self.config.dir
    }

    fn get_editor_cmd(&self) -> &str {
        &self.config.editor_cmd
    }

    fn get_tree_tool(&self) -> &str {
        &self.config.tree_tool
    }

    fn get_monitor_tool(&self) -> &str {
        &self.config.monitor_tool
    }

    fn get_git_tool(&self) -> &str {
        &self.config.git_tool
    }
}
