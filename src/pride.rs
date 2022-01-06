use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use crate::{RGBA, to_rgba};

pub struct PrideState {
    pub(crate) name: String,
    pub(crate) colors: HashMap<String,RGBA>,
    pub(crate) ratio: Vec<i64>,
    pub(crate) flag: Vec<RGBA>
}

pub fn read_pride_file(path: &str) -> Option<PrideState> {
    let mut state = PrideState {
        name: "".to_string(),
        colors: Default::default(),
        ratio: vec![],
        flag: vec![]
    };

    let data = fs::read_to_string(path).expect("error reading file");
    if data == "error reading file" {
        return None
    }
    for str in data.split("\n") {
        if str.contains("Name: ") {
            state.name = str.replace("Name: ", "")
        }

        if str.contains("Ratio: ") {
            let ratio = str.trim().replace("Ratio: ", "");
            if ratio != "1" {
                for int in ratio.split(":").map(|it|i64::from_str(it)) {
                    match int  {
                        Ok(x) => {state.ratio.push(x)}
                        Err(_) => {println!("Well it failed on the ratio parsing");return None}
                    }
                }
            }
        }

        if str.contains("=#") {
            let data : Vec<&str> = str.split("=#").collect();
            let name = data[0].trim();
            let hex_str = data[1].trim();
            let hex = u32::from_str_radix(hex_str, 16);
            match hex {
                Ok(yo) => {
                    let color = to_rgba(yo);
                    state.colors.insert(name.parse().unwrap(), color);
                }
                Err(_) => {println!("Well it failed on the color parsing");return None}
            }
        }

        if state.colors.contains_key(str.trim()) {
            if state.ratio.is_empty() {
                state.flag.push(*state.colors.get(str.trim()).unwrap())
            } else {
                for _ in 0..state.ratio.pop().unwrap() {
                    state.flag.push(*state.colors.get(str.trim()).unwrap())
                }
            }
        }
    }

    return Option::Some(state)
}