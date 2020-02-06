#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinition_Type {
  /// Used when the type is "Reference" or "canonical", and identifies a profile
  /// structure or implementation Guide that applies to the target of the reference
  /// this element refers to. If any profiles are specified, then the content must
  /// conform to at least one of them. The URL can be a local reference - to a
  /// contained StructureDefinition, or a reference to another StructureDefinition or
  /// Implementation Guide by a canonical URL. When an implementation guide is
  /// specified, the target resource SHALL conform to at least one profile defined in
  /// the implementation guide.
  #[serde(rename = "targetProfile")]
  target_profile: Option<Vec<String>>,

  /// Extensions for code
  #[serde(rename = "_code")]
  _code: Option<Element>,

  /// Identifies a profile structure or implementation Guide that applies to the
  /// datatype this element refers to. If any profiles are specified, then the content
  /// must conform to at least one of them. The URL can be a local reference - to a
  /// contained StructureDefinition, or a reference to another StructureDefinition or
  /// Implementation Guide by a canonical URL. When an implementation guide is
  /// specified, the type SHALL conform to at least one profile defined in the
  /// implementation guide.
  profile: Option<Vec<String>>,

  /// Whether this reference needs to be version specific or version independent, or
  /// whether either can be used.
  versioning: Option<ElementDefinition_TypeVersioning>,

  /// Extensions for versioning
  #[serde(rename = "_versioning")]
  _versioning: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

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

  /// URL of Data type or Resource that is a(or the) type used for this element.
  /// References are URLs that are relative to http://hl7.org/fhir/StructureDefinition
  /// e.g. "string" is a reference to http://hl7.org/fhir/StructureDefinition/string.
  /// Absolute URLs are only allowed in logical models.
  code: Option<String>,

  /// Extensions for aggregation
  #[serde(rename = "_aggregation")]
  _aggregation: Option<Vec<Element>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ElementDefinition_TypeVersioning {
  #[serde(rename = "either")]
  Either,

  #[serde(rename = "independent")]
  Independent,

  #[serde(rename = "specific")]
  Specific,

}
