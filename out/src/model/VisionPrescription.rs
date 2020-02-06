#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;
use crate::model::VisionPrescription_LensSpecification::VisionPrescription_LensSpecification;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::ResourceList::ResourceList;
use crate::model::Element::Element;
use crate::model::Meta::Meta;


/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisionPrescription {
  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// A reference to a resource that identifies the particular occurrence of contact
  /// between patient and health care provider during which the prescription was
  /// issued.
  encounter: Option<Box<Reference>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for created
  #[serde(rename = "_created")]
  _created: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

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
  modifier_extension: Option<Vec<Extension>>,

  /// A unique identifier assigned to this vision prescription.
  identifier: Option<Vec<Identifier>>,

  /// The date this resource was created.
  created: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The status of the resource instance.
  status: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// A resource reference to the person to whom the vision prescription applies.
  patient: Box<Reference>,

  /// The date (and perhaps time) when the prescription was written.
  #[serde(rename = "dateWritten")]
  date_written: Option<String>,

  /// Extensions for dateWritten
  #[serde(rename = "_dateWritten")]
  _date_written: Option<Element>,

  /// Contain the details of  the individual lens specifications and serves as the
  /// authorization for the fullfillment by certified professionals.
  #[serde(rename = "lensSpecification")]
  lens_specification: Vec<VisionPrescription_LensSpecification>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The healthcare professional responsible for authorizing the prescription.
  prescriber: Box<Reference>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

}
