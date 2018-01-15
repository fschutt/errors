//! This bug took me, in total, a month to find.
//! The fixing commit is here:
//! https://github.com/fschutt/proj5/commit/c1d7ec5843faa1fd70f09558949d07c170d76e90

//! This bug was due to an intermediate value.
//! I passed in x, copied it into a local variable,
//! but forgot to rewrite parts of the code to use the
//! new local variable

//! At some point I knew I had a bug, but didn't know where
//! So I left the project unfinished and worked on something else.
//! The bug was in a shared library between project A and project B.

//! After I fixed the bug, I updated the library for project B 
//! and it worked perfectly again.
//! The whole error was a bit different (the example would 
//! have been caught by an "unused variable" warning).

fn something(x: &mut f32, y: &mut f32) {
    let new_x = *x / 100_000.0_f32;
    let new_y = *y / 100_000.0_f32;

    // error: should use new_x, new_y !!!
    let (new_x, new_y) = do_transform(new_x, y);
    
    *x = new_x;
    *y = new_y;
}
