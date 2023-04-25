use serde_yaml::Value;

struct arch;
impl arch {
    fn community() {}
}


pub fn recognizer(distro:String, pkgs: Vec<String>) {
    println!("{:?}", pkgs);
}
