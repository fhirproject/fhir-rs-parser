#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::Identifier::Identifier;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DocumentReference_RelatesTo::DocumentReference_RelatesTo;
use crate::model::DocumentReference_Context::DocumentReference_Context;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::DocumentReference_Content::DocumentReference_Content;


/// A reference to a document of any kind for any purpose. Provides metadata about
/// the document so that the document can be discovered and managed. The scope of a
/// document is any seralized object with a mime-type, so includes formal patient
/// centric documents (CDA), cliical notes, scanned paper, and non-patient specific
/// documents like policy text.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReference {
  /// Human-readable description of the source document.
  description: String,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The base language in which the resource is written.
  language: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Other identifiers associated with the document, including version independent
  /// identifiers.
  identifier: Vec<Identifier>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Who or what the document is about. The document can be about a person, (patient
  /// or healthcare practitioner), a device (e.g. a machine) or even a group of
  /// subjects (such as a document about a herd of farm animals, or a set of patients
  /// that share a common exposure).
  subject: Box<Reference>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// A set of Security-Tag codes specifying the level of privacy/security of the
  /// Document. Note that DocumentReference.meta.security contains the security labels
  /// of the "reference" to the document, while DocumentReference.securityLabel
  /// contains a snapshot of the security labels on the document the reference refers
  /// to.
  #[serde(rename = "securityLabel")]
  security_label: Vec<CodeableConcept>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Document identifier as assigned by the source of the document. This identifier
  /// is specific to this version of the document. This unique identifier may be used
  /// elsewhere to identify this version of the document.
  #[serde(rename = "masterIdentifier")]
  master_identifier: Identifier,

  /// Extensions for status
  _status: Element,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Which person or organization authenticates that this document is valid.
  authenticator: Box<Reference>,

  /// The status of this document reference.
  status: DocumentReferenceStatus,

  /// The status of the underlying document.
  #[serde(rename = "docStatus")]
  doc_status: String,

  /// Identifies who is responsible for adding the information to the document.
  author: Vec<Box<Reference>>,

  /// A categorization for the type of document referenced - helps for indexing and
  /// searching. This may be implied by or derived from the code specified in the
  /// DocumentReference.type.
  category: Vec<CodeableConcept>,

  /// Extensions for language
  _language: Element,

  /// Relationships that this document has with other document references that already
  /// exist.
  #[serde(rename = "relatesTo")]
  relates_to: Vec<DocumentReference_RelatesTo>,

  /// Extensions for date
  _date: Element,

  /// Extensions for docStatus
  #[serde(rename = "_docStatus")]
  _doc_status: Element,

  /// The clinical context in which the document was prepared.
  context: DocumentReference_Context,

  /// Extensions for description
  _description: Element,

  /// When the document reference was created.
  date: i32,

  /// Identifies the organization or group who is responsible for ongoing maintenance
  /// of and access to the document.
  custodian: Box<Reference>,

  /// The document and format referenced. There may be multiple content element
  /// repetitions, each with a different format.
  content: Vec<DocumentReference_Content>,

  /// Specifies the particular kind of document referenced  (e.g. History and
  /// Physical, Discharge Summary, Progress Note). This usually equates to the purpose
  /// of making the document referenced.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentReferenceStatus {
  #[serde(rename = "current")]
  Current,

  #[serde(rename = "superseded")]
  Superseded,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}
