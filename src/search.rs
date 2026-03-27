use strsim::{jaro_winkler, normalized_levenshtein};

pub struct Searcher<'a, T>
where
    T: Searchable,
{
    vec: &'a Vec<T>,
}

impl<'a, T> Searcher<'a, T>
where
    T: Searchable,
{
    pub fn new(vec: &'a Vec<T>) -> Self {
        Self { vec }
    }

    pub fn search(&self, request: &str) -> impl Iterator<Item = (u32, f64)> {
        self.vec
            .iter()
            .enumerate()
            .map(|(i, item)| (i as u32, item.score(request)))
            .filter(|it| it.1 > 0.3)
    }
}

pub trait Searchable {
    fn score(&self, request: &str) -> f64;
}

impl Searchable for str {
    fn score(&self, request: &str) -> f64 {
        if request.len() <= 3 {
            jaro_winkler(self, request)
        } else {
            normalized_levenshtein(self, request)
                + self.contains(request).then(|| 0.5).unwrap_or(0.0)
        }
    }
}
