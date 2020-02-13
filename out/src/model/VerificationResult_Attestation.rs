#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Signature::Signature;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use serde_json::value::Value;



/// Describes validation requirements, source(s), status and dates for one or more
/// elements.

#[derive(Debug)]
pub struct VerificationResult_Attestation<'a> {
  pub value: &'a Value,
}

impl VerificationResult_Attestation<'_> {
  /// The method by which attested information was submitted/retrieved (manual; API;
  /// Push).
  pub fn communication_method(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("communicationMethod") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Extensions for sourceIdentityCertificate
  pub fn _source_identity_certificate(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_sourceIdentityCertificate") {
      return Some(Element { value: val });
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

  /// When the who is asserting on behalf of another (organization or individual).
  pub fn on_behalf_of(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("onBehalfOf") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Signed assertion by the proxy entity indicating that they have the right to
  /// submit attested information on behalf of the attestation source.
  pub fn proxy_signature(&self) -> Option<Signature> {
    if let Some(val) = self.value.get("proxySignature") {
      return Some(Signature { value: val });
    }
    return None;
  }

  /// Signed assertion by the attestation source that they have attested to the
  /// information.
  pub fn source_signature(&self) -> Option<Signature> {
    if let Some(val) = self.value.get("sourceSignature") {
      return Some(Signature { value: val });
    }
    return None;
  }

  /// The date the information was attested to.
  pub fn date(&self) -> Option<i64> {
    if let Some(val) = self.value.get("date") {
      return Some(val.as_i64().unwrap());
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

  /// A digital identity certificate associated with the attestation source.
  pub fn source_identity_certificate(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("sourceIdentityCertificate") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The individual or organization attesting to information.
  pub fn who(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("who") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Extensions for date
  pub fn _date(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_date") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A digital identity certificate associated with the proxy entity submitting
  /// attested information on behalf of the attestation source.
  pub fn proxy_identity_certificate(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("proxyIdentityCertificate") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for proxyIdentityCertificate
  pub fn _proxy_identity_certificate(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_proxyIdentityCertificate") {
      return Some(Element { value: val });
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

}
