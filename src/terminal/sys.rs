#[cfg(not(target_os = "redox"))]
mod other;
#[cfg(target_os = "redox")]
mod redox;
