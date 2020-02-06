#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Attachment::Attachment;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Period::Period;
use crate::model::CodeableConcept::CodeableConcept;


/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim_SupportingInfo {
  /// System and code pertaining to the specific information regarding special
  /// conditions relating to the setting, treatment or patient  for which care is
  /// sought.
  code: Option<CodeableConcept>,

  /// Extensions for timingDate
  #[serde(rename = "_timingDate")]
  _timing_date: Option<Element>,

  /// Additional data or information such as resources, documents, images etc.
  /// including references to the data or the actual inclusion of the data.
  #[serde(rename = "valueQuantity")]
  value_quantity: Option<Quantity>,

  /// Extensions for sequence
  #[serde(rename = "_sequence")]
  _sequence: Option<Element>,

  /// The date when or period to which this information refers.
  #[serde(rename = "timingPeriod")]
  timing_period: Option<Period>,

  /// Additional data or information such as resources, documents, images etc.
  /// including references to the data or the actual inclusion of the data.
  #[serde(rename = "valueString")]
  value_string: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The general class of the information supplied: information; exception; accident,
  /// employment; onset, etc.
  category: CodeableConcept,

  /// Additional data or information such as resources, documents, images etc.
  /// including references to the data or the actual inclusion of the data.
  #[serde(rename = "valueBoolean")]
  value_boolean: Option<bool>,

  /// The date when or period to which this information refers.
  #[serde(rename = "timingDate")]
  timing_date: Option<String>,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Option<Element>,

  /// A number to uniquely identify supporting information entries.
  sequence: Option<i32>,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Option<Element>,

  /// Additional data or information such as resources, documents, images etc.
  /// including references to the data or the actual inclusion of the data.
  #[serde(rename = "valueAttachment")]
  value_attachment: Option<Attachment>,

  /// Additional data or information such as resources, documents, images etc.
  /// including references to the data or the actual inclusion of the data.
  #[serde(rename = "valueReference")]
  value_reference: Option<Box<Reference>>,

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

  /// Provides the reason in the situation where a reason code is required in addition
  /// to the content.
  reason: Option<CodeableConcept>,

}
