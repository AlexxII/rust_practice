use serde::Deserialize;
use serde::de::{self, Visitor};

struct Tree {
    value: i32,
    children: Vec<Tree>,
}

struct TreeVisitor {}

impl<'de> Visitor<'de> for TreeVisitor {
    type Value = Tree;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a Tree represented as a map or a sequence")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut value: Option<i32> = None;
        let mut children: Option<Vec<Tree>> = None;

        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "value" => {
                    if value.is_some() {
                        return Err(de::Error::duplicate_field("value"));
                    }
                    value = Some(map.next_value()?);
                }
                "children" => {
                    children = Some(map.next_value()?);
                }
                _ => return Err(de::Error::unknown_field(key, &["value", "children"])),
            }
        }

        let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
        let children = children.ok_or_else(|| de::Error::missing_field("children"))?;

        Ok(Tree { value, children })
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let value: i32 = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &"expected [value, children]"))?;
        let children: Vec<Tree> = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &"expected [value, children]"))?;
        if seq.next_element::<de::IgnoredAny>()?.is_some() {
            return Err(de::Error::invalid_length(3, &"expected exactly 2 elements"));
        }
        Ok(Tree { value, children })
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = i32::try_from(v).map_err(|_| de::Error::custom("out of range for i32"))?;
        Ok(Tree {
            value,
            children: vec![],
        })
    }
}

impl<'de> Deserialize<'de> for Tree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(TreeVisitor {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    // --- OK: map формат ---

    #[test]
    fn map_simple() {
        let t = from_str::<Tree>(r#"{ "value": 1, "children": [] }"#).unwrap();

        assert_eq!(t.value, 1);
        assert!(t.children.is_empty());
    }

    #[test]
    fn map_nested() {
        let t = from_str::<Tree>(
            r#"
            {
                "value": 1,
                "children": [
                    { "value": 2, "children": [] },
                    { "value": 3, "children": [] }
                ]
            }
            "#,
        )
        .unwrap();

        assert_eq!(t.value, 1);
        assert_eq!(t.children.len(), 2);
        assert_eq!(t.children[0].value, 2);
        assert_eq!(t.children[1].value, 3);
    }

    // --- OK: seq формат ---

    #[test]
    fn seq_simple() {
        let t = from_str::<Tree>(r#"[1, []]"#).unwrap();

        assert_eq!(t.value, 1);
        assert!(t.children.is_empty());
    }

    #[test]
    fn seq_with_numbers() {
        let t = from_str::<Tree>(r#"[1, [2, 3]]"#).unwrap();

        assert_eq!(t.value, 1);
        assert_eq!(t.children.len(), 2);

        assert_eq!(t.children[0].value, 2);
        assert!(t.children[0].children.is_empty());

        assert_eq!(t.children[1].value, 3);
    }

    #[test]
    fn seq_nested_full() {
        let t = from_str::<Tree>(r#"[1, [[2, []], [3, []]]]"#).unwrap();

        assert_eq!(t.children.len(), 2);
        assert_eq!(t.children[0].value, 2);
        assert_eq!(t.children[1].value, 3);
    }

    // --- Ошибки: map ---

    #[test]
    fn map_missing_value() {
        assert!(from_str::<Tree>(r#"{ "children": [] }"#).is_err());
    }

    #[test]
    fn map_missing_children() {
        assert!(from_str::<Tree>(r#"{ "value": 1 }"#).is_err());
    }

    #[test]
    fn map_duplicate_field() {
        assert!(from_str::<Tree>(r#"{ "value": 1, "value": 2, "children": [] }"#).is_err());
    }

    // --- Ошибки: seq ---

    #[test]
    fn seq_missing_children() {
        assert!(from_str::<Tree>(r#"[1]"#).is_err());
    }

    #[test]
    fn seq_too_many_elements() {
        assert!(from_str::<Tree>(r#"[1, [], 123]"#).is_err());
    }

    // --- Проверка visit_i64 ---

    #[test]
    fn number_as_tree() {
        let t = from_str::<Tree>(r#"5"#).unwrap();

        assert_eq!(t.value, 5);
        assert!(t.children.is_empty());
    }
}

fn main() {}
