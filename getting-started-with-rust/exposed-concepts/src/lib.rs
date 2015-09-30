#[derive(Debug)]
pub struct DependsOnOthers<'a> {
    body: &'a str,
}

pub fn upper<'a>(s : &str) -> DependsOnOthers {
    DependsOnOthers {
        body: s
    }
}
