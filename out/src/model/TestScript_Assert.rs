#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScript_Assert {
  /// The ID of the Profile to validate against.
  #[serde(rename = "validateProfileId")]
  validate_profile_id: String,

  /// The mime-type contents to compare against the request or response message
  /// 'Content-Type' header.
  #[serde(rename = "contentType")]
  content_type: String,

  /// okay | created | noContent | notModified | bad | forbidden | notFound |
  /// methodNotAllowed | conflict | gone | preconditionFailed | unprocessable.
  response: TestScript_AssertResponse,

  /// The FHIRPath expression to evaluate against the source fixture. When
  /// compareToSourceId is defined, either compareToSourceExpression or
  /// compareToSourcePath must be defined, but not both.
  #[serde(rename = "compareToSourceExpression")]
  compare_to_source_expression: String,

  /// The FHIRPath expression to be evaluated against the request or response message
  /// contents - HTTP headers and payload.
  expression: String,

  /// The value of the HTTP response code to be tested.
  #[serde(rename = "responseCode")]
  response_code: String,

  /// Extensions for value
  _value: Element,

  /// Whether or not the test execution will produce a warning only on error for this
  /// assert.
  #[serde(rename = "warningOnly")]
  warning_only: bool,

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

  /// Extensions for minimumId
  #[serde(rename = "_minimumId")]
  _minimum_id: Element,

  /// Extensions for compareToSourceExpression
  #[serde(rename = "_compareToSourceExpression")]
  _compare_to_source_expression: Element,

  /// Extensions for direction
  _direction: Element,

  /// The description would be used by test engines for tracking and reporting
  /// purposes.
  description: String,

  /// Extensions for navigationLinks
  #[serde(rename = "_navigationLinks")]
  _navigation_links: Element,

  /// Extensions for requestURL
  #[serde(rename = "_requestURL")]
  _request_u_r_l: Element,

  /// Extensions for resource
  _resource: Element,

  /// The XPath or JSONPath expression to be evaluated against the fixture
  /// representing the response received from server.
  path: String,

  /// Extensions for validateProfileId
  #[serde(rename = "_validateProfileId")]
  _validate_profile_id: Element,

  /// Extensions for compareToSourcePath
  #[serde(rename = "_compareToSourcePath")]
  _compare_to_source_path: Element,

  /// Extensions for headerField
  #[serde(rename = "_headerField")]
  _header_field: Element,

  /// Extensions for requestMethod
  #[serde(rename = "_requestMethod")]
  _request_method: Element,

  /// Id of the source fixture used as the contents to be evaluated by either the
  /// "source/expression" or "sourceId/path" definition.
  #[serde(rename = "compareToSourceId")]
  compare_to_source_id: String,

  /// The label would be used for tracking/logging purposes by test engines.
  label: String,

  /// The value to use in a comparison against the request URL path string.
  #[serde(rename = "requestURL")]
  request_u_r_l: String,

  /// Extensions for operator
  _operator: Element,

  /// Extensions for response
  _response: Element,

  /// Extensions for contentType
  #[serde(rename = "_contentType")]
  _content_type: Element,

  /// The type of the resource.  See http://build.fhir.org/resourcelist.html.
  resource: String,

  /// Extensions for label
  _label: Element,

  /// Extensions for sourceId
  #[serde(rename = "_sourceId")]
  _source_id: Element,

  /// Extensions for compareToSourceId
  #[serde(rename = "_compareToSourceId")]
  _compare_to_source_id: Element,

  /// The value to compare to.
  value: String,

  /// Extensions for expression
  _expression: Element,

  /// The request method or HTTP operation code to compare against that used by the
  /// client system under test.
  #[serde(rename = "requestMethod")]
  request_method: TestScript_AssertRequestMethod,

  /// Fixture to evaluate the XPath/JSONPath expression or the headerField  against.
  #[serde(rename = "sourceId")]
  source_id: String,

  /// Whether or not the test execution performs validation on the bundle navigation
  /// links.
  #[serde(rename = "navigationLinks")]
  navigation_links: bool,

  /// The HTTP header field name e.g. 'Location'.
  #[serde(rename = "headerField")]
  header_field: String,

  /// Extensions for description
  _description: Element,

  /// Extensions for path
  _path: Element,

  /// The ID of a fixture.  Asserts that the response contains at a minimum the
  /// fixture specified by minimumId.
  #[serde(rename = "minimumId")]
  minimum_id: String,

  /// Extensions for warningOnly
  #[serde(rename = "_warningOnly")]
  _warning_only: Element,

  /// XPath or JSONPath expression to evaluate against the source fixture. When
  /// compareToSourceId is defined, either compareToSourceExpression or
  /// compareToSourcePath must be defined, but not both.
  #[serde(rename = "compareToSourcePath")]
  compare_to_source_path: String,

  /// The operator type defines the conditional behavior of the assert. If not
  /// defined, the default is equals.
  operator: TestScript_AssertOperator,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The direction to use for the assertion.
  direction: TestScript_AssertDirection,

  /// Extensions for responseCode
  #[serde(rename = "_responseCode")]
  _response_code: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum TestScript_AssertResponse {
  #[serde(rename = "okay")]
  Okay,

  #[serde(rename = "created")]
  Created,

  #[serde(rename = "noContent")]
  NoContent,

  #[serde(rename = "notModified")]
  NotModified,

  #[serde(rename = "bad")]
  Bad,

  #[serde(rename = "forbidden")]
  Forbidden,

  #[serde(rename = "notFound")]
  NotFound,

  #[serde(rename = "methodNotAllowed")]
  MethodNotAllowed,

  #[serde(rename = "conflict")]
  Conflict,

  #[serde(rename = "gone")]
  Gone,

  #[serde(rename = "preconditionFailed")]
  PreconditionFailed,

  #[serde(rename = "unprocessable")]
  Unprocessable,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum TestScript_AssertRequestMethod {
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

#[derive(Debug, Serialize, Deserialize)]
pub enum TestScript_AssertOperator {
  #[serde(rename = "equals")]
  Equals,

  #[serde(rename = "notEquals")]
  NotEquals,

  #[serde(rename = "in")]
  In,

  #[serde(rename = "notIn")]
  NotIn,

  #[serde(rename = "greaterThan")]
  GreaterThan,

  #[serde(rename = "lessThan")]
  LessThan,

  #[serde(rename = "empty")]
  Empty,

  #[serde(rename = "notEmpty")]
  NotEmpty,

  #[serde(rename = "contains")]
  Contains,

  #[serde(rename = "notContains")]
  NotContains,

  #[serde(rename = "eval")]
  Eval,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum TestScript_AssertDirection {
  #[serde(rename = "response")]
  Response,

  #[serde(rename = "request")]
  Request,

}
