use std::collections::HashMap;

#[derive(Debug)]
pub struct Palette {
   pub name: String,
   pub colors: HashMap<String, String>,
}

    // TODO: Moves these and rearrange
    // pref store them in json or YAML or smth...
    //let solarized = Palette {
    //    name: "Solarized Osaka".to_string(),
    //    colors: HashMap::from([
    //        ("base03".to_string(), "#001419".to_string()),
    //        ("base02".to_string(), "#002b36".to_string()),
    //        ("base01".to_string(), "#586e75".to_string()),
    //        ("base00".to_string(), "#657b83".to_string()),
    //        ("base0".to_string(), "#839496".to_string()),
    //        ("base1".to_string(), "#93a1a1".to_string()),
    //        ("base2".to_string(), "#eee8d5".to_string()),
    //        ("base3".to_string(), "#fdf6e3".to_string()),
    //        ("yellow".to_string(), "#b58900".to_string()),
    //        ("orange".to_string(), "#cb4b16".to_string()),
    //        ("red".to_string(), "#dc322f".to_string()),
    //        ("magenta".to_string(), "#d33682".to_string()),
    //        ("violet".to_string(), "#6c71c4".to_string()),
    //        ("blue".to_string(), "#268bd2".to_string()),
    //        ("cyan".to_string(), "#2aa198".to_string()),
    //        ("green".to_string(), "#859900".to_string()),
    //    ]),
    //};

    //let catppuccin = Palette {
    //    name: "Catppuccin Mocha".to_string(),
    //    colors: HashMap::from([
    //        ("base03".to_string(), "#11111b".to_string()),
    //        ("base02".to_string(), "#181825".to_string()),
    //        ("base01".to_string(), "#313244".to_string()),
    //        ("base00".to_string(), "#45475a".to_string()),
    //        ("base0".to_string(), "#cdd6f4".to_string()),
    //        ("base1".to_string(), "#bac2de".to_string()),
    //        ("base2".to_string(), "#f5e0dc".to_string()),
    //        ("base3".to_string(), "#f2cdcd".to_string()),
    //        ("yellow".to_string(), "#f9e2af".to_string()),
    //        ("orange".to_string(), "#fab387".to_string()),
    //        ("red".to_string(), "#f38ba8".to_string()),
    //        ("magenta".to_string(), "#f5c2e7".to_string()),
    //        ("violet".to_string(), "#cba6f7".to_string()),
    //        ("blue".to_string(), "#89b4fa".to_string()),
    //        ("cyan".to_string(), "#94e2d5".to_string()),
    //        ("green".to_string(), "#a6e3a1".to_string()),
    //    ]),
    //};

    //    let content = String::from(
    //        "[colors]
    //base03 = #11111b
    //base02 = #181825
    //base01 = #313244
    //base00 = #45475a
    //base0  = #cdd6f4
    //base1  = #bac2de
    //base2  = #f5e0dc
    //base3  = #f2cdcd
    //
    //yellow   = #f9e2af
    //orange   = #fab387
    //red      = #f38ba8
    //magenta  = #f5c2e7
    //violet   = #cba6f7
    //blue     = #89b4fa
    //cyan     = #94e2d5
    //green    = #a6e3a1
    //",
    //    );

