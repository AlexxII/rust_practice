use crate::iterators::tree::Node;
use serde::de::Visitor;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, de};

impl<T: Serialize> Serialize for Node<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 2)?;

        state.serialize_field("value", &self.value)?;
        state.serialize_field("children", &self.children)?;
        state.end()
    }
}

struct NodeVisitor<T> {
    marker: std::marker::PhantomData<T>,
}

impl<'de, T> Visitor<'de> for NodeVisitor<T> {
    type Value = Node<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "struct Node")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut value = None;
        let mut children = None;

        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "value" => {
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

        Ok(Node { value, children })
    }
}

impl<'de, T> Deserialize<'de> for Node<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Node",
            &["value", "children"],
            NodeVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}
