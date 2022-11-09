use std::fs::read_to_string;
use std::fs;
use std::env;
use indoc::formatdoc;
use serde_derive::Deserialize;

fn main() {
    let config = Template::parse_toml("config.toml");
    let kdl = formatdoc! {"
        layout {{
            default_tab_template {{
                pane size=1 borderless=true {{
                    plugin location=\"zellij:tab-bar\"
                }}
                children
                pane size=2 borderless=true {{
                    plugin location=\"zellij:status-bar\"
                }}
            }}
            tab name=\"work\" focus=true split_direction=\"Vertical\" {{
                pane split_direction=\"Vertical\" {{
                    pane name=\"editor\" command=\"{shell}\" focus=true {{
                        args \"-c\" \"cd {dir} && {editor_cmd}\"
                    }}
                    pane split_direction=\"Horizontal\" {{
                        pane name=\"files\" command=\"{shell}\" {{
                            args \"-c\" \"cd {dir} && {tree}\"
                        }}
                        pane name=\"shell\" command=\"{shell}\" {{
                            args \"-c\" \"cd {dir} && {shell}\"
                        }}
                    }}
                }}
            }}
            tab name=\"perf\" {{
                pane name=\"monitor\" command=\"{monitor}\"
            }}
            tab name=\"git\" {{
                pane name=\"git\" {{
                    command \"{shell}\"
                    args \"-c\" \"cd {dir} && {git}\"
                }}
            }}
        }}
        session_name \"{name}\"
        attach_to_session true
        ",
        name = config.get_name(),
        dir = config.get_dir(),
        editor_cmd = config.get_editor_cmd(),
        tree = config.get_tree_tool(),
        monitor = config.get_monitor_tool(),
        git = config.get_git_tool(),
        shell = env::var("SHELL").unwrap_or_else(|_| "sh".to_string()),
    };

    let kdl_file_name = format!("{}.kdl", config.get_name());
    fs::write(&kdl_file_name, kdl).unwrap();

    println!("Generating {} completed!", kdl_file_name);
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
