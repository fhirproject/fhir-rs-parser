use serde::{Deserialize, Serialize};

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestScript_Operation {
  /// The HTTP method the test engine MUST use for this operation regardless of any
  /// other operation details.
  method: TestScript_OperationMethod,

  /// Extensions for requestId
  #[serde(rename = "_requestId")]
  _request_id: Element,

  /// The mime-type to use for RESTful operation in the 'Content-Type' header.
  #[serde(rename = "contentType")]
  content_type: String,

  /// Extensions for contentType
  #[serde(rename = "_contentType")]
  _content_type: Element,

  /// The server where the request message originates from.  Must be one of the server
  /// numbers listed in TestScript.origin section.
  origin: integer,

  /// The description would be used by test engines for tracking and reporting
  /// purposes.
  description: String,

  /// Extensions for description
  _description: Element,

  /// Server interaction or operation type.
  type: Coding,

  /// Extensions for resource
  _resource: Element,

  /// Extensions for method
  _method: Element,

  /// Path plus parameters after [type].  Used to set parts of the request URL
  /// explicitly.
  params: String,

  /// Extensions for params
  _params: Element,

  /// The type of the resource.  See http://build.fhir.org/resourcelist.html.
  resource: String,

  /// Extensions for label
  _label: Element,

  /// The fixture id (maybe new) to map to the response.
  #[serde(rename = "responseId")]
  response_id: id,

  /// Complete request URL.
  url: String,

  /// Extensions for accept
  _accept: Element,

  /// Header elements would be used to set HTTP headers.
  #[serde(rename = "requestHeader")]
  request_header: Vec<TestScript_RequestHeader>,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Id of fixture used for extracting the [id],  [type], and [vid] for GET requests.
  #[serde(rename = "targetId")]
  target_id: id,

  /// The mime-type to use for RESTful operation in the 'Accept' header.
  accept: String,

  /// Whether or not to implicitly send the request url in encoded format. The default
  /// is true to match the standard RESTful client behavior. Set to false when
  /// communicating with a server that does not support encoded url paths.
  #[serde(rename = "encodeRequestUrl")]
  encode_request_url: bool,

  /// Extensions for destination
  _destination: Element,

  /// The fixture id (maybe new) to map to the request.
  #[serde(rename = "requestId")]
  request_id: id,

  /// Extensions for responseId
  #[serde(rename = "_responseId")]
  _response_id: Element,

  /// Extensions for origin
  _origin: Element,

  /// Extensions for sourceId
  #[serde(rename = "_sourceId")]
  _source_id: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The server where the request message is destined for.  Must be one of the server
  /// numbers listed in TestScript.destination section.
  destination: integer,

  /// Extensions for encodeRequestUrl
  #[serde(rename = "_encodeRequestUrl")]
  _encode_request_url: Element,

  /// Extensions for targetId
  #[serde(rename = "_targetId")]
  _target_id: Element,

  /// Extensions for url
  _url: Element,

  /// The label would be used for tracking/logging purposes by test engines.
  label: String,

  /// The id of the fixture used as the body of a PUT or POST request.
  #[serde(rename = "sourceId")]
  source_id: id,

}

#[derive(Debug, Serialize, Deserialize)]
enum TestScript_OperationMethod {
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
