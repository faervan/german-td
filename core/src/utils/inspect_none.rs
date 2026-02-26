pub trait InspectNoneExt {
    fn inspect_none<F>(self, f: F) -> Self
    where
        F: FnOnce();
}

impl<T> InspectNoneExt for Option<T> {
    fn inspect_none<F>(self, f: F) -> Self
    where
        F: FnOnce(),
    {
        if self.is_none() {
            f();
        }
        self
    }
}
