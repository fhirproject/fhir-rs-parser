#![allow(unused_imports, non_camel_case_types)]

use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Period::Period;
use crate::model::CoverageEligibilityResponse_Item::CoverageEligibilityResponse_Item;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.

#[derive(Debug)]
pub struct CoverageEligibilityResponse_Insurance<'a> {
  pub value: &'a Value,
}

impl CoverageEligibilityResponse_Insurance<'_> {
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

  /// Flag indicating if the coverage provided is inforce currently if no service
  /// date(s) specified or for the whole duration of the service dates.
  pub fn inforce(&self) -> Option<bool> {
    if let Some(val) = self.value.get("inforce") {
      return Some(val.as_bool().unwrap());
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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
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

  /// The term of the benefits documented in this response.
  pub fn benefit_period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("benefitPeriod") {
      return Some(Period { value: val });
    }
    return None;
  }

  /// Benefits and optionally current balances, and authorization details by category
  /// or service.
  pub fn item(&self) -> Option<Vec<CoverageEligibilityResponse_Item>> {
    if let Some(Value::Array(val)) = self.value.get("item") {
      return Some(val.into_iter().map(|e| CoverageEligibilityResponse_Item { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for inforce
  pub fn _inforce(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_inforce") {
      return Some(Element { value: val });
    }
    return None;
  }

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.inforce() {
    }
    let _ = self.coverage().validate();
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.benefit_period() {
      _val.validate();
    }
    if let Some(_val) = self.item() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._inforce() {
      _val.validate();
    }
    return true;
  }

}
