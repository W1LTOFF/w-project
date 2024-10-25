use std::{collections::HashMap, fs::File, io::Read};

pub struct Config {
    path: String,
    config_raw_s: String,
    // {{rust: {value1: "test", value2: "42"}}, {cpp: {...}, ...}}
    conf_pair: HashMap<String, HashMap<String, ConfigValue>>,
    arg_pair: HashMap<String, String>,
}
#[derive(Clone, Debug)]
pub struct ConfigValue {
    raw_value: String,
}

impl Config {
    pub fn new(conf_path: &str) -> Self {
        Self {
            path: conf_path.to_owned(),
            config_raw_s: String::new(),
            conf_pair: HashMap::new(),
            arg_pair: HashMap::new(),
        }
    }

    pub fn read(&mut self) {
        match File::open(&self.path) {
            Ok(mut file) => {
                file.read_to_string(&mut self.config_raw_s).unwrap();
            }
            Err(e) => {
                println!("Config file can't be opened\nError: \"{}\"", e);
            }
        }
    }

    pub fn handle_args(&mut self, args: std::env::Args) {
        // args
        // $project_name=rust_test
        // $language=rust

        for arg in args {
            // println!("{}", arg);
            if arg.starts_with("$") {
                self.arg_pair.insert(
                    arg.get(1..arg.find("=").unwrap()).unwrap().to_string(),
                    arg.get(arg.find("=").unwrap() + 1..).unwrap().to_string(),
                );
                // println!("{:#?}", self.arg_pair());
            }

            // let arg_key_instert;
            // let arg_value_insert;
            // if arg_state_temp {
            //     arg_value_insert = arg.clone();
            // } else {
            //     arg_value_insert = "".to_string()
            // }
            // if arg.starts_with("$") {
            //     arg_key_instert = arg.get(1..).unwrap().to_string();
            //     if arg_state_temp {
            //         self.arg_pair.insert(arg_key_instert, arg_value_insert);
            //     }
            //     arg_state_temp = true;
            // }
        }
    }

    pub fn handle(&mut self) {
        self.read();
        let mut state = false;
        // temp variables
        let mut pair_temp: HashMap<String, ConfigValue> = HashMap::new();
        let mut name_temp = "";

        let lines: Vec<&str> = self.config_raw_s.split('\r').collect();
        for line in &lines {
            let line = line.trim();
            // print!("{:#?}\n", line);
            if !line.starts_with("#") && !line.is_empty() {
                if line.starts_with("]") {
                    self.conf_pair
                        .insert(name_temp.to_owned(), pair_temp.clone());
                    state = false;
                    // clearing temp variables
                    name_temp = "";
                    pair_temp.clear();
                }
                if state {
                    pair_temp.insert(
                        line.get(1..line.find(" = ").unwrap()).unwrap().to_owned(),
                        ConfigValue {
                            raw_value: line
                                .get(line.find(" = ").unwrap() + 3..)
                                .unwrap()
                                .to_owned(),
                        },
                    );
                }
                if line.starts_with("[") {
                    name_temp = line.get(1..).unwrap();
                    state = true;
                }
            }
        }
        // println!("{:#?}", self.conf_pair)
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn conf_pair(&self) -> &HashMap<String, HashMap<String, ConfigValue>> {
        &self.conf_pair
    }
    pub fn arg_pair(&self) -> &HashMap<String, String> {
        &self.arg_pair
    }
}

impl ConfigValue {
    pub fn value_s(&self) -> Result<String, ()> {
        let value_type = self.raw_value.get(..1).unwrap();
        if value_type == "s" {
            Ok(self
                .raw_value
                .get(2..self.raw_value.len() - 1)
                .unwrap()
                .to_owned())
        } else {
            Err(())
        }
    }
    pub fn value_i(&self) -> Result<i64, ()> {
        let value_type = self.raw_value.get(..1).unwrap();
        if value_type == "i" {
            Ok(self
                .raw_value
                .get(1..self.raw_value.len())
                .unwrap()
                .parse()
                .unwrap())
        } else {
            Err(())
        }
    }
}
