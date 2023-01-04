use arboard::{Clipboard, SetExtLinux};
use std::{env, error::Error, process};

/// Extra long and special naming so it will not be passed by the as argument by accident.
/// This keyword instructs the app to start a new process for keeping the set clipboard alive
/// on linux.
const DEAMON_KEYWORD: &str = "__INTERNAL_DAEMON_FAV_FOLDER";

pub fn put_into_clipboard(content: &str) -> Result<(), Box<dyn Error>> {
    if is_on_linux() {
        spawn_daemon_for_clipboard(content)?;
    } else {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(content)?;
    }
    Ok(())
}
pub fn execute_as_possible_daemon_clipboard() -> Result<(), Box<dyn Error>> {
    if !is_on_linux() {
        return Ok(());
    }
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && args[1] == DEAMON_KEYWORD {
        let clipboard = &args[2];
        set_and_wait_for_clipboard_as_daemon(clipboard)?;
        std::process::exit(0);
    }

    Ok(())
}

fn is_on_linux() -> bool {
    cfg!(target_os = "linux")
}

fn set_and_wait_for_clipboard_as_daemon(content: &str) -> Result<(), Box<dyn Error>> {
    let mut clip = Clipboard::new()?;
    clip.set().wait().text(content)?;

    Ok(())
}

fn spawn_daemon_for_clipboard(content: &str) -> Result<(), Box<dyn Error>> {
    let exe_path = env::current_exe()?;
    process::Command::new(exe_path)
        .arg(DEAMON_KEYWORD)
        .arg(content)
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .current_dir("/")
        .spawn()?;

    Ok(())
}
