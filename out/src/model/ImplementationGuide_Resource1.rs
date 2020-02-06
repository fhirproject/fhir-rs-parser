#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;


/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuide_Resource1 {
  /// If true or a reference, indicates the resource is an example instance.  If a
  /// reference is present, indicates that the example is an example of the specified
  /// profile.
  #[serde(rename = "exampleBoolean")]
  example_boolean: bool,

  /// Extensions for relativePath
  #[serde(rename = "_relativePath")]
  _relative_path: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

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

  /// If true or a reference, indicates the resource is an example instance.  If a
  /// reference is present, indicates that the example is an example of the specified
  /// profile.
  #[serde(rename = "exampleCanonical")]
  example_canonical: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for exampleBoolean
  #[serde(rename = "_exampleBoolean")]
  _example_boolean: Element,

  /// Where this resource is found.
  reference: Box<Reference>,

  /// Extensions for exampleCanonical
  #[serde(rename = "_exampleCanonical")]
  _example_canonical: Element,

  /// The relative path for primary page for this resource within the IG.
  #[serde(rename = "relativePath")]
  relative_path: String,

}
