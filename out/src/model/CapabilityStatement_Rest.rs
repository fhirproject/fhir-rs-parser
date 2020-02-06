#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CapabilityStatement_Security::CapabilityStatement_Security;
use crate::model::Extension::Extension;
use crate::model::CapabilityStatement_Operation::CapabilityStatement_Operation;
use crate::model::CapabilityStatement_SearchParam::CapabilityStatement_SearchParam;
use crate::model::Element::Element;
use crate::model::CapabilityStatement_Resource::CapabilityStatement_Resource;
use crate::model::CapabilityStatement_Interaction1::CapabilityStatement_Interaction1;


/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatement_Rest {
  /// Extensions for mode
  #[serde(rename = "_mode")]
  _mode: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// A specification of restful operations supported by the system.
  interaction: Option<Vec<CapabilityStatement_Interaction1>>,

  /// Information about the system's restful capabilities that apply across all
  /// applications, such as security.
  documentation: Option<String>,

  /// Search parameters that are supported for searching all resources for
  /// implementations to support and/or make use of - either references to ones
  /// defined in the specification, or additional ones defined for/by the
  /// implementation.
  #[serde(rename = "searchParam")]
  search_param: Option<Vec<CapabilityStatement_SearchParam>>,

  /// Definition of an operation or a named query together with its parameters and
  /// their meaning and type.
  operation: Option<Vec<CapabilityStatement_Operation>>,

  /// Extensions for documentation
  #[serde(rename = "_documentation")]
  _documentation: Option<Element>,

  /// An absolute URI which is a reference to the definition of a compartment that the
  /// system supports. The reference is to a CompartmentDefinition resource by its
  /// canonical URL .
  compartment: Option<Vec<String>>,

  /// Identifies whether this portion of the statement is describing the ability to
  /// initiate or receive restful operations.
  mode: Option<CapabilityStatement_RestMode>,

  /// Information about security implementation from an interface perspective - what a
  /// client needs to know.
  security: Option<CapabilityStatement_Security>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

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

  /// A specification of the restful capabilities of the solution for a specific
  /// resource type.
  resource: Option<Vec<CapabilityStatement_Resource>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CapabilityStatement_RestMode {
  #[serde(rename = "client")]
  Client,

  #[serde(rename = "server")]
  Server,

}
