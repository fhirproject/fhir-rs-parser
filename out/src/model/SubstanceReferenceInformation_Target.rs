#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::Quantity::Quantity;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Range::Range;


/// Todo.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformation_Target {
  /// Todo.
  #[serde(rename = "organismType")]
  organism_type: Option<CodeableConcept>,

  /// Todo.
  #[serde(rename = "amountRange")]
  amount_range: Option<Range>,

  /// Todo.
  #[serde(rename = "amountString")]
  amount_string: Option<String>,

  /// Todo.
  interaction: Option<CodeableConcept>,

  /// Extensions for amountString
  #[serde(rename = "_amountString")]
  _amount_string: Option<Element>,

  /// Todo.
  source: Option<Vec<Box<Reference>>>,

  /// Todo.
  #[serde(rename = "amountType")]
  amount_type: Option<CodeableConcept>,

  /// Todo.
  target: Option<Identifier>,

  /// Todo.
  #[serde(rename = "amountQuantity")]
  amount_quantity: Option<Quantity>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

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
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Extension>>,

  /// Todo.
  organism: Option<CodeableConcept>,

  /// Todo.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

}
