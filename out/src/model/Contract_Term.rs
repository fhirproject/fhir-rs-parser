#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Period::Period;
use crate::model::Identifier::Identifier;
use crate::model::Contract_Asset::Contract_Asset;
use crate::model::Contract_Offer::Contract_Offer;
use crate::model::Extension::Extension;
use crate::model::Contract_SecurityLabel::Contract_SecurityLabel;
use crate::model::Contract_Action::Contract_Action;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract_Term<'a> {
  pub value: &'a Value,
}

impl Contract_Term<'_> {
  /// Relevant time or time-period when this Contract Provision is applicable.
  pub fn applies(&self) -> Option<Period> {
    if let Some(val) = self.value.get("applies") {
      return Some(Period { value: val });
    }
    return None;
  }

  /// Extensions for text
  pub fn _text(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_text") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// An actor taking a role in an activity for which it can be assigned some degree
  /// of responsibility for the activity taking place.
  pub fn action(&self) -> Option<Vec<Contract_Action>> {
    if let Some(Value::Array(val)) = self.value.get("action") {
      return Some(val.into_iter().map(|e| Contract_Action { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for issued
  pub fn _issued(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_issued") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The entity that the term applies to.
  pub fn topic_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("topicReference") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Nested group of Contract Provisions.
  pub fn group(&self) -> Option<Vec<Contract_Term>> {
    if let Some(Value::Array(val)) = self.value.get("group") {
      return Some(val.into_iter().map(|e| Contract_Term { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Contract Term Asset List.
  pub fn asset(&self) -> Option<Vec<Contract_Asset>> {
    if let Some(Value::Array(val)) = self.value.get("asset") {
      return Some(val.into_iter().map(|e| Contract_Asset { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A specialized legal clause or condition based on overarching contract type.
  pub fn sub_type(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("subType") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The entity that the term applies to.
  pub fn topic_codeable_concept(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("topicCodeableConcept") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Unique identifier for this particular Contract Provision.
  pub fn identifier(&self) -> Option<Identifier> {
    if let Some(val) = self.value.get("identifier") {
      return Some(Identifier { value: val });
    }
    return None;
  }

  /// The matter of concern in the context of this provision of the agrement.
  pub fn offer(&self) -> Contract_Offer {
    Contract_Offer {
      value: &self.value["offer"],
    }
  }

  /// When this Contract Provision was issued.
  pub fn issued(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("issued") {
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

  /// A legal clause or condition contained within a contract that requires one or
  /// both parties to perform a particular requirement by some specified time or
  /// prevents one or both parties from performing a particular requirement by some
  /// specified time.
  pub fn fhir_type(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("type") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Statement of a provision in a policy or a contract.
  pub fn text(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("text") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Security labels that protect the handling of information about the term and its
  /// elements, which may be specifically identified..
  pub fn security_label(&self) -> Option<Vec<Contract_SecurityLabel>> {
    if let Some(Value::Array(val)) = self.value.get("securityLabel") {
      return Some(val.into_iter().map(|e| Contract_SecurityLabel { value: e }).collect::<Vec<_>>());
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

}
