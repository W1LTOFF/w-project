use std::env::args;

use config::Config;

mod config;
mod setup;

fn main() {
    let mut conf = Config::new("./src/wilt_projects.conf");
    conf.handle_args(args());
    conf.handle();
    // args()
    // print!("{:#?}", conf.conf_pair());
    setup::copy_dir(
        &format!(
            "{}/{}",
            &conf
                .conf_pair()
                .get("config")
                .unwrap()
                .get("template_folder_path")
                .unwrap()
                .value_s()
                .unwrap(),
            conf.arg_pair().get("language").unwrap()
        ),
        &format!(
            "{}/{}",
            conf.conf_pair()
                .get("config")
                .unwrap()
                .get("folder_out_path")
                .unwrap()
                .value_s()
                .unwrap(),
            // conf.conf_pair()
            //     .get("rust")
            //     .unwrap()
            //     .get("project_name")
            //     .unwrap()
            //     .value_s()
            //     .unwrap(),
            &conf.arg_pair().get("project_name").unwrap(),
        ),
        &conf,
    );

    // args().last();
    // // setup::copy_dir(path, copy_to);
    // println!(
    //     "{}",
    //     conf.conf_pair()
    //         .get("rust")
    //         .unwrap()
    //         .get("value")
    //         .unwrap()
    //         .value_i()
    //         .unwrap()
    // )
}
