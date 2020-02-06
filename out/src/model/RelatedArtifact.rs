#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Attachment::Attachment;


/// Related artifacts such as additional documentation, justification, or
/// bibliographic references.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedArtifact {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A bibliographic citation for the related artifact. This text SHOULD be formatted
  /// according to an accepted citation format.
  citation: String,

  /// Extensions for url
  _url: Element,

  /// Extensions for type
  _type: Element,

  /// Extensions for label
  _label: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// A brief description of the document or knowledge resource being referenced,
  /// suitable for display to a consumer.
  display: String,

  /// A url for the artifact that can be followed to access the actual content.
  url: String,

  /// The document being referenced, represented as an attachment. This is exclusive
  /// with the resource element.
  document: Attachment,

  /// A short label that can be used to reference the citation from elsewhere in the
  /// containing artifact, such as a footnote index.
  label: String,

  /// Extensions for display
  _display: Element,

  /// The related resource, such as a library, value set, profile, or other knowledge
  /// resource.
  resource: String,

  /// Extensions for citation
  _citation: Element,

  /// The type of relationship to the related artifact.
  #[serde(rename = "type")]
  fhir_type: RelatedArtifactType,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum RelatedArtifactType {
  #[serde(rename = "documentation")]
  Documentation,

  #[serde(rename = "justification")]
  Justification,

  #[serde(rename = "citation")]
  Citation,

  #[serde(rename = "predecessor")]
  Predecessor,

  #[serde(rename = "successor")]
  Successor,

  #[serde(rename = "derived-from")]
  DerivedFrom,

  #[serde(rename = "depends-on")]
  DependsOn,

  #[serde(rename = "composed-of")]
  ComposedOf,

}
