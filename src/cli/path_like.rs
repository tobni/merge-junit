use std::path::Path;

pub trait PathLike {
    fn exists(&self) -> bool;
    fn as_path(&self) -> &Path;
    fn name(&self) -> &str;
}

impl PathLike for String {
    fn exists(&self) -> bool {
        <String as AsRef<Path>>::as_ref(self).exists()
    }

    fn as_path(&self) -> &Path {
        <String as AsRef<Path>>::as_ref(self)
    }

    fn name(&self) -> &str {
        self
    }
}
