#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Coding::Coding;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// The CodeSystem resource is used to declare the existence of and describe a code
/// system or code system supplement and its key properties, and optionally define a
/// part or all of its content.

#[derive(Debug)]
pub struct CodeSystem_Property1<'a> {
  pub value: &'a Value,
}

impl CodeSystem_Property1<'_> {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  pub fn extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("extension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The value of this property.
  pub fn value_integer(&self) -> Option<i64> {
    if let Some(val) = self.value.get("valueInteger") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// The value of this property.
  pub fn value_boolean(&self) -> Option<bool> {
    if let Some(val) = self.value.get("valueBoolean") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

  /// The value of this property.
  pub fn value_code(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("valueCode") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for valueCode
  pub fn _value_code(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_valueCode") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for valueString
  pub fn _value_string(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_valueString") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The value of this property.
  pub fn value_date_time(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("valueDateTime") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for valueInteger
  pub fn _value_integer(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_valueInteger") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for valueDateTime
  pub fn _value_date_time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_valueDateTime") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for valueBoolean
  pub fn _value_boolean(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_valueBoolean") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The value of this property.
  pub fn value_decimal(&self) -> Option<i64> {
    if let Some(val) = self.value.get("valueDecimal") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// Extensions for code
  pub fn _code(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_code") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The value of this property.
  pub fn value_coding(&self) -> Option<Coding> {
    if let Some(val) = self.value.get("valueCoding") {
      return Some(Coding { value: val });
    }
    return None;
  }

  /// The value of this property.
  pub fn value_string(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("valueString") {
      return Some(string.to_string());
    }
    return None;
  }

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for valueDecimal
  pub fn _value_decimal(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_valueDecimal") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A code that is a reference to CodeSystem.property.code.
  pub fn code(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("code") {
      return Some(string.to_string());
    }
    return None;
  }

}
