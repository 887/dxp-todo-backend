use tracing::info;

pub(crate) fn get_current_working_dir() -> String {
    let res = std::env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}

pub(crate) fn get_lib_path() -> String {
    let res = std::env::var("LD_LIBRARY_PATH");
    match res {
        Ok(path) => path.to_string(),
        Err(_) => "FAILED".to_string(),
    }
}

pub(crate) fn print_paths() {
    info!("working dir: {}", get_current_working_dir());
    info!("lib path: {}", get_lib_path());
}
