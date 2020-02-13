#![allow(unused_imports, non_camel_case_types)]

use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// This resource provides the adjudication details from the processing of a Claim
/// resource.

#[derive(Debug)]
pub struct ClaimResponse_Insurance<'a> {
  pub value: &'a Value,
}

impl ClaimResponse_Insurance<'_> {
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

  /// Extensions for businessArrangement
  pub fn _business_arrangement(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_businessArrangement") {
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

  /// Extensions for focal
  pub fn _focal(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_focal") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The result of the adjudication of the line items for the Coverage specified in
  /// this insurance.
  pub fn claim_response(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("claimResponse") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Reference to the insurance card level information contained in the Coverage
  /// resource. The coverage issuing insurer will use these details to locate the
  /// patient's actual coverage within the insurer's information system.
  pub fn coverage(&self) -> Reference {
    Reference {
      value: &self.value["coverage"],
    }
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

  /// A flag to indicate that this Coverage is to be used for adjudication of this
  /// claim when set to true.
  pub fn focal(&self) -> Option<bool> {
    if let Some(val) = self.value.get("focal") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

  /// A number to uniquely identify insurance entries and provide a sequence of
  /// coverages to convey coordination of benefit order.
  pub fn sequence(&self) -> Option<i64> {
    if let Some(val) = self.value.get("sequence") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// Extensions for sequence
  pub fn _sequence(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_sequence") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A business agreement number established between the provider and the insurer for
  /// special business processing purposes.
  pub fn business_arrangement(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("businessArrangement") {
      return Some(string.to_string());
    }
    return None;
  }

}
