# Notes about Rust


## Result

`.expect("Error message")` will get the Ok() from a result otherwise crash.

Use match:

```rust
match [Result<T, E>] {
    Ok(val) => {},
    Err(val) => [},
}
```
## Lifetime

A lifetime must be used, if the rust compiler doesn't know how long a pointer lives. 

Example:

```rust
enum Quality {
    Good,
    Questionable,
    Invalid,
}
// TODO: Implement as trait
struct HasQuality<'a> {
    pub value: &'a Box<&'a str>,
    quality: Quality,
}

impl<'a> HasQuality<'a> {
    pub fn new(value: &'a Box<&'a str>, quality: Quality) -> Self {
        Self {
            value,
            quality,
        }
    }
}
impl<'a> Drop for HasQuality<'a> {
    fn drop(&mut self) { }
}

#[test]
fn it_works() {
    let message = Box::new("Hello World!");
    let information = HasQuality::new(&message, Quality::Good);
    assert_eq!(*information.value, Box::new("Hello World!"));
    drop(message);
    assert_eq!(*information.value, Box::new("Hello World!"));
    drop(information);
    assert_eq!(*information.value, Box::new("Hello World!"));
}
```

The result of a check is:

```
error[E0505]: cannot move out of `message` because it is borrowed
  --> src/lib.rs:29:10
   |
27 |     let information = HasQuality::new(&message, Quality::Good);
   |                                       -------- borrow of `message` occurs here
28 |     assert_eq!(*information.value, Box::new("Hello World!"));
29 |     drop(message);
   |          ^^^^^^^ move out of `message` occurs here
30 |     assert_eq!(*information.value, Box::new("Hello World!"));
   |     --------------------------------------------------------- borrow later used here

```

So that example shows, that the lifetime specifier requires the input variable to live as long as the structure itself.
