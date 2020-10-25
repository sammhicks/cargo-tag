use std::borrow::Cow;
use std::process::Command;

fn main() {
    let manifest_location = std::env::args()
        .find(|arg| arg.strip_suffix("Cargo.toml").is_some())
        .map_or(Cow::Borrowed("Cargo.toml"), Cow::Owned);

    let output = Command::new("git")
        .arg("show")
        .arg(manifest_location.as_ref())
        .output()
        .unwrap();

    if !output.status.success() {
        panic!("Git show failed");
    }

    let stdout = String::from_utf8(output.stdout).unwrap();
    let new_version = stdout
        .lines()
        .skip_while(|line| !line.starts_with("@@"))
        .skip(1)
        .filter_map(|line| line.strip_prefix("+version = \"")?.strip_suffix("\""))
        .next();

    if let Some(new_version) = new_version {
        let tag = format!("v{}", new_version);
        println!("Tagging with {:?}", tag);
        if !Command::new("git")
            .arg("tag")
            .arg(tag)
            .output()
            .unwrap()
            .status
            .success()
        {
            panic!("Git tag failed");
        };
    }
}
