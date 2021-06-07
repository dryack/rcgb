#[cfg(target_os = "unix")]
extern crate libc;

#[cfg(target_os = "windows")]
use atty::Stream;


pub(crate) fn istty() -> bool {
    #[cfg(target_os = "unix")]
    let istty = unsafe { libc::isatty(libc::STDIN_FILENO as i32) } != 0;

    #[cfg(target_os = "windows")]
    let istty = atty::is(Stream::Stdin);

    return if istty {
        true
    } else {
        false
    }
}