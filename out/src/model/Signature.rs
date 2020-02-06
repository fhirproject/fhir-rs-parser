#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Coding::Coding;
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A signature along with supporting context. The signature may be a digital
/// signature that is cryptographic in nature, or some other signature acceptable to
/// the domain. This other signature may be as simple as a graphical image
/// representing a hand-written signature, or a signature ceremony Different
/// signature approaches have different utilities.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
  /// An indication of the reason that the entity signed this document. This may be
  /// explicitly included as part of the signature information and can be used when
  /// determining accountability for various actions concerning the document.
  #[serde(rename = "type")]
  fhir_type: Vec<Coding>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// A reference to an application-usable description of the identity that is
  /// represented by the signature.
  #[serde(rename = "onBehalfOf")]
  on_behalf_of: Option<Box<Reference>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for when
  #[serde(rename = "_when")]
  _when: Option<Element>,

  /// Extensions for data
  #[serde(rename = "_data")]
  _data: Option<Element>,

  /// A mime type that indicates the technical format of the signature. Important mime
  /// types are application/signature+xml for X ML DigSig, application/jose for JWS,
  /// and image/* for a graphical image of a signature, etc.
  #[serde(rename = "sigFormat")]
  sig_format: Option<String>,

  /// The base64 encoding of the Signature content. When signature is not recorded
  /// electronically this element would be empty.
  data: Option<String>,

  /// Extensions for sigFormat
  #[serde(rename = "_sigFormat")]
  _sig_format: Option<Element>,

  /// When the digital signature was signed.
  when: Option<String>,

  /// A reference to an application-usable description of the identity that signed
  /// (e.g. the signature used their private key).
  who: Box<Reference>,

  /// A mime type that indicates the technical format of the target resources signed
  /// by the signature.
  #[serde(rename = "targetFormat")]
  target_format: Option<String>,

  /// Extensions for targetFormat
  #[serde(rename = "_targetFormat")]
  _target_format: Option<Element>,

}
