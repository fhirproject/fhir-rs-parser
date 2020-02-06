#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;


/// A collection of error, warning, or information messages that result from a
/// system action.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationOutcome_Issue {
  /// Additional diagnostic information about the issue.
  diagnostics: Option<String>,

  /// Extensions for location
  #[serde(rename = "_location")]
  _location: Option<Vec<Element>>,

  /// Extensions for expression
  #[serde(rename = "_expression")]
  _expression: Option<Vec<Element>>,

  /// Additional details about the error. This may be a text description of the error
  /// or a system code that identifies the error.
  details: Option<CodeableConcept>,

  /// Describes the type of the issue. The system that creates an OperationOutcome
  /// SHALL choose the most applicable code from the IssueType value set, and may
  /// additional provide its own code for the error in the details element.
  code: Option<OperationOutcome_IssueCode>,

  /// Extensions for diagnostics
  #[serde(rename = "_diagnostics")]
  _diagnostics: Option<Element>,

  /// This element is deprecated because it is XML specific. It is replaced by
  /// issue.expression, which is format independent, and simpler to parse.     For
  /// resource issues, this will be a simple XPath limited to element names,
  /// repetition indicators and the default child accessor that identifies one of the
  /// elements in the resource that caused this issue to be raised.  For HTTP errors,
  /// will be "http." + the parameter name.
  location: Option<Vec<String>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Indicates whether the issue indicates a variation from successful processing.
  severity: Option<OperationOutcome_IssueSeverity>,

  /// Extensions for severity
  #[serde(rename = "_severity")]
  _severity: Option<Element>,

  /// A [simple subset of FHIRPath](fhirpath.html#simple) limited to element names,
  /// repetition indicators and the default child accessor that identifies one of the
  /// elements in the resource that caused this issue to be raised.
  expression: Option<Vec<String>>,

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
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// Extensions for code
  #[serde(rename = "_code")]
  _code: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationOutcome_IssueCode {
  #[serde(rename = "invalid")]
  Invalid,

  #[serde(rename = "structure")]
  Structure,

  #[serde(rename = "required")]
  Required,

  #[serde(rename = "value")]
  Value,

  #[serde(rename = "invariant")]
  Invariant,

  #[serde(rename = "security")]
  Security,

  #[serde(rename = "login")]
  Login,

  #[serde(rename = "unknown")]
  Unknown,

  #[serde(rename = "expired")]
  Expired,

  #[serde(rename = "forbidden")]
  Forbidden,

  #[serde(rename = "suppressed")]
  Suppressed,

  #[serde(rename = "processing")]
  Processing,

  #[serde(rename = "not-supported")]
  NotSupported,

  #[serde(rename = "duplicate")]
  Duplicate,

  #[serde(rename = "multiple-matches")]
  MultipleMatches,

  #[serde(rename = "not-found")]
  NotFound,

  #[serde(rename = "deleted")]
  Deleted,

  #[serde(rename = "too-long")]
  TooLong,

  #[serde(rename = "code-invalid")]
  CodeInvalid,

  #[serde(rename = "extension")]
  Extension,

  #[serde(rename = "too-costly")]
  TooCostly,

  #[serde(rename = "business-rule")]
  BusinessRule,

  #[serde(rename = "conflict")]
  Conflict,

  #[serde(rename = "transient")]
  Transient,

  #[serde(rename = "lock-error")]
  LockError,

  #[serde(rename = "no-store")]
  NoStore,

  #[serde(rename = "exception")]
  Exception,

  #[serde(rename = "timeout")]
  Timeout,

  #[serde(rename = "incomplete")]
  Incomplete,

  #[serde(rename = "throttled")]
  Throttled,

  #[serde(rename = "informational")]
  Informational,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationOutcome_IssueSeverity {
  #[serde(rename = "fatal")]
  Fatal,

  #[serde(rename = "error")]
  Error,

  #[serde(rename = "warning")]
  Warning,

  #[serde(rename = "information")]
  Information,

}
