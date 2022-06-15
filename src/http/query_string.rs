use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buffer> {
    data: HashMap<&'buffer str, Value<'buffer>>,
}

#[derive(Debug)]
pub enum Value<'buffer> {
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>),
}

impl<'buffer> QueryString<'buffer> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buffer> From<&'buffer str> for QueryString<'buffer> {
    fn from(query_string: &'buffer str) -> Self {
        let mut data = HashMap::new();
        for pair in query_string.split('&') {
            let mut key = pair;
            let mut value: &'buffer str = &pair[..0];

            if let Some(i) = pair.find('=') {
                key = &pair[..i];
                value = &pair[i + 1..];
            }
            data.entry(key)
                .and_modify(|curr: &mut Value| match curr {
                    Value::Single(prev) => *curr = Value::Multiple(vec![prev, value]),
                    Value::Multiple(prev) => prev.push(value)
                })
                .or_insert(Value::Single(value));
        }

        Self { data }
    }
}
