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
