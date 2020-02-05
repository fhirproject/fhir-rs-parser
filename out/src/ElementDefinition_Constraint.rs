use serde::{Deserialize, Serialize};

/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElementDefinition_Constraint {
  /// A [FHIRPath](fhirpath.html) expression of constraint that can be executed to see
  /// if this constraint is met.
  expression: String,

  /// An XPath expression of constraint that can be executed to see if this constraint
  /// is met.
  xpath: String,

  /// Extensions for requirements
  _requirements: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Extensions for severity
  _severity: Element,

  /// Identifies the impact constraint violation has on the conformance of the
  /// instance.
  severity: ElementDefinition_ConstraintSeverity,

  /// Allows identification of which elements have their cardinalities impacted by the
  /// constraint.  Will not be referenced for constraints that do not affect
  /// cardinality.
  key: id,

  /// Description of why this constraint is necessary or appropriate.
  requirements: String,

  /// Extensions for xpath
  _xpath: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for expression
  _expression: Element,

  /// Extensions for key
  _key: Element,

  /// Text that can be used to describe the constraint in messages identifying that
  /// the constraint has been violated.
  human: String,

  /// A reference to the original source of the constraint, for traceability purposes.
  source: canonical,

  /// Extensions for human
  _human: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

}

#[derive(Debug, Serialize, Deserialize)]
enum ElementDefinition_ConstraintSeverity {
  #[serde(rename = "error")]
  Error,

  #[serde(rename = "warning")]
  Warning,

}
