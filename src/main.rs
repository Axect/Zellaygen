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
                  command: {{ cmd: zsh, args: [\"-c\", \"cd {dir} && nvim\"] }}
                focus: true
              - direction: Horizontal
                parts:
                  - direction: Vertical
                    run:
                      command: {{ cmd: zsh, args: [\"-c\", \"cd {dir} && broot\"] }}
                  - direction: Vertical
                    run:
                      command: {{ cmd: zsh, args: [\"-c\", \"cd {dir} && zsh\"] }}
          - name: \"perf\"
            direction: Vertical
            run:
              command: {{ cmd: btm }}
          - name: \"git\"
            direction: Vertical
            run:
              command: {{ cmd: zsh, args: [\"-c\", \"cd {dir} && gitui\"] }}
        ",
        name = config.get_name(),
        dir = config.get_dir(),
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
    dir: String
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
}
