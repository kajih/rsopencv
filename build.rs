fn main() {
    build_for_os();
}

#[cfg(target_os = "windows")]
fn build_for_os() {
    // set VCPKGRS_DYNAMIC=1
    // for dynamic link libraries (on windows)
    let mut vccfg = vcpkg::Config::new();
    vccfg.find_package("opencv4").expect("lib not found");

    /*
       vcpkg::Config::new()
       .emit_includes(true)
       .find_package("opencv4")
       .expect("lib not found");
    */
}

#[cfg(not(target_os = "windows"))]
fn build_for_os() {
    println!("Not windows");
}
