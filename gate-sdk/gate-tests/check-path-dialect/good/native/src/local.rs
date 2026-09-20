// A module outside the speller that reaches each primitive through a named walk helper. Nothing
// here spells one, so the locality arm sees no occurrence at all — which is the shape every
// routed site takes after the sweep.
use crate::walk;

pub fn inside(root: &str, p: &str) -> bool {
    walk::under(root, p)
}

pub fn rooted(p: &str) -> bool {
    walk::path_root(p)
}
