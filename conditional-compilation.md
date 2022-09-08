Conditional Compilation in Rust
===============================

Into
----

1. Rust has a special attribute, `#[cfg]`, which allows you to compile code based on a configuration
   option (a predicate).

2. Configuration options can be set, e.g. `rustc --cfg foo`

3. If the configuration option `foo` is set then the code annotated by the attribute `#[cfg(foo)]`
   will be compiled in, if `foo` is not set then the code will not be compiled in.

```
#[cfg(foo)]
{
    // Code in this block is compiled in if, and only if, `foo` is set.
}
```

Basics of configuration options
-------------------------------

Configuration options are names or key=value pairs.

```
#[cfg(foo)]
```


```
#[cfg(bar = "baz")]
```

They can be combined using three helpers `any`, `all`, `not`

```
#[cfg(any(foo, bar))]

#[cfg(all(foo, bar))]

#[cfg(not(foo))]
```

These can nest arbitrarily

### Setting conditional configuration options

Conditional config options can be set in three ways (that I know of)

- Directly (as shown above): `rustc --cfg foo`
- Using cargo's `--features` option
- Using another attribute `#[cfg_attr]`

```
cfg_attr(a, b)
```

Conditionally set configuration option `b` if `a` is set

### Branching

To branch on a conditional configuration option use `cfg!`

```rust
    if cfg!(foo) {
    	println!("foo is set");
    }
```


Features
--------

The term "feature" is overloaded

1. feature attribute: `#![feature(test)]` (discussed below)
2. cargo features - discussed now

- Cargo uses `cfg` and key=value pairs to support features.

- features get translated by cargo into an argument to rustc using the key=value form

`cargo --feature=foo ...`  =>  `rustc --cfg 'feature="foo"' ...`

- There is nothing special about the identifier `feature`, it is just a cargo convention.

- keys are not unique, multiple features get translated as multiple `--cfg` options

- By convention features should be additive, this means one should be able to enable any combination
 of features including _all_ features (`cargo check --all-features`).


Benchmark pattern
-----------------

Its nice if features work the same for all toolchains, currently online there is a pattern put
forward to guard benchmarks with an "unstable" feature

```
#[cfg_attr(all(test, feature = "unstable"), feature(test))]
#[cfg(all(test, feature = "unstable"))] extern crate test;
```

However this breaks the build for the stable toolchain because `feature(test)` cannot be used with a
stable toolchain. An alternative is to manually pass the compiler conditional option to rustc
instead of relying on cargo.

```
    #[cfg(bench)]
    mod bench {
        // conditionally compiled bench code.
    }
```

And compile the bench code with
```
	RUSTFLAGS='--cfg=bench' cargo check
```

We can now happily build with all features
```
cargo +stable check --all-features
```


References
----------

- The Rust Book:
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/conditional-compilation.html

- The Rust Reference:
https://doc.rust-lang.org/reference/conditional-compilation.html

