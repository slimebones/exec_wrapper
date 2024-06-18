/// Creates a wrapper executable for an action. Solves some nasty problem with
/// calling non-executables on Windows from nushell terminal (when non-exe)
/// files require extension in order to be called, which is not convenient).
///
/// Note that all commands are executed using "sh". For now it's hardcoded.
use std::{
    env::{self, current_dir, temp_dir},
    fs,
    process::Command,
};

static DEPLOY_STR: &str = r#"use std::process::Command;
static CMD: &str = "REPLACE_TARGET";

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg(CMD)
        .output()
        .expect(format!("failed to run cmd {CMD}").as_str());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        println!("status: {}", output.status);
    }
    if !stdout.trim().is_empty() {
        println!("{}", stdout);
    }
    if !stderr.trim().is_empty() {
        println!("{}", stderr);
    }

    assert!(output.status.success());
}
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 3,
        "please provide target cmd argument and output filename"
    );
    let cmd = &args[1];
    let out_filename = &args[2];

    let replaced_deploy = str::replace(&DEPLOY_STR, "REPLACE_TARGET", cmd);
    let temp_dir = temp_dir();
    fs::write(temp_dir.join("exec_wrapper_deploy.rs"), replaced_deploy)
        .expect("unable to write file");

    let output = Command::new("rustc")
        .current_dir(&temp_dir)
        .arg("exec_wrapper_deploy.rs")
        .output()
        .expect("fail to compile deploy");
    assert!(output.status.success());

    let current_dir = current_dir().unwrap();
    fs::copy(
        temp_dir.join("exec_wrapper_deploy.exe"),
        current_dir.join(&out_filename),
    )
    .unwrap();
}
