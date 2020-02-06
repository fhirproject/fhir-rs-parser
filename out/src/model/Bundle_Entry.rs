#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Bundle_Request::Bundle_Request;
use crate::model::Bundle_Search::Bundle_Search;
use crate::model::ResourceList::ResourceList;
use crate::model::Bundle_Link::Bundle_Link;
use crate::model::Bundle_Response::Bundle_Response;


/// A container for a collection of resources.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle_Entry {
  /// Extensions for fullUrl
  #[serde(rename = "_fullUrl")]
  _full_url: Option<Element>,

  /// Indicates the results of processing the corresponding 'request' entry in the
  /// batch or transaction being responded to or what the results of an operation
  /// where when returning history.
  response: Option<Bundle_Response>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// A series of links that provide context to this entry.
  link: Option<Vec<Bundle_Link>>,

  /// Additional information about how this entry should be processed as part of a
  /// transaction or batch.  For history, it shows how the entry was processed to
  /// create the version contained in the entry.
  request: Option<Bundle_Request>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Information about the search process that lead to the creation of this entry.
  search: Option<Bundle_Search>,

  /// The Resource for the entry. The purpose/meaning of the resource is determined by
  /// the Bundle.type.
  resource: Option<ResourceList>,

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

  /// The Absolute URL for the resource.  The fullUrl SHALL NOT disagree with the id
  /// in the resource - i.e. if the fullUrl is not a urn:uuid, the URL shall be
  /// version-independent URL consistent with the Resource.id. The fullUrl is a
  /// version independent reference to the resource. The fullUrl element SHALL have a
  /// value except that:   * fullUrl can be empty on a POST (although it does not need
  /// to when specifying a temporary id for reference in the bundle)  * Results from
  /// operations might involve resources that are not identified.
  #[serde(rename = "fullUrl")]
  full_url: Option<String>,

}
