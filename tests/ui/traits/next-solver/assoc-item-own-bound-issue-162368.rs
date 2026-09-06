//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

trait Trait {
    type Assoc
    where
        Self: Sized;
}

impl<T: ?Sized> Trait for T {
    type Assoc = i32;
    //~^ ERROR the size for values of type `T` cannot be known at compilation time
}

fn main() {}
