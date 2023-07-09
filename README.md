# libsixel-rs

This is a port of [libsixel](https://github.com/libsixel/libsixel) from C to Rust.

It is currently a work-in-progress, and does not work.

Initial Rust files were created using the [c2rust](https://github.com/immunant/c2rust) project. This project is partially an experiment in using `c2rust` as a porting tool.

## Following along

If, for whatever reason, you are following the development of this library, (partially) completed modules are included in `src/lib.rs`.

All other code is from the first `c2rust` pass over the original C project. As porting is completed, modules will be added to `src/lib.rs`.

In a number of cases, looking at the original C code is easier than figuring out the `c2rust` code. Partly, because `c2rust` makes a best-attempt at translating C-isms like `goto` in a somewhat confusing way.

`main` will also be clobbered until the entire library is ported. After that, I will add some additional tests, a fuzzing suite, fix bugs discovered by tests, and make a first beta release.

After the first beta release, `main` will be considered stable, and normal developement will happen through opening and merging PRs.
