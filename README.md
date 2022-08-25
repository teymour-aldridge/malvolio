# Malvolio

Malvolio is a library with a "builder-syntax" for creating complex HTML
documents.

Unlike "JSX" style solutions, Malvolio avoids climbing out what has been
described as the "macro escape hatch"
[[1]](https://twitter.com/graydon_pub/status/1294692200916246528). This means that
you get good editor support, automatic code formatting, and easier refactoring.

## Usage

Malvolio should be relatively simple to use.

A quick couple of examples (see the examples and documentation section lower
down for more details):

```rust
malvolio::prelude::Form::new()
    .attribute(Method::Post)
    .child(
        Input::default()
            .attribute(Type::Text)
            .attribute(Name::new("invited-user-identifier")),
    )
    .child(
        Input::default()
            .attribute(Type::Submit)
            .attribute(Value::new("Invite teacher!")),
    )
```

## Documentation

Malvolio has API docs which are [hosted on docs.rs](https://docs.rs/malvolio).

## License

Malvolio is licensed under the BSD 3-Clause license - a copy of which may be
found in the `LICENSE` file in the repository root, currently at
<https://github.com/puck-rs/malvolio>.

At your option, the program is also licensed under the terms of the Apache 2.0
software license - details as to where the license can be located can be found
in the aforementioned repository root, in the file `APACHE-2.0`.

## Semver

Malvolio follows semantic versioning.

For pre-1.0.0 versions we make the following guarantees. If the previous release
has version `0.x.y`, then the next release will have version
- `0.x+1.0` if and only if there was a
  [breaking change](https://doc.rust-lang.org/cargo/reference/semver.html#api-compatibility)
- `0.x.y+1` if and only if the changes made were not breaking.
