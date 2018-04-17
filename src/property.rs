/// High-level Property base class implementation.

use serde_json;

use thing::Thing;

pub trait Property {
    /// Initialize the object.
    ///
    /// thing -- the Thing this property belongs to
    /// name -- name of the property
    /// value -- Value object to hold the property value
    /// metadata -- property metadata, i.e. type, description, unit, etc., as a Map
    fn new(
        thing: Thing,
        name: String,
        value: serde_json::Value,
        metadata: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Self;

    /// Get the property description.
    ///
    /// Returns a JSON value describing the property.
    fn as_property_description(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut description = self.get_metadata().clone();
        description.insert("href".to_string(), json!(self.get_href()));
        description
    }

    /// Set the prefix of any hrefs associated with this property.
    ///
    /// prefix -- the prefix
    fn set_href_prefix(&mut self, prefix: String);

    /// Get the href of this property.
    fn get_href(&self) -> String;

    /// Get the current property value.
    fn get_value(&self) -> serde_json::Value;

    /// Set the current value of the property.
    ///
    /// value -- the value to set
    fn set_value(&self, value: serde_json::Value);

    /// Get the name of this property.
    fn get_name(&self) -> String;

    /// Get the thing associated with this property.
    fn get_thing(&self) -> &Thing;

    /// Get the metadata associated with this property.
    fn get_metadata(&self) -> serde_json::Map<String, serde_json::Value>;
}

/// A Property represents an individual state value of a thing.
pub struct BaseProperty {
    thing: Thing,
    name: String,
    value: serde_json::Value,
    href_prefix: String,
    href: String,
    metadata: serde_json::Map<String, serde_json::Value>,
}

impl Property for BaseProperty {
    /// Initialize the object.
    ///
    /// thing -- the Thing this property belongs to
    /// name -- name of the property
    /// value -- Value object to hold the property value
    /// metadata -- property metadata, i.e. type, description, unit, etc., as a Map
    fn new(
        thing: Thing,
        name: String,
        value: serde_json::Value,
        metadata: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> BaseProperty {
        let meta = match metadata {
            Some(m) => m,
            None => serde_json::Map::new(),
        };

        let href = format!("/properties/{}", name);

        BaseProperty {
            thing: thing,
            name: name,
            value: value,
            href_prefix: "".to_owned(),
            href: href,
            metadata: meta,
        }

        // TODO: Add the property change observer to notify the Thing about a property change.
        // self.value.on('update', lambda _: self.thing.property_notify(self))
    }

    /// Set the prefix of any hrefs associated with this property.
    ///
    /// prefix -- the prefix
    fn set_href_prefix(&mut self, prefix: String) {
        self.href_prefix = prefix;
    }

    /// Get the href of this property.
    fn get_href(&self) -> String {
        format!("{}{}", self.href_prefix, self.href).to_string()
    }

    /// Get the current property value.
    fn get_value(&self) -> serde_json::Value {
        self.value.clone()
    }

    /// Set the current value of the property.
    ///
    /// value -- the value to set
    fn set_value(&self, value: serde_json::Value) {
        // TODO: self.value.set(value);
    }

    /// Get the name of this property.
    fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Get the thing associated with this property.
    fn get_thing(&self) -> &Thing {
        &self.thing
    }

    /// Get the metadata associated with this property.
    fn get_metadata(&self) -> serde_json::Map<String, serde_json::Value> {
        self.metadata.clone()
    }
}
