#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A container for a collection of resources.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle_Request {
  /// The URL for this entry, relative to the root (the address to which the request
  /// is posted).
  url: String,

  /// Extensions for ifModifiedSince
  #[serde(rename = "_ifModifiedSince")]
  _if_modified_since: Element,

  /// Extensions for url
  _url: Element,

  /// Extensions for ifNoneExist
  #[serde(rename = "_ifNoneExist")]
  _if_none_exist: Element,

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

  /// Only perform the operation if the Etag value matches. For more information, see
  /// the API section ["Managing Resource Contention"](http.html#concurrency).
  #[serde(rename = "ifMatch")]
  if_match: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Only perform the operation if the last updated date matches. See the API
  /// documentation for ["Conditional Read"](http.html#cread).
  #[serde(rename = "ifModifiedSince")]
  if_modified_since: i32,

  /// Extensions for ifMatch
  #[serde(rename = "_ifMatch")]
  _if_match: Element,

  /// Extensions for ifNoneMatch
  #[serde(rename = "_ifNoneMatch")]
  _if_none_match: Element,

  /// Extensions for method
  _method: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Instruct the server not to perform the create if a specified resource already
  /// exists. For further information, see the API documentation for ["Conditional
  /// Create"](http.html#ccreate). This is just the query portion of the URL - what
  /// follows the "?" (not including the "?").
  #[serde(rename = "ifNoneExist")]
  if_none_exist: String,

  /// In a transaction or batch, this is the HTTP action to be executed for this
  /// entry. In a history bundle, this indicates the HTTP action that occurred.
  method: Bundle_RequestMethod,

  /// If the ETag values match, return a 304 Not Modified status. See the API
  /// documentation for ["Conditional Read"](http.html#cread).
  #[serde(rename = "ifNoneMatch")]
  if_none_match: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Bundle_RequestMethod {
  #[serde(rename = "GET")]
  GET,

  #[serde(rename = "HEAD")]
  HEAD,

  #[serde(rename = "POST")]
  POST,

  #[serde(rename = "PUT")]
  PUT,

  #[serde(rename = "DELETE")]
  DELETE,

  #[serde(rename = "PATCH")]
  PATCH,

}
