use serde_yaml::{Value, Mapping};
use std::fs;
use crate::Errors;
use crate::packages;


pub fn install(usrrepo: &str) {
    match usrrepo.split_once("/") {
        Some((user, repo)) => {
            let _user=user; let _repo=repo;
        },
        None => {
            Errors::wformat_usrepo(usrrepo.to_string());
        }
    }
}

fn vparse(map: Mapping, vals: &[&str], mandatory: bool) -> String {
    let mut val = &Value::Mapping(map);
    let path = vals.join("->");
    for vall in vals {
        val = match val {
            Value::Mapping(map) => match map.get(*vall) {
                Some(v) => v,
                None => {
                    if mandatory {Errors::missing_field(path.to_string());}
                    return "null".to_string()
                }
            },
            _ => {
                Errors::missing_field(path.to_string());
                return "null".to_string();
            }
        };
    }
    match val {
        Value::String(variable) => variable.to_string(),
        _ => {
            if mandatory {Errors::missing_field(path.to_string());}
            "null".to_string()
        }
    }
}

fn mparse(map: Mapping, distro: String) -> Value {
    if let Some(dependencies_value) = map.get(&Value::String("dependencies".to_string())) {
        if let Some(os_value) = dependencies_value.get(&Value::String(distro.clone())) {
            return os_value.clone();
        } else {
            return ybam();
        }
    } else {
        return ybam();
    }
}

fn ybas()->String {
    return String::from("null");
}

fn ybam()->Value{
    return Value::Sequence(vec![]);
}

fn gld() -> String {
    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        for line in contents.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 && parts[0] == "ID" {
                return parts[1].trim_matches('"').to_string();
            }
        }
    }
    return "none".to_string();
}

fn cnv(values: &Vec<Value>) -> Vec<String> {
    values.iter().map(|v| v.as_str().unwrap().to_string()).collect()
}

pub fn finstall() {
    let contents = fs::read_to_string(".mate").expect("Failed to read file");
    let value: Value = serde_yaml::from_str(&contents).unwrap();
    
    let mut author=ybas();
    let mut repo  =ybas();
    let mut deps  =ybam();
    if let Value::Mapping(map) = value {
        //author = vparse(&map["infoz"]["author"], "infoz->author");
        author = vparse(map.clone(), &["info", "author"]        , true );
        repo   = vparse(map.clone(), &["info", "repo"]          , false);
        deps   = mparse(map.clone(), gld());
    }
    
    if deps==ybam() {
       Errors::missing_distro();
    }
    else {
        packages::recognizer(gld(), cnv(deps.as_sequence().unwrap()));
    }
    println!("{}", author);
    println!("{}", repo);
}
