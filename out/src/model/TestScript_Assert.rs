#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScript_Assert {
  /// okay | created | noContent | notModified | bad | forbidden | notFound |
  /// methodNotAllowed | conflict | gone | preconditionFailed | unprocessable.
  response: Option<TestScript_AssertResponse>,

  /// The direction to use for the assertion.
  direction: Option<TestScript_AssertDirection>,

  /// Extensions for compareToSourceExpression
  #[serde(rename = "_compareToSourceExpression")]
  _compare_to_source_expression: Option<Element>,

  /// The FHIRPath expression to be evaluated against the request or response message
  /// contents - HTTP headers and payload.
  expression: Option<String>,

  /// Extensions for navigationLinks
  #[serde(rename = "_navigationLinks")]
  _navigation_links: Option<Element>,

  /// Extensions for requestMethod
  #[serde(rename = "_requestMethod")]
  _request_method: Option<Element>,

  /// The request method or HTTP operation code to compare against that used by the
  /// client system under test.
  #[serde(rename = "requestMethod")]
  request_method: Option<TestScript_AssertRequestMethod>,

  /// Extensions for requestURL
  #[serde(rename = "_requestURL")]
  _request_u_r_l: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for compareToSourceId
  #[serde(rename = "_compareToSourceId")]
  _compare_to_source_id: Option<Element>,

  /// Extensions for resource
  #[serde(rename = "_resource")]
  _resource: Option<Element>,

  /// Fixture to evaluate the XPath/JSONPath expression or the headerField  against.
  #[serde(rename = "sourceId")]
  source_id: Option<String>,

  /// The value to compare to.
  value: Option<String>,

  /// Extensions for contentType
  #[serde(rename = "_contentType")]
  _content_type: Option<Element>,

  /// Whether or not the test execution will produce a warning only on error for this
  /// assert.
  #[serde(rename = "warningOnly")]
  warning_only: Option<bool>,

  /// The label would be used for tracking/logging purposes by test engines.
  label: Option<String>,

  /// Extensions for compareToSourcePath
  #[serde(rename = "_compareToSourcePath")]
  _compare_to_source_path: Option<Element>,

  /// The description would be used by test engines for tracking and reporting
  /// purposes.
  description: Option<String>,

  /// The ID of a fixture.  Asserts that the response contains at a minimum the
  /// fixture specified by minimumId.
  #[serde(rename = "minimumId")]
  minimum_id: Option<String>,

  /// Extensions for response
  #[serde(rename = "_response")]
  _response: Option<Element>,

  /// The FHIRPath expression to evaluate against the source fixture. When
  /// compareToSourceId is defined, either compareToSourceExpression or
  /// compareToSourcePath must be defined, but not both.
  #[serde(rename = "compareToSourceExpression")]
  compare_to_source_expression: Option<String>,

  /// The mime-type contents to compare against the request or response message
  /// 'Content-Type' header.
  #[serde(rename = "contentType")]
  content_type: Option<String>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// The type of the resource.  See http://build.fhir.org/resourcelist.html.
  resource: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The HTTP header field name e.g. 'Location'.
  #[serde(rename = "headerField")]
  header_field: Option<String>,

  /// The operator type defines the conditional behavior of the assert. If not
  /// defined, the default is equals.
  operator: Option<TestScript_AssertOperator>,

  /// Extensions for operator
  #[serde(rename = "_operator")]
  _operator: Option<Element>,

  /// Extensions for warningOnly
  #[serde(rename = "_warningOnly")]
  _warning_only: Option<Element>,

  /// The value to use in a comparison against the request URL path string.
  #[serde(rename = "requestURL")]
  request_u_r_l: Option<String>,

  /// The XPath or JSONPath expression to be evaluated against the fixture
  /// representing the response received from server.
  path: Option<String>,

  /// The ID of the Profile to validate against.
  #[serde(rename = "validateProfileId")]
  validate_profile_id: Option<String>,

  /// XPath or JSONPath expression to evaluate against the source fixture. When
  /// compareToSourceId is defined, either compareToSourceExpression or
  /// compareToSourcePath must be defined, but not both.
  #[serde(rename = "compareToSourcePath")]
  compare_to_source_path: Option<String>,

  /// Extensions for expression
  #[serde(rename = "_expression")]
  _expression: Option<Element>,

  /// Extensions for minimumId
  #[serde(rename = "_minimumId")]
  _minimum_id: Option<Element>,

  /// Whether or not the test execution performs validation on the bundle navigation
  /// links.
  #[serde(rename = "navigationLinks")]
  navigation_links: Option<bool>,

  /// Extensions for label
  #[serde(rename = "_label")]
  _label: Option<Element>,

  /// Id of the source fixture used as the contents to be evaluated by either the
  /// "source/expression" or "sourceId/path" definition.
  #[serde(rename = "compareToSourceId")]
  compare_to_source_id: Option<String>,

  /// The value of the HTTP response code to be tested.
  #[serde(rename = "responseCode")]
  response_code: Option<String>,

  /// Extensions for responseCode
  #[serde(rename = "_responseCode")]
  _response_code: Option<Element>,

  /// Extensions for value
  #[serde(rename = "_value")]
  _value: Option<Element>,

  /// Extensions for direction
  #[serde(rename = "_direction")]
  _direction: Option<Element>,

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
  modifier_extension: Option<Vec<Extension>>,

  /// Extensions for sourceId
  #[serde(rename = "_sourceId")]
  _source_id: Option<Element>,

  /// Extensions for validateProfileId
  #[serde(rename = "_validateProfileId")]
  _validate_profile_id: Option<Element>,

  /// Extensions for headerField
  #[serde(rename = "_headerField")]
  _header_field: Option<Element>,

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
pub enum TestScript_AssertDirection {
  #[serde(rename = "response")]
  Response,

  #[serde(rename = "request")]
  Request,

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
