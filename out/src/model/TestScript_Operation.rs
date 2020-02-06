#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::TestScript_RequestHeader::TestScript_RequestHeader;
use crate::model::Coding::Coding;
use crate::model::Element::Element;


/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScript_Operation {
  /// The mime-type to use for RESTful operation in the 'Accept' header.
  accept: Option<String>,

  /// Extensions for responseId
  #[serde(rename = "_responseId")]
  _response_id: Option<Element>,

  /// The label would be used for tracking/logging purposes by test engines.
  label: Option<String>,

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

  /// The HTTP method the test engine MUST use for this operation regardless of any
  /// other operation details.
  method: Option<TestScript_OperationMethod>,

  /// Extensions for encodeRequestUrl
  #[serde(rename = "_encodeRequestUrl")]
  _encode_request_url: Option<Element>,

  /// Extensions for params
  #[serde(rename = "_params")]
  _params: Option<Element>,

  /// Extensions for destination
  #[serde(rename = "_destination")]
  _destination: Option<Element>,

  /// The type of the resource.  See http://build.fhir.org/resourcelist.html.
  resource: Option<String>,

  /// Extensions for resource
  #[serde(rename = "_resource")]
  _resource: Option<Element>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The description would be used by test engines for tracking and reporting
  /// purposes.
  description: Option<String>,

  /// Whether or not to implicitly send the request url in encoded format. The default
  /// is true to match the standard RESTful client behavior. Set to false when
  /// communicating with a server that does not support encoded url paths.
  #[serde(rename = "encodeRequestUrl")]
  encode_request_url: Option<bool>,

  /// The fixture id (maybe new) to map to the response.
  #[serde(rename = "responseId")]
  response_id: Option<String>,

  /// Extensions for accept
  #[serde(rename = "_accept")]
  _accept: Option<Element>,

  /// Server interaction or operation type.
  #[serde(rename = "type")]
  fhir_type: Option<Coding>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Complete request URL.
  url: Option<String>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for label
  #[serde(rename = "_label")]
  _label: Option<Element>,

  /// Extensions for contentType
  #[serde(rename = "_contentType")]
  _content_type: Option<Element>,

  /// Header elements would be used to set HTTP headers.
  #[serde(rename = "requestHeader")]
  request_header: Option<Vec<TestScript_RequestHeader>>,

  /// Extensions for requestId
  #[serde(rename = "_requestId")]
  _request_id: Option<Element>,

  /// The id of the fixture used as the body of a PUT or POST request.
  #[serde(rename = "sourceId")]
  source_id: Option<String>,

  /// Extensions for sourceId
  #[serde(rename = "_sourceId")]
  _source_id: Option<Element>,

  /// The server where the request message is destined for.  Must be one of the server
  /// numbers listed in TestScript.destination section.
  destination: Option<i32>,

  /// Extensions for origin
  #[serde(rename = "_origin")]
  _origin: Option<Element>,

  /// The server where the request message originates from.  Must be one of the server
  /// numbers listed in TestScript.origin section.
  origin: Option<i32>,

  /// The mime-type to use for RESTful operation in the 'Content-Type' header.
  #[serde(rename = "contentType")]
  content_type: Option<String>,

  /// Extensions for targetId
  #[serde(rename = "_targetId")]
  _target_id: Option<Element>,

  /// Path plus parameters after [type].  Used to set parts of the request URL
  /// explicitly.
  params: Option<String>,

  /// Id of fixture used for extracting the [id],  [type], and [vid] for GET requests.
  #[serde(rename = "targetId")]
  target_id: Option<String>,

  /// Extensions for method
  #[serde(rename = "_method")]
  _method: Option<Element>,

  /// The fixture id (maybe new) to map to the request.
  #[serde(rename = "requestId")]
  request_id: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum TestScript_OperationMethod {
  #[serde(rename = "delete")]
  Delete,

  #[serde(rename = "get")]
  Get,

  #[serde(rename = "options")]
  Options,

  #[serde(rename = "patch")]
  Patch,

  #[serde(rename = "post")]
  Post,

  #[serde(rename = "put")]
  Put,

  #[serde(rename = "head")]
  Head,

}
