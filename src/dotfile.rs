use crate::{config, install};
#[derive(PartialEq, Eq)]
pub enum DotfileConfig {
    JustInstall,
    JustConfig,
    None,
}

pub fn read_dotfile(manager: String, flag: Option<String>, type_: DotfileConfig) {
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
                if type_ == DotfileConfig::JustConfig {
                    continue;
                }
                let dependencies = std::fs::read_to_string(format!("./dependencies-{manager}"));
                // 디펜전시 파일이 없으면 오류를 출력후 멈춤
                if dependencies.is_err() {
                    log::error!("\"dependencies-{}\" is not found", manager.clone());
                    return;
                }
                let dependencies = dependencies.unwrap();
                install::install(dependencies, manager.clone(), flag.clone());
            }
            x @ ("./.git" | "./.ds_store" | "license" | "readme.md") => {
                log::info!(target: &x[2..], "skipping");
            }
            config_folder => {
                if type_ == DotfileConfig::JustInstall {
                    continue;
                }
                if config_folder.ends_with(".unused") {
                    log::info!(target: &config_folder[2..], "skipping");
                }
                config::config(config_folder.to_string(), flag.clone());
            }
        }
    }
}
