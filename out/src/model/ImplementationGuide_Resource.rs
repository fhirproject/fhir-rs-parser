#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Reference::Reference;


/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuide_Resource {
  /// Extensions for exampleBoolean
  #[serde(rename = "_exampleBoolean")]
  _example_boolean: Option<Element>,

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

  /// Extensions for fhirVersion
  #[serde(rename = "_fhirVersion")]
  _fhir_version: Option<Vec<Element>>,

  /// If true or a reference, indicates the resource is an example instance.  If a
  /// reference is present, indicates that the example is an example of the specified
  /// profile.
  #[serde(rename = "exampleBoolean")]
  example_boolean: Option<bool>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// If true or a reference, indicates the resource is an example instance.  If a
  /// reference is present, indicates that the example is an example of the specified
  /// profile.
  #[serde(rename = "exampleCanonical")]
  example_canonical: Option<String>,

  /// Extensions for exampleCanonical
  #[serde(rename = "_exampleCanonical")]
  _example_canonical: Option<Element>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// A human assigned name for the resource. All resources SHOULD have a name, but
  /// the name may be extracted from the resource (e.g. ValueSet.name).
  name: Option<String>,

  /// Extensions for groupingId
  #[serde(rename = "_groupingId")]
  _grouping_id: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Reference to the id of the grouping this resource appears in.
  #[serde(rename = "groupingId")]
  grouping_id: Option<String>,

  /// Where this resource is found.
  reference: Box<Reference>,

  /// A description of the reason that a resource has been included in the
  /// implementation guide.
  description: Option<String>,

}
