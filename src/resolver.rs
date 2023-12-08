use crate::digest::Digest;
use crate::windows_fn::WindowsFn;

pub struct Resolver;

impl Resolver {
    pub fn resolve_fn<F>(&self, _module: &str, _name: &str) -> WindowsFn<F> {
        unimplemented!()
    }
}
