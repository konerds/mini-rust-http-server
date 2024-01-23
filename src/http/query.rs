use std::collections::HashMap;

#[derive(Debug)]
pub struct Query<'buf> {
    data: HashMap<&'buf str, TypeValueQuery<'buf>>,
}

#[derive(Debug)]
pub enum TypeValueQuery<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> Query<'buf> {
    pub fn get(&self, key: &str) -> Option<&TypeValueQuery> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for Query<'buf> {
    fn from(qs: &'buf str) -> Self {
        let mut data = HashMap::new();
        for el_pair in qs.split('&') {
            let mut key = el_pair;
            let mut value = "";
            if let Some(i) = el_pair.find('=') {
                key = &el_pair[..i];
                value = &el_pair[i + 1..];
            }
            data.entry(key)
                .and_modify(|existing: &mut TypeValueQuery| match existing {
                    TypeValueQuery::Single(value_prev) => {
                        let mut vec = vec![value_prev, value];
                        *existing = TypeValueQuery::Multiple(vec![value_prev, value]);
                    }
                    TypeValueQuery::Multiple(vec) => vec.push(value),
                })
                .or_insert(TypeValueQuery::Single(value));
        }
        Query { data }
    }
}
