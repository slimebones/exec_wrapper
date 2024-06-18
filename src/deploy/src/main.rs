use std::process::Command;

fn main() {
    const CMD: &str = "REPLACEMENT_TARGET";
    let output = Command::new(CMD)
        .arg("--help")
        .output()
        .expect(format!("failed to run cmd {CMD}").as_str());
    println!("status: {}", output.status);
    if output.stdout.len() > 0 {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    if output.stderr.len() > 0 {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    assert!(output.status.success());
}
