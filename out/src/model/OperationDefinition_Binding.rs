#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationDefinition_Binding {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Indicates the degree of conformance expectations associated with this binding -
  /// that is, the degree to which the provided value set must be adhered to in the
  /// instances.
  strength: Option<OperationDefinition_BindingStrength>,

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

  /// Extensions for strength
  #[serde(rename = "_strength")]
  _strength: Option<Element>,

  /// Points to the value set or external definition (e.g. implicit value set) that
  /// identifies the set of codes to be used.
  #[serde(rename = "valueSet")]
  value_set: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationDefinition_BindingStrength {
  #[serde(rename = "required")]
  Required,

  #[serde(rename = "extensible")]
  Extensible,

  #[serde(rename = "preferred")]
  Preferred,

  #[serde(rename = "example")]
  Example,

}
