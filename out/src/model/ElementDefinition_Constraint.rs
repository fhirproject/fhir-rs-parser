#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinition_Constraint {
  /// Extensions for severity
  #[serde(rename = "_severity")]
  _severity: Option<Element>,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Allows identification of which elements have their cardinalities impacted by the
  /// constraint.  Will not be referenced for constraints that do not affect
  /// cardinality.
  key: Option<String>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for human
  #[serde(rename = "_human")]
  _human: Option<Element>,

  /// A [FHIRPath](fhirpath.html) expression of constraint that can be executed to see
  /// if this constraint is met.
  expression: Option<String>,

  /// An XPath expression of constraint that can be executed to see if this constraint
  /// is met.
  xpath: Option<String>,

  /// Description of why this constraint is necessary or appropriate.
  requirements: Option<String>,

  /// Text that can be used to describe the constraint in messages identifying that
  /// the constraint has been violated.
  human: Option<String>,

  /// Extensions for xpath
  #[serde(rename = "_xpath")]
  _xpath: Option<Element>,

  /// Extensions for requirements
  #[serde(rename = "_requirements")]
  _requirements: Option<Element>,

  /// A reference to the original source of the constraint, for traceability purposes.
  source: Option<String>,

  /// Extensions for expression
  #[serde(rename = "_expression")]
  _expression: Option<Element>,

  /// Identifies the impact constraint violation has on the conformance of the
  /// instance.
  severity: Option<ElementDefinition_ConstraintSeverity>,

  /// Extensions for key
  #[serde(rename = "_key")]
  _key: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ElementDefinition_ConstraintSeverity {
  #[serde(rename = "error")]
  Error,

  #[serde(rename = "warning")]
  Warning,

}
