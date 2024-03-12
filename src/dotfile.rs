use clap::builder::Str;

pub fn read_dotfile(manager: String) {
    let manager = manager.replace("nix-env", "nix");
    let entries = std::fs::read_dir(".").unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        // println!("{}", format!("./dependencies-{manager}"));
        match path
            .to_str()
            .unwrap()
            .replace(
                &format!("./dependencies-{}", manager.clone()),
                "./dependencies",
            )
            .to_lowercase()
            .as_str()
        {
            // 디펜던시 확인후 설치
            "./dependencies" => {
                let dependencies = std::fs::read_to_string(format!("./dependencies-{manager}"));
                // 디펜전시 파일이 없으면 오류를 출력후 멈춤
                if dependencies.is_err() {
                    log::error!("\"dependencies-{}\" is not found", manager.clone());
                    return;
                }
                let dependencies = dependencies.unwrap();
                for dependency in dependencies.trim().split("\n") {
                    log::info!("installing: {dependency}");
                    match manager.as_str() {
                        "nix" => {
                            std::process::Command::new("nix-env")
                                .arg("-iv")
                                .arg(dependency)
                                .status()
                                .unwrap();
                        }
                        _ => {
                            std::process::Command::new(manager.clone())
                                .arg("install")
                                .arg(dependency)
                                .status()
                                .unwrap();
                        }
                    }
                }
            }
            x @ ("./.git" | "./.ds_store" | "license" | "reademe.md") => {
                log::info!("skipping: {}", x);
            }
            config_folder => {
                std::process::Command::new("mv")
                    .arg("-f")
                    .arg(config_folder)
                    .arg("~/.config/")
                    .status()
                    .unwrap();
            }
        }
    }
}
