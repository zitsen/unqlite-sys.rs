extern crate gcc;

fn main() {
    let mut config = gcc::Config::new();
    config.file("src/unqlite.c");
    if cfg!(features = "enable-threads") {
        config.define("UNQLITE_ENABLE_THREADS", None)
            .flag("-lpthread");
    }
    if cfg!(feature = "jx9-disable-builtin-func") {
        config.define("JX9_DISABLE_BUILTIN_FUNC", None);
    }
    if cfg!(feature = "jx9-enable-math-fuc") {
        config.define("JX9_ENABLE_MATH_FUNC", None);
    }
    if cfg!(feature = "jx9-disable-disk-io") {
        config.define("JX9_DISABLE_DISK_IO", None);
    }
    if cfg!(feature = "enable-jx9-hash-io") {
        config.define("UNQLITE_ENABLE_JX9_HASH_IO", None);
    }
    config.compile("libunqlite.a");
}
