pub mod things;

pub struct Alpha;

pub enum Beta {
    A,
    B,
}

pub trait Gamma {
    fn g(&self);
}

pub fn delta() {}

pub(crate) fn theta() {}

pub const EPSILON: u32 = 3;

pub static ZETA: u32 = 4;

pub type Eta = u32;

fn private_not_shown() {}
