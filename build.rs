extern crate gcc;

fn main() {
    gcc::Config::new().file("src/unqlite.c")
        .if_enable_threads()
        .if_jx9_diable_builtin_func()
        .if_jx9_enable_math_func()
        .if_jx9_disable_disk_io()
        .if_enable_jx9_hash_io()
        .compile("libunqlite.a");
}
trait ConfigExt {
    fn if_enable_threads(&mut self) -> &mut Self {
        self
    }
    fn if_jx9_diable_builtin_func(&mut self) -> &mut Self {
        self
    }
    fn if_jx9_enable_math_func(&mut self) -> &mut Self {
        self
    }
    fn if_jx9_disable_disk_io(&mut self) -> &mut Self {
        self
    }
    fn if_enable_jx9_hash_io(&mut self) -> &mut Self {
        self
    }
}

impl ConfigExt for gcc::Config {
    #[cfg(feature = "enable-threads")]
    fn if_enable_threads(&mut self) -> &mut Self {
        self.define("UNQLITE_ENABLE_THREADS", None).flag("-lpthread")
    }
    #[cfg(feature = "jx9-disable-builtin-func")]
    fn if_jx9_diable_builtin_func(&mut self) -> &mut Self {
        self.define("JX9_DISABLE_BUILTIN_FUNC", None)
    }
    #[cfg(feature = "jx9-enable-math-func")]
    fn if_jx9_enable_math_func(&mut self) -> &mut Self {
        self.define("JX9_ENABLE_MATH_FUNC", None)
    }
    #[cfg(feature = "jx9-disable-disk-io")]
    fn if_jx9_disable_disk_io(&mut self) -> &mut Self {
        self.define("JX9_DISABLE_DISK_IO", None)
    }
    #[cfg(feature = "enable-jx9-hash-io")]
    fn if_enable_jx9_hash_io(&mut self) -> &mut Self {
        self.define("UNQLITE_ENABLE_JX9_HASH_IO", None)
    }
}
