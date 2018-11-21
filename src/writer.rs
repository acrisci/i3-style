use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdout, BufReader};
use std::path::Path;

use theme::Theme;

fn leading_spaces(string: &String) -> String {
    let mut leading = String::new();

    for c in string.chars() {
        if c.is_whitespace() {
            leading.push(c);
        } else {
            break;
        }
    }

    leading
}

pub fn write_config(input: &String, output: Option<&String>, theme: &Theme) {
    let input_file = File::open(input).unwrap();
    let reader = BufReader::new(input_file);
    write_config_from_reader(reader, output, &theme);
}

pub fn write_config_from_reader(input: BufReader<File>, output: Option<&String>, theme: &Theme) {
    let mut writer = match output {
        Some(x) => {
            let path = Path::new(x.as_str());
            Box::new(File::create(&path).unwrap()) as Box<Write>
        }
        None => Box::new(stdout()) as Box<Write>,
    };

    let mut in_bar = false;
    let mut in_colors = false;
    let mut colors_found = false;
    let mut found_bar_colors = HashSet::new();
    let mut found_window_colors = HashSet::new();

    for line in input.lines() {
        let original_line = line.unwrap() + "\n";
        let leading = leading_spaces(&original_line);
        // TODO count leading spaces
        let line = original_line.trim();
        let mut vec: Vec<&str> = Vec::new();

        for word in line.split(' ') {
            if word != "" {
                vec.push(word);
            }
        }

        if vec.len() > 0 && !vec[0].starts_with("#") {
            if in_colors && vec[0] == "}" {
                let bar_colors = &theme.bar_colors.as_ref().unwrap();
                if !found_bar_colors.contains("separator") {
                    bar_colors.separator.as_ref().map(|color| {
                        writer.write(leading.as_bytes()).unwrap();
                        writer.write(leading.as_bytes()).unwrap();
                        writer.write(b"separator ").unwrap();
                        writer.write(color.as_bytes()).unwrap();
                        writer.write(b"\n").unwrap();
                    });
                }

                if !found_bar_colors.contains("background") {
                    bar_colors.background.as_ref().map(|color| {
                        writer.write(leading.as_bytes()).unwrap();
                        writer.write(leading.as_bytes()).unwrap();
                        writer.write(b"background ").unwrap();
                        writer.write(color.as_bytes()).unwrap();
                        writer.write(b"\n").unwrap();
                    });
                }

                if !found_bar_colors.contains("statusline") {
                    bar_colors.statusline.as_ref().map(|color| {
                        writer.write(leading.as_bytes()).unwrap();
                        writer.write(leading.as_bytes()).unwrap();
                        writer.write(b"statusline ").unwrap();
                        writer.write(color.as_bytes()).unwrap();
                        writer.write(b"\n").unwrap();
                    });
                }

                let group_names = vec![
                    "focused_workspace",
                    "active_workspace",
                    "inactive_workspace",
                    "urgent_workspace",
                ];
                for group_name in &group_names {
                    if found_bar_colors.contains(&group_name.to_string()) {
                        continue;
                    }
                    let group = match group_name.as_ref() {
                        "focused_workspace" => bar_colors.focused_workspace.as_ref(),
                        "active_workspace" => bar_colors.active_workspace.as_ref(),
                        "inactive_workspace" => bar_colors.inactive_workspace.as_ref(),
                        "urgent_workspace" => bar_colors.urgent_workspace.as_ref(),
                        _ => panic!("not reached"),
                    };
                    if group.is_none() {
                        continue;
                    }

                    let group = group.unwrap();

                    if group.border.is_none() || group.background.is_none() || group.text.is_none()
                    {
                        continue;
                    }

                    writer.write(leading.as_bytes()).unwrap();
                    writer.write(leading.as_bytes()).unwrap();
                    writer.write(group_name.as_bytes()).unwrap();
                    writer.write(b" ").unwrap();
                    writer
                        .write(group.border.as_ref().unwrap().as_bytes())
                        .unwrap();
                    writer.write(b" ").unwrap();
                    writer
                        .write(group.background.as_ref().unwrap().as_bytes())
                        .unwrap();
                    writer.write(b" ").unwrap();
                    writer
                        .write(group.text.as_ref().unwrap().as_bytes())
                        .unwrap();
                    writer.write(b" ").unwrap();

                    match group.indicator {
                        Some(ref color) => {
                            writer.write(b" ").unwrap();
                            writer.write(color.as_bytes()).unwrap();
                            ()
                        }
                        None => (),
                    };

                    writer.write(b"\n").unwrap();
                }

                in_colors = false;
                found_bar_colors.clear();
                writer.write(original_line.as_bytes()).unwrap();
                continue;
            } else if in_bar && vec[0] == "}" {
                let bar_colors = &theme.bar_colors.as_ref().unwrap();
                if !colors_found {
                    writer.write(b"  colors {\n").unwrap();
                    bar_colors.separator.as_ref().map(|color| {
                        writer.write(b"    separator ").unwrap();
                        writer.write(color.as_bytes()).unwrap();
                        writer.write(b"\n").unwrap();
                    });
                    bar_colors.background.as_ref().map(|color| {
                        writer.write(b"    background ").unwrap();
                        writer.write(color.as_bytes()).unwrap();
                        writer.write(b"\n").unwrap();
                    });
                    bar_colors.statusline.as_ref().map(|color| {
                        writer.write(b"    statusline ").unwrap();
                        writer.write(color.as_bytes()).unwrap();
                        writer.write(b"\n").unwrap();
                    });

                    let group_names = vec![
                        "focused_workspace",
                        "active_workspace",
                        "inactive_workspace",
                        "urgent_workspace",
                    ];
                    for group_name in &group_names {
                        let group = match group_name.as_ref() {
                            "focused_workspace" => bar_colors.focused_workspace.as_ref(),
                            "active_workspace" => bar_colors.active_workspace.as_ref(),
                            "inactive_workspace" => bar_colors.inactive_workspace.as_ref(),
                            "urgent_workspace" => bar_colors.urgent_workspace.as_ref(),
                            _ => panic!("not reached"),
                        };
                        if group.is_none() {
                            continue;
                        }

                        let group = group.unwrap();

                        if group.border.is_none()
                            || group.background.is_none()
                            || group.text.is_none()
                        {
                            continue;
                        }

                        writer.write(b"    ").unwrap();
                        writer.write(group_name.as_bytes()).unwrap();
                        writer.write(b" ").unwrap();
                        writer
                            .write(group.border.as_ref().unwrap().as_bytes())
                            .unwrap();
                        writer.write(b" ").unwrap();
                        writer
                            .write(group.background.as_ref().unwrap().as_bytes())
                            .unwrap();
                        writer.write(b" ").unwrap();
                        writer
                            .write(group.text.as_ref().unwrap().as_bytes())
                            .unwrap();
                        writer.write(b" ").unwrap();

                        match group.indicator {
                            Some(ref color) => {
                                writer.write(b" ").unwrap();
                                writer.write(color.as_bytes()).unwrap();
                                ()
                            }
                            None => (),
                        };

                        writer.write(b"\n").unwrap();
                    }
                    writer.write(b"  }\n").unwrap();
                }

                colors_found = false;
                in_bar = false;
                writer.write(original_line.as_bytes()).unwrap();
                continue;
            }

            if in_colors {
                if theme.bar_colors.is_none() {
                    writer.write(original_line.as_bytes()).unwrap();
                    continue;
                }

                let bar_colors = &theme.bar_colors.as_ref().unwrap();

                if vec!["separator", "background", "statusline"].contains(&vec[0]) {
                    found_bar_colors.insert(vec[0].to_string());
                    writer.write(leading.as_bytes()).unwrap();
                    writer.write(vec[0].as_bytes()).unwrap();
                    writer.write(b" ").unwrap();

                    writer
                        .write(match vec[0] {
                            "separator" => match bar_colors.separator {
                                Some(ref color) => color.as_bytes(),
                                None => vec[1].as_bytes(),
                            },
                            "background" => match bar_colors.background {
                                Some(ref color) => color.as_bytes(),
                                None => vec[1].as_bytes(),
                            },
                            "statusline" => match bar_colors.statusline {
                                Some(ref color) => color.as_bytes(),
                                None => vec[1].as_bytes(),
                            },
                            _ => vec[1].as_bytes(),
                        })
                        .unwrap();
                    writer.write(b"\n").unwrap();
                    continue;
                } else if vec![
                    "focused_workspace",
                    "active_workspace",
                    "inactive_workspace",
                    "urgent_workspace",
                ]
                .contains(&vec[0])
                {
                    found_bar_colors.insert(vec[0].to_string());
                    let group = match vec[0] {
                        "focused_workspace" => bar_colors.focused_workspace.as_ref(),
                        "active_workspace" => bar_colors.active_workspace.as_ref(),
                        "inactive_workspace" => bar_colors.inactive_workspace.as_ref(),
                        "urgent_workspace" => bar_colors.urgent_workspace.as_ref(),
                        _ => panic!("not reached"),
                    };

                    if group.is_none() {
                        writer.write(original_line.as_bytes()).unwrap();
                        continue;
                    }

                    let group = group.unwrap();

                    writer.write(leading.as_bytes()).unwrap();
                    writer.write(vec[0].as_bytes()).unwrap();
                    writer.write(b" ").unwrap();

                    writer
                        .write(match group.border.as_ref() {
                            Some(color) => color.as_bytes(),
                            None => vec[1].as_bytes(),
                        })
                        .unwrap();
                    writer.write(b" ").unwrap();

                    writer
                        .write(match group.background.as_ref() {
                            Some(color) => color.as_bytes(),
                            None => vec[2].as_bytes(),
                        })
                        .unwrap();
                    writer.write(b" ").unwrap();

                    writer
                        .write(match group.text.as_ref() {
                            Some(color) => color.as_bytes(),
                            None => vec[3].as_bytes(),
                        })
                        .unwrap();

                    if vec.get(3).is_some() || group.indicator.is_some() {
                        writer.write(b" ").unwrap();
                        writer
                            .write(match group.indicator.as_ref() {
                                Some(color) => color.as_bytes(),
                                None => vec[3].as_bytes(),
                            })
                            .unwrap();
                    }
                    writer.write(b"\n").unwrap();

                    continue;
                }
                continue;
            }

            if vec[0] == "bar" {
                in_bar = true;
                writer.write(original_line.as_bytes()).unwrap();
                continue;
            }
            if in_bar && vec[0] == "colors" {
                in_colors = true;
                colors_found = true;
                writer.write(original_line.as_bytes()).unwrap();
                continue;
            }

            if vec![
                "client.focused",
                "client.unfocused",
                "client.focused_inactive",
                "client.urgent",
            ]
            .contains(&vec[0])
            {
                found_window_colors.insert(vec[0].to_string());
                if theme.window_colors.is_none() {
                    writer.write(original_line.as_bytes()).unwrap();
                    continue;
                }

                let window_colors = &theme.window_colors.as_ref().unwrap();

                let group = match vec[0] {
                    "client.focused" => window_colors.focused.as_ref(),
                    "client.unfocused" => window_colors.unfocused.as_ref(),
                    "client.focused_inactive" => window_colors.focused_inactive.as_ref(),
                    "client.urgent" => window_colors.urgent.as_ref(),
                    _ => panic!("not reached"),
                };

                if group.is_none() {
                    writer.write(original_line.as_bytes()).unwrap();
                    continue;
                }

                let group = group.unwrap();

                writer.write(leading.as_bytes()).unwrap();
                writer.write(vec[0].as_bytes()).unwrap();
                writer.write(b" ").unwrap();

                writer
                    .write(match group.border.as_ref() {
                        Some(color) => color.as_bytes(),
                        None => vec[1].as_bytes(),
                    })
                    .unwrap();
                writer.write(b" ").unwrap();

                writer
                    .write(match group.background.as_ref() {
                        Some(color) => color.as_bytes(),
                        None => vec[2].as_bytes(),
                    })
                    .unwrap();
                writer.write(b" ").unwrap();

                if vec.get(3).is_some() || group.text.is_some() {
                    writer
                        .write(match group.text.as_ref() {
                            Some(color) => color.as_bytes(),
                            None => vec[3].as_bytes(),
                        })
                        .unwrap();
                    writer.write(b" ").unwrap();

                    // cant write the indicator in the text field
                    if vec.get(4).is_some() || group.indicator.is_some() {
                        writer
                            .write(match group.indicator.as_ref() {
                                Some(color) => color.as_bytes(),
                                None => vec[4].as_bytes(),
                            })
                            .unwrap();
                    }
                }

                writer.write(b"\n").unwrap();
                continue;
            }
        }

        writer.write(original_line.as_bytes()).unwrap();
    }

    let window_color_names = vec![
        "client.focused",
        "client.focused_inactive",
        "client.unfocused",
        "client.urgent",
    ];
    for window_color_name in &window_color_names {
        if found_window_colors.contains(&window_color_name.to_string()) {
            continue;
        }
        let window_colors = &theme.window_colors.as_ref().unwrap();

        let group = match window_color_name.as_ref() {
            "client.focused" => window_colors.focused.as_ref(),
            "client.unfocused" => window_colors.unfocused.as_ref(),
            "client.focused_inactive" => window_colors.focused_inactive.as_ref(),
            "client.urgent" => window_colors.urgent.as_ref(),
            _ => panic!("not reached"),
        };

        if group.is_none() {
            continue;
        }

        let group = group.unwrap();

        writer.write(window_color_name.as_bytes()).unwrap();
        writer.write(b" ").unwrap();
        writer
            .write(match group.border.as_ref() {
                Some(color) => color.as_bytes(),
                None => b"#000000",
            })
            .unwrap();
        writer.write(b" ").unwrap();

        writer
            .write(match group.background.as_ref() {
                Some(color) => color.as_bytes(),
                None => b"#000000",
            })
            .unwrap();
        writer.write(b" ").unwrap();

        writer
            .write(match group.text.as_ref() {
                Some(color) => color.as_bytes(),
                None => b"#000000",
            })
            .unwrap();

        group.indicator.as_ref().map(|color| {
            writer.write(b" ").unwrap();
            writer.write(color.as_bytes()).unwrap();
        });

        writer.write(b"\n").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate tempfile;
    extern crate yaml_rust;

    use self::tempfile::tempdir;
    use std::path::PathBuf;
    use yaml_rust::YamlLoader;

    fn get_file_contents(path: &String) -> String {
        let mut file = File::open(path).expect("could not open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        contents
    }

    fn get_resource_contents(path: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test-resources");
        d.push(path);
        get_file_contents(&d.to_str().unwrap().to_string())
    }

    #[test]
    fn test_minimal_config_template() {
        let contents = get_resource_contents("test-theme.yaml");

        let docs =
            YamlLoader::load_from_str(contents.as_str()).expect("Could not parse yaml for theme");
        let doc = &docs[0];

        let theme = from_yaml(&doc);

        let dir = tempdir().unwrap();
        let output_path = dir
            .path()
            .join("writer-test-output")
            .to_str()
            .unwrap()
            .to_string();

        let configs = vec![
            vec!["test-resources/minimal-config", "minimal-config-expected"],
            vec!["test-resources/missing-config", "missing-config-expected"],
        ];

        for config in &configs {
            let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            d.push(&config[0]);
            let input_path = d.to_str().unwrap().to_string();

            write_config(&input_path, Some(&output_path), &theme);

            let contents = get_file_contents(&output_path);
            let expected_contents = get_resource_contents(&config[1]);

            println!("'{}'", contents);
            println!("'{}'", expected_contents);
            assert_eq!(contents, expected_contents);
        }
    }
}
