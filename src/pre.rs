use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref CMDS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("help", "Shows this");
        map.insert("install", "Install dotfiles. [With Args: username/repo] [Or without]");
        map.insert("dist", "Pack your dotfiles");
        map.insert("info", "Shows info about your packed dotfiles");
        map
    };
}



pub fn help_gen() -> String {
    let mut help_str = String::new();
    for (name, desc) in CMDS.iter() {
        help_str += &format!("- {}{}|{}\n", 
                    name,
                    " ".repeat(12-name.len()),
                    desc);
    }
    return help_str;
}
