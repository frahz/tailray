use std::path::PathBuf;
use which::which;
use whoami::username;

use crate::tailscale::utils::check_tailscale_operator;

pub fn get_path() -> PathBuf {
    which("pkexec").unwrap_or_else(|_| panic!("pkexec not found in PATH"))
}

// We don't need to elevate privileges if we're using the Tray service
// as the root user. This shouldn't really happen, but it's possible
// depending on how Tailray is ran.
pub fn should_elevate_perms() -> bool {
    let parent_user = username();

    if check_tailscale_operator(&parent_user) {
        return false;
    }

    if parent_user.eq("root") {
        return false;
    }

    true
}
