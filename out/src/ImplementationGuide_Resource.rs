use serde::{Deserialize, Serialize};

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImplementationGuide_Resource {
  /// Extensions for description
  _description: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// A human assigned name for the resource. All resources SHOULD have a name, but
  /// the name may be extracted from the resource (e.g. ValueSet.name).
  name: String,

  /// Extensions for exampleCanonical
  #[serde(rename = "_exampleCanonical")]
  _example_canonical: Element,

  /// Where this resource is found.
  reference: Reference,

  /// Extensions for name
  _name: Element,

  /// If true or a reference, indicates the resource is an example instance.  If a
  /// reference is present, indicates that the example is an example of the specified
  /// profile.
  #[serde(rename = "exampleBoolean")]
  example_boolean: bool,

  /// Extensions for exampleBoolean
  #[serde(rename = "_exampleBoolean")]
  _example_boolean: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for fhirVersion
  #[serde(rename = "_fhirVersion")]
  _fhir_version: Vec<Element>,

  /// A description of the reason that a resource has been included in the
  /// implementation guide.
  description: String,

  /// Reference to the id of the grouping this resource appears in.
  #[serde(rename = "groupingId")]
  grouping_id: id,

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

  /// If true or a reference, indicates the resource is an example instance.  If a
  /// reference is present, indicates that the example is an example of the specified
  /// profile.
  #[serde(rename = "exampleCanonical")]
  example_canonical: String,

  /// Extensions for groupingId
  #[serde(rename = "_groupingId")]
  _grouping_id: Element,

}
