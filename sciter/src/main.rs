use std::path::*;

fn find_resource(path:&str) -> Result<PathBuf,failure::Error> {
    let base_dir = std::env::current_dir()?;
    if base_dir.join(path).exists() {
        return Ok(base_dir.join(path));
    }
    let mut exe_path = std::env::current_exe()?;
    exe_path.pop();

    let base_dir = std::env::current_dir()?;
    if base_dir.join(path).exists() {
        return Ok(base_dir.join(path));
    }
    return Err(failure::err_msg("could not fould valid path"));
}

fn main() {
    println!("{:?} {:?}",std::env::current_dir(),std::env::current_exe());
    let mut frame = sciter::Window::new();
    let html_path = find_resource("./resource/index.htm");
    let html_path = html_path.unwrap().to_string_lossy().to_string();
    frame.load_file(&html_path);
    frame.run_app();
}