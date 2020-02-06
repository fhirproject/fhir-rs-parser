#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Measure_Stratifier::Measure_Stratifier;
use crate::model::Measure_Population::Measure_Population;
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// The Measure resource provides the definition of a quality measure.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Measure_Group {
  /// The stratifier criteria for the measure report, specified as either the name of
  /// a valid CQL expression defined within a referenced library or a valid FHIR
  /// Resource Path.
  stratifier: Option<Vec<Measure_Stratifier>>,

  /// Indicates a meaning for the group. This can be as simple as a unique identifier,
  /// or it can establish meaning in a broader context by drawing from a terminology,
  /// allowing groups to be correlated across measures.
  code: Option<CodeableConcept>,

  /// A population criteria for the measure.
  population: Option<Vec<Measure_Population>>,

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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The human readable description of this population group.
  description: Option<String>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

}
