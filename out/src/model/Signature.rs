#![allow(unused_imports, non_camel_case_types)]

use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Coding::Coding;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// A signature along with supporting context. The signature may be a digital
/// signature that is cryptographic in nature, or some other signature acceptable to
/// the domain. This other signature may be as simple as a graphical image
/// representing a hand-written signature, or a signature ceremony Different
/// signature approaches have different utilities.

#[derive(Debug)]
pub struct Signature<'a> {
  pub value: &'a Value,
}

impl Signature<'_> {
  /// A mime type that indicates the technical format of the target resources signed
  /// by the signature.
  pub fn target_format(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("targetFormat") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A reference to an application-usable description of the identity that is
  /// represented by the signature.
  pub fn on_behalf_of(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("onBehalfOf") {
      return Some(Reference { value: val });
    }
    return None;
  }

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

  /// When the digital signature was signed.
  pub fn when(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("when") {
      return Some(string.to_string());
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

  /// A mime type that indicates the technical format of the signature. Important mime
  /// types are application/signature+xml for X ML DigSig, application/jose for JWS,
  /// and image/* for a graphical image of a signature, etc.
  pub fn sig_format(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("sigFormat") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A reference to an application-usable description of the identity that signed
  /// (e.g. the signature used their private key).
  pub fn who(&self) -> Reference {
    Reference {
      value: &self.value["who"],
    }
  }

  /// The base64 encoding of the Signature content. When signature is not recorded
  /// electronically this element would be empty.
  pub fn data(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("data") {
      return Some(string.to_string());
    }
    return None;
  }

  /// An indication of the reason that the entity signed this document. This may be
  /// explicitly included as part of the signature information and can be used when
  /// determining accountability for various actions concerning the document.
  pub fn fhir_type(&self) -> Vec<Coding> {
    self.value.get("type").unwrap().as_array().unwrap().into_iter().map(|e| Coding { value: e }).collect::<Vec<_>>()
  }

  /// Extensions for when
  pub fn _when(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_when") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for sigFormat
  pub fn _sig_format(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_sigFormat") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for data
  pub fn _data(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_data") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for targetFormat
  pub fn _target_format(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_targetFormat") {
      return Some(Element { value: val });
    }
    return None;
  }

}
