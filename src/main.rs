/// Creates a wrapper executable for an action. Solves some nasty problem with
/// calling non-executables on Windows from nushell terminal (when non-exe)
/// files require extension in order to be called, which is not convenient).

use std::{env, fs, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "please provide only target cmd argument");
    let cmd = &args[1];

    let deploy = fs::read_to_string(path)

    let output = Command::new("cargo")
        .arg("--help")
        .output()
        .expect("fail to run cmd");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}
