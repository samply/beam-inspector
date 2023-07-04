use std::process::Command;

fn main() {
    if option_env!("CROSS_RUNNER").is_none() {
        std::env::set_current_dir("./frontend").expect("frontend has been deleted");
        Command::new("npm")
            .arg("install")
            .spawn()
            .expect("npm needs to be installed to compile monitoring feature")
            .wait()
            .expect("Failed to run npm install command");
        Command::new("npm")
            .args([ "run", "build" ])
            .spawn()
            .expect("npm needs to be installed to compile monitoring feature")
            .wait()
            .expect("Failed to run npm run build command");
    }
}
