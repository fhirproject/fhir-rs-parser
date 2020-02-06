#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Meta::Meta;
use crate::model::Specimen_Processing::Specimen_Processing;
use crate::model::Specimen_Container::Specimen_Container;
use crate::model::Narrative::Narrative;
use crate::model::Annotation::Annotation;
use crate::model::ResourceList::ResourceList;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use crate::model::Specimen_Collection::Specimen_Collection;
use crate::model::Identifier::Identifier;


/// A sample to be used for analysis.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Specimen {
  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The kind of material that forms the specimen.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// Reference to the parent (source) specimen which is used when the specimen was
  /// either derived from or a component of another specimen.
  parent: Option<Vec<Box<Reference>>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The container holding the specimen.  The recursive nature of containers; i.e.
  /// blood in tube in tray in rack is not addressed here.
  container: Option<Vec<Specimen_Container>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Where the specimen came from. This may be from patient(s), from a location
  /// (e.g., the source of an environmental sample), or a sampling of a substance or a
  /// device.
  subject: Option<Box<Reference>>,

  /// Details concerning the specimen collection.
  collection: Option<Specimen_Collection>,

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
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// The identifier assigned by the lab when accessioning specimen(s). This is not
  /// necessarily the same as the specimen identifier, depending on local lab
  /// procedures.
  #[serde(rename = "accessionIdentifier")]
  accession_identifier: Option<Identifier>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Id for specimen.
  identifier: Option<Vec<Identifier>>,

  /// Time when specimen was received for processing or testing.
  #[serde(rename = "receivedTime")]
  received_time: Option<String>,

  /// Extensions for receivedTime
  #[serde(rename = "_receivedTime")]
  _received_time: Option<Element>,

  /// The availability of the specimen.
  status: Option<SpecimenStatus>,

  /// Details concerning processing and processing steps for the specimen.
  processing: Option<Vec<Specimen_Processing>>,

  /// A mode or state of being that describes the nature of the specimen.
  condition: Option<Vec<CodeableConcept>>,

  /// Details concerning a service request that required a specimen to be collected.
  request: Option<Vec<Box<Reference>>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// To communicate any details or issues about the specimen or during the specimen
  /// collection. (for example: broken vial, sent with patient, frozen).
  note: Option<Vec<Annotation>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpecimenStatus {
  #[serde(rename = "available")]
  Available,

  #[serde(rename = "unavailable")]
  Unavailable,

  #[serde(rename = "unsatisfactory")]
  Unsatisfactory,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}
