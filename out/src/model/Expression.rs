#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A expression that is evaluated in a specified context and returns a value. The
/// context of use of the expression must specify the context in which the
/// expression is evaluated, and how the result of the expression is used.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expression {
  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Extensions for expression
  #[serde(rename = "_expression")]
  _expression: Option<Element>,

  /// An expression in the specified language that returns a value.
  expression: Option<String>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// The media type of the language for the expression.
  language: Option<ExpressionLanguage>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// A URI that defines where the expression is found.
  reference: Option<String>,

  /// Extensions for reference
  #[serde(rename = "_reference")]
  _reference: Option<Element>,

  /// A short name assigned to the expression to allow for multiple reuse of the
  /// expression in the context where it is defined.
  name: Option<String>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// A brief, natural language description of the condition that effectively
  /// communicates the intended semantics.
  description: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExpressionLanguage {
  #[serde(rename = "text/cql")]
  TextCql,

  #[serde(rename = "text/fhirpath")]
  TextFhirpath,

  #[serde(rename = "application/x-fhir-query")]
  ApplicationXFhirQuery,

}
