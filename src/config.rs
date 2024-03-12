pub fn config(config_folder: String, flag: Option<String>) {
    let home = std::env::var("HOME").unwrap();
    // let home = std::env::var("pwd").unwrap();
    if let Ok(_) = std::process::Command::new("cp")
        .arg(if flag.is_some() {
            format!("-{}", flag.clone().unwrap())
        } else {
            "-rf".to_string()
        })
        .arg(format!("{}", config_folder))
        .arg(format!("{home}/.config/"))
        .status()
    {
        log::info!(target: format!("{config_folder}")[2..].trim(), "success")
    } else {
        log::error!(target: format!("{config_folder}")[2..].trim(), "error")
    }
}
