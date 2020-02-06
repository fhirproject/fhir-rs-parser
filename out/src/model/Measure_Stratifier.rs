#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Expression::Expression;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Measure_Component::Measure_Component;
use crate::model::Extension::Extension;


/// The Measure resource provides the definition of a quality measure.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Measure_Stratifier {
  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The human readable description of this stratifier criteria.
  description: Option<String>,

  /// An expression that specifies the criteria for the stratifier. This is typically
  /// the name of an expression defined within a referenced library, but it may also
  /// be a path to a stratifier element.
  criteria: Option<Expression>,

  /// A component of the stratifier criteria for the measure report, specified as
  /// either the name of a valid CQL expression defined within a referenced library or
  /// a valid FHIR Resource Path.
  component: Option<Vec<Measure_Component>>,

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

  /// Indicates a meaning for the stratifier. This can be as simple as a unique
  /// identifier, or it can establish meaning in a broader context by drawing from a
  /// terminology, allowing stratifiers to be correlated across measures.
  code: Option<CodeableConcept>,

}
