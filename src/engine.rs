use serde_yaml::{Value, Mapping};
use std::fs;
use crate::Errors;


pub fn install(usrrepo: String) {
    match usrrepo.split_once("/") {
        Some((user, repo)) => {
            let user=user; let repo=repo;
        },
        None => {
            Errors::wformat_usrepo(usrrepo);
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
                    return {
                        if mandatory {Errors::missing_field(path.to_string());}
                        "null".to_string()
                    };
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

fn ybas()->String {
    return String::from("null");
}

pub fn finstall() {
    let contents = fs::read_to_string(".mate").expect("Failed to read file");
    let value: Value = serde_yaml::from_str(&contents).unwrap();
    
    let mut author=ybas();
    let mut repo  =ybas();
    if let Value::Mapping(map) = value {
        //author = vparse(&map["infoz"]["author"], "infoz->author");
        author = vparse(map.clone(), &["info", "author"], true);
        repo   = vparse(map.clone(), &["info", "repo"]  , false);
    }

    println!("{}", author);
    println!("{}", repo);
}
