pub struct App<'a> {
    pub title: &'a str,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App { title }
    }
}
