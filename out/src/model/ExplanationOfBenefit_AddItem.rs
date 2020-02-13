#![allow(unused_imports, non_camel_case_types)]

use crate::model::ExplanationOfBenefit_Adjudication::ExplanationOfBenefit_Adjudication;
use crate::model::Address::Address;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ExplanationOfBenefit_Detail1::ExplanationOfBenefit_Detail1;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::Money::Money;
use crate::model::Quantity::Quantity;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.

#[derive(Debug)]
pub struct ExplanationOfBenefit_AddItem<'a> {
  pub value: &'a Value,
}

impl ExplanationOfBenefit_AddItem<'_> {
  /// A region or surface of the bodySite, e.g. limb region or tooth surface(s).
  pub fn sub_site(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("subSite") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The number of repetitions of a service or product.
  pub fn quantity(&self) -> Option<Quantity> {
    if let Some(val) = self.value.get("quantity") {
      return Some(Quantity { value: val });
    }
    return None;
  }

  /// The quantity times the unit price for an additional service or product or
  /// charge.
  pub fn net(&self) -> Option<Money> {
    if let Some(val) = self.value.get("net") {
      return Some(Money { value: val });
    }
    return None;
  }

  /// Claim items which this service line is intended to replace.
  pub fn item_sequence(&self) -> Option<Vec<i64>> {
    if let Some(Value::Array(val)) = self.value.get("itemSequence") {
      return Some(val.into_iter().map(|e| e.as_i64().unwrap()).collect::<Vec<_>>());
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

  /// Extensions for servicedDate
  pub fn _serviced_date(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_servicedDate") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Physical service site on the patient (limb, tooth, etc.).
  pub fn body_site(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("bodySite") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The adjudication results.
  pub fn adjudication(&self) -> Option<Vec<ExplanationOfBenefit_Adjudication>> {
    if let Some(Value::Array(val)) = self.value.get("adjudication") {
      return Some(val.into_iter().map(|e| ExplanationOfBenefit_Adjudication { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The sequence number of the details within the claim item which this line is
  /// intended to replace.
  pub fn detail_sequence(&self) -> Option<Vec<i64>> {
    if let Some(Value::Array(val)) = self.value.get("detailSequence") {
      return Some(val.into_iter().map(|e| e.as_i64().unwrap()).collect::<Vec<_>>());
    }
    return None;
  }

  /// The date or dates when the service or product was supplied, performed or
  /// completed.
  pub fn serviced_date(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("servicedDate") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for itemSequence
  pub fn _item_sequence(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_itemSequence") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The date or dates when the service or product was supplied, performed or
  /// completed.
  pub fn serviced_period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("servicedPeriod") {
      return Some(Period { value: val });
    }
    return None;
  }

  /// Where the product or service was provided.
  pub fn location_codeable_concept(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("locationCodeableConcept") {
      return Some(CodeableConcept { value: val });
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

  /// The providers who are authorized for the services rendered to the patient.
  pub fn provider(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("provider") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Identifies the program under which this may be recovered.
  pub fn program_code(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("programCode") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for subDetailSequence
  pub fn _sub_detail_sequence(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_subDetailSequence") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for factor
  pub fn _factor(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_factor") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// When the value is a group code then this item collects a set of related claim
  /// details, otherwise this contains the product, service, drug or other billing
  /// code for the item.
  pub fn product_or_service(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["productOrService"],
    }
  }

  /// Where the product or service was provided.
  pub fn location_address(&self) -> Option<Address> {
    if let Some(val) = self.value.get("locationAddress") {
      return Some(Address { value: val });
    }
    return None;
  }

  /// A real number that represents a multiplier used in determining the overall value
  /// of services delivered and/or goods received. The concept of a Factor allows for
  /// a discount or surcharge multiplier to be applied to a monetary amount.
  pub fn factor(&self) -> Option<f64> {
    if let Some(val) = self.value.get("factor") {
      return Some(val.as_f64().unwrap());
    }
    return None;
  }

  /// The sequence number of the sub-details woithin the details within the claim item
  /// which this line is intended to replace.
  pub fn sub_detail_sequence(&self) -> Option<Vec<i64>> {
    if let Some(Value::Array(val)) = self.value.get("subDetailSequence") {
      return Some(val.into_iter().map(|e| e.as_i64().unwrap()).collect::<Vec<_>>());
    }
    return None;
  }

  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  pub fn modifier(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("modifier") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Where the product or service was provided.
  pub fn location_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("locationReference") {
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

  /// Extensions for detailSequence
  pub fn _detail_sequence(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_detailSequence") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// If the item is not a group then this is the fee for the product or service,
  /// otherwise this is the total of the fees for the details of the group.
  pub fn unit_price(&self) -> Option<Money> {
    if let Some(val) = self.value.get("unitPrice") {
      return Some(Money { value: val });
    }
    return None;
  }

  /// Extensions for noteNumber
  pub fn _note_number(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_noteNumber") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The numbers associated with notes below which apply to the adjudication of this
  /// item.
  pub fn note_number(&self) -> Option<Vec<i64>> {
    if let Some(Value::Array(val)) = self.value.get("noteNumber") {
      return Some(val.into_iter().map(|e| e.as_i64().unwrap()).collect::<Vec<_>>());
    }
    return None;
  }

  /// The second-tier service adjudications for payor added services.
  pub fn detail(&self) -> Option<Vec<ExplanationOfBenefit_Detail1>> {
    if let Some(Value::Array(val)) = self.value.get("detail") {
      return Some(val.into_iter().map(|e| ExplanationOfBenefit_Detail1 { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

}
