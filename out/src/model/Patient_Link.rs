#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Reference::Reference;


/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient_Link {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

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

  /// The other patient resource that the link refers to.
  other: Box<Reference>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// The type of link between this patient resource and another patient resource.
  #[serde(rename = "type")]
  fhir_type: Option<Patient_LinkType>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Patient_LinkType {
  #[serde(rename = "replaced-by")]
  ReplacedBy,

  #[serde(rename = "replaces")]
  Replaces,

  #[serde(rename = "refer")]
  Refer,

  #[serde(rename = "seealso")]
  Seealso,

}
