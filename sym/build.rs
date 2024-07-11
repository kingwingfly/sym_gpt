use std::path::PathBuf;

fn main() {
    let src_dir = PathBuf::from("../dist").canonicalize().unwrap();
    let dest_dir = PathBuf::from("../target/debug/dist");
    #[cfg(not(target_os = "windows"))]
    std::os::unix::fs::symlink(src_dir, dest_dir).ok();
    #[cfg(target_os = "windows")]
    std::os::windows::fs::symlink_dir(src_dir, dest_dir).ok();
}
