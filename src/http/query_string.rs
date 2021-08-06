use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
  data: HashMap<&'buf str, Value<'buf>>,
}
#[derive(Debug)]
pub enum Value<'buf> {
  Single(&'buf str),
  Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
  pub fn get(&self, key: &str) -> Option<&Value> {
    self.data.get(key)
  }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
  fn from(s: &'buf str) -> Self {
    let mut data = HashMap::new();

    for sub_str in s.split("&") {

      if let Some(i) = sub_str.find("=") {
        let key = &sub_str[..i];
        let val = &sub_str[i + 1..];

        data
          .entry(key)
          .and_modify(|existing| match existing {
            Value::Single(prev_value) => *existing = Value::Multiple(vec![prev_value, val]),
            Value::Multiple(vec) => vec.push(val),
          })
          .or_insert(Value::Single(val));
      }
    }
    QueryString { data }
  }
}
