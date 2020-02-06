#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Signature::Signature;


/// Describes validation requirements, source(s), status and dates for one or more
/// elements.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResult_Attestation {
  /// Extensions for sourceIdentityCertificate
  #[serde(rename = "_sourceIdentityCertificate")]
  _source_identity_certificate: Element,

  /// A digital identity certificate associated with the attestation source.
  #[serde(rename = "sourceIdentityCertificate")]
  source_identity_certificate: String,

  /// Extensions for proxyIdentityCertificate
  #[serde(rename = "_proxyIdentityCertificate")]
  _proxy_identity_certificate: Element,

  /// The date the information was attested to.
  date: i32,

  /// When the who is asserting on behalf of another (organization or individual).
  #[serde(rename = "onBehalfOf")]
  on_behalf_of: Box<Reference>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The method by which attested information was submitted/retrieved (manual; API;
  /// Push).
  #[serde(rename = "communicationMethod")]
  communication_method: CodeableConcept,

  /// Extensions for date
  _date: Element,

  /// Signed assertion by the attestation source that they have attested to the
  /// information.
  #[serde(rename = "sourceSignature")]
  source_signature: Signature,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

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

  /// The individual or organization attesting to information.
  who: Box<Reference>,

  /// A digital identity certificate associated with the proxy entity submitting
  /// attested information on behalf of the attestation source.
  #[serde(rename = "proxyIdentityCertificate")]
  proxy_identity_certificate: String,

  /// Signed assertion by the proxy entity indicating that they have the right to
  /// submit attested information on behalf of the attestation source.
  #[serde(rename = "proxySignature")]
  proxy_signature: Signature,

}
