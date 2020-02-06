#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Coding::Coding;


/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataRequirement_CodeFilter {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The valueset for the code filter. The valueSet and code elements are additive.
  /// If valueSet is specified, the filter will return only those data items for which
  /// the value of the code-valued element specified in the path is a member of the
  /// specified valueset.
  #[serde(rename = "valueSet")]
  value_set: Option<String>,

  /// Extensions for searchParam
  #[serde(rename = "_searchParam")]
  _search_param: Option<Element>,

  /// A token parameter that refers to a search parameter defined on the specified
  /// type of the DataRequirement, and which searches on elements of type code,
  /// Coding, or CodeableConcept.
  #[serde(rename = "searchParam")]
  search_param: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for path
  #[serde(rename = "_path")]
  _path: Option<Element>,

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

  /// The codes for the code filter. If values are given, the filter will return only
  /// those data items for which the code-valued attribute specified by the path has a
  /// value that is one of the specified codes. If codes are specified in addition to
  /// a value set, the filter returns items matching a code in the value set or one of
  /// the specified codes.
  code: Option<Vec<Coding>>,

  /// The code-valued attribute of the filter. The specified path SHALL be a FHIRPath
  /// resolveable on the specified type of the DataRequirement, and SHALL consist only
  /// of identifiers, constant indexers, and .resolve(). The path is allowed to
  /// contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to
  /// traverse multiple-cardinality sub-elements (see the [Simple FHIRPath
  /// Profile](fhirpath.html#simple) for full details). Note that the index must be an
  /// integer constant. The path must resolve to an element of type code, Coding, or
  /// CodeableConcept.
  path: Option<String>,

}
