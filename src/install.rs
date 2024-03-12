pub fn install(dependencies: String, manager: String, flag: Option<String>) {
    for dependency in dependencies.trim().split("\n") {
        log::info!("installing: {dependency}");
        match manager.as_str() {
            "nix" => {
                std::process::Command::new("nix-env")
                    .arg(if flag.is_some() {
                        format!("-{}", flag.clone().unwrap())
                    } else {
                        "-iv".to_string()
                    })
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
