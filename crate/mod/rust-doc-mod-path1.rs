// src/a/b.rs	src/a/b/thread/local_data.rs	crate::a::b::thread_files::local_data

#[path = "thread_files"]
mod thread {
    // Load the `local_data` module from `src/a/b/thread_files/tls.rs` relative to
    // this source file's directory.
    #[path = "tls.rs"]
    mod local_data;
}