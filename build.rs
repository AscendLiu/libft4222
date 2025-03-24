use std::{env, path::PathBuf};

fn search_path_ftd2xx() -> PathBuf {
    let mut path: PathBuf = PathBuf::from("vendor");
    path.push("libftd2xx");

    match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86_64" => {
            match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
                "windows" => path.push("windows"),
                "linux" => path.push("linux"),
                target_os => panic!("Target OS not supported: {target_os}")
            }
        },
        target_arch => panic!("Target architecture not supported: {target_arch}"),
    }

    path
}

fn search_path_ft4222() -> PathBuf {
    let mut path: PathBuf = PathBuf::from("vendor");
    path.push("libft4222");

    match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86_64" => {
            match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
                "windows" => path.push("windows"),
                "linux" => path.push("linux"),
                target_os => panic!("Target OS not supported: {target_os}")
            }
        },
        target_arch => panic!("Target architecture not supported: {target_arch}"),
    }

    path
}


fn main() {
    let cwd: PathBuf = env::current_dir().unwrap();
    let mut ftd2xx_path: PathBuf = cwd.clone();
    ftd2xx_path.push(search_path_ftd2xx());

    let mut ft4222_path: PathBuf = cwd;
    ft4222_path.push(search_path_ft4222());


    match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => {
            println!(
                "cargo:rustc-link-search=native={}",
                ftd2xx_path.to_str().unwrap()
            );
            println!("cargo:rustc-link-lib=static=ftd2xx");
        }
        _ => (),
    }

    println!(
        "cargo:rustc-link-search=native={}",
        ft4222_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=ft4222");

}
