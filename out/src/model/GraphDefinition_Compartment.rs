#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A formal computable definition of a graph of resources - that is, a coherent set
/// of resources that form a graph by following references. The Graph Definition
/// resource defines a set and makes rules about the set.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinition_Compartment {
  /// Extensions for code
  _code: Element,

  /// Identifies the compartment.
  code: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for description
  _description: Element,

  /// Defines how the compartment rule is used - whether it it is used to test whether
  /// resources are subject to the rule, or whether it is a rule that must be
  /// followed.
  #[serde(rename = "use")]
  fhir_use: GraphDefinition_CompartmentUse,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Custom rule, as a FHIRPath expression.
  expression: String,

  /// Documentation for FHIRPath expression.
  description: String,

  /// Extensions for use
  _use: Element,

  /// Extensions for expression
  _expression: Element,

  /// Extensions for rule
  _rule: Element,

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

  /// identical | matching | different | no-rule | custom.
  rule: GraphDefinition_CompartmentRule,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum GraphDefinition_CompartmentUse {
  #[serde(rename = "condition")]
  Condition,

  #[serde(rename = "requirement")]
  Requirement,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum GraphDefinition_CompartmentRule {
  #[serde(rename = "identical")]
  Identical,

  #[serde(rename = "matching")]
  Matching,

  #[serde(rename = "different")]
  Different,

  #[serde(rename = "custom")]
  Custom,

}
