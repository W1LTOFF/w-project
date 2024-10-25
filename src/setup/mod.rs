use crate::config;
use std::fs;
use std::io::Write;

pub fn make_dir(path: &str) {
    let silent = false;
    if !silent {
        // println!()
    }

    match fs::create_dir(path) {
        Ok(_) => {}
        Err(_) => {
            println!("Directory already exists")
        }
    }
}

pub fn copy_dir(path: &str, copy_to: &str, conf: &config::Config) {
    make_dir(copy_to);
    // println!("{}", path);
    let dir_items = fs::read_dir(path).unwrap();

    for item in dir_items {
        if item.as_ref().unwrap().file_type().unwrap().is_file() {
            let file =
                fs::read_to_string(item.as_ref().unwrap().path().display().to_string()).unwrap();
            let file = file.replace(
                "PROJECT_NAME",
                // &conf
                //     .conf_pair()
                //     .get("rust")
                //     .unwrap()
                //     .get("project_name")
                //     .unwrap()
                //     .value_s()
                //     .unwrap(),
                &conf.arg_pair().get("project_name").unwrap(),
            );

            // println!("{:#?}\n", file);
            if fs::exists(format!(
                "{}/{}",
                copy_to,
                item.as_ref().unwrap().file_name().into_string().unwrap()
            ))
            .unwrap()
            {
                fs::write(item.as_ref().unwrap().path().display().to_string(), file).unwrap();
            } else {
                let mut new_file = fs::File::create(format!(
                    "{}/{}",
                    copy_to,
                    item.as_ref().unwrap().file_name().into_string().unwrap()
                ))
                .unwrap();
                new_file.write(file.as_bytes()).unwrap();
            }
            // fs::copy(
            //     item.as_ref().unwrap().path().display().to_string(),
            //     format!(
            //         "{}/{}",
            //         copy_to,
            //         item.as_ref().unwrap().file_name().into_string().unwrap()
            //     ),
            // )
            // .unwrap();
        } else if item.as_ref().unwrap().file_type().unwrap().is_dir() {
            copy_dir(
                &item.as_ref().unwrap().path().display().to_string(),
                &format!(
                    "{}/{}",
                    copy_to,
                    &item.as_ref().unwrap().file_name().into_string().unwrap()
                ),
                &conf,
            );
        }

        // println!(
        //     "Name: {}\t|Is dir: {}",
        //     item.as_ref().unwrap().file_name().into_string().unwrap(),
        //     item.as_ref().unwrap().file_type().unwrap().is_dir()
        // )
    }
}
