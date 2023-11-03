use std::borrow::Cow;
use std::path::Path;
use std::process::Command;

#[derive(Debug, serde::Deserialize)]
struct Package<'a> {
    name: Cow<'a, str>,
    id: Cow<'a, str>,
    manifest_path: Cow<'a, Path>,
}

#[derive(Debug, serde::Deserialize)]
struct Metadata<'a> {
    #[serde(borrow)]
    packages: Vec<Package<'a>>,
    #[serde(borrow)]
    workspace_members: Vec<Cow<'a, str>>,
}

fn main() {
    let cargo_metadata = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .output()
        .unwrap();

    if !cargo_metadata.status.success() {
        panic!("cargo metadata failed");
    }

    let metadata: Metadata = serde_json::from_slice(cargo_metadata.stdout.as_ref()).unwrap();

    let workspace_packages = metadata
        .packages
        .iter()
        .filter(|package| metadata.workspace_members.contains(&package.id))
        .collect::<Vec<_>>();

    for workspace_package in workspace_packages.iter() {
        let git_show = Command::new("git")
            .arg("show")
            .arg(workspace_package.manifest_path.as_ref())
            .output()
            .unwrap();

        if !git_show.status.success() {
            panic!("Git show failed");
        }

        let stdout = String::from_utf8(git_show.stdout).unwrap();
        let new_version = stdout
            .lines()
            .skip_while(|line| !line.starts_with("@@"))
            .skip(1)
            .filter_map(|line| line.strip_prefix("+version = \"")?.strip_suffix('\"'))
            .next();

        if let Some(new_version) = new_version {
            let tag = if workspace_packages.len() == 1 {
                format!("v{}", new_version)
            } else {
                format!("{}-v{}", workspace_package.name, new_version)
            };
            println!("Tagging {} with {:?}", workspace_package.name, tag);
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
}
