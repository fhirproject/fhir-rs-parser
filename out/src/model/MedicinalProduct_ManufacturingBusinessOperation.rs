#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;


/// Detailed definition of a medicinal product, typically for uses other than direct
/// patient care (e.g. regulatory use).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProduct_ManufacturingBusinessOperation {
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
  modifier_extension: Vec<Extension>,

  /// To indicate if this proces is commercially confidential.
  #[serde(rename = "confidentialityIndicator")]
  confidentiality_indicator: CodeableConcept,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// A regulator which oversees the operation.
  regulator: Box<Reference>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The type of manufacturing operation.
  #[serde(rename = "operationType")]
  operation_type: CodeableConcept,

  /// The manufacturer or establishment associated with the process.
  manufacturer: Vec<Box<Reference>>,

  /// Extensions for effectiveDate
  #[serde(rename = "_effectiveDate")]
  _effective_date: Element,

  /// Regulatory authorization reference number.
  #[serde(rename = "authorisationReferenceNumber")]
  authorisation_reference_number: Identifier,

  /// Regulatory authorization date.
  #[serde(rename = "effectiveDate")]
  effective_date: String,

}
