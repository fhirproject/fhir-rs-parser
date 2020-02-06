#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Procedure_Performer::Procedure_Performer;
use crate::model::Reference::Reference;
use crate::model::Procedure_FocalDevice::Procedure_FocalDevice;
use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ResourceList::ResourceList;
use crate::model::Period::Period;
use crate::model::Meta::Meta;
use crate::model::Identifier::Identifier;
use crate::model::Extension::Extension;
use crate::model::Range::Range;
use crate::model::Narrative::Narrative;
use crate::model::Age::Age;
use crate::model::Element::Element;


/// An action that is or was performed on or for a patient. This can be a physical
/// intervention like an operation, or less invasive like long term services,
/// counseling, or hypnotherapy.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedure {
  /// Any complications that occurred during the procedure, or in the immediate post-
  /// performance period. These are generally tracked separately from the notes,
  /// which will typically describe the procedure itself rather than any 'post
  /// procedure' issues.
  complication: Option<Vec<CodeableConcept>>,

  /// The URL pointing to a FHIR-defined protocol, guideline, order set or other
  /// definition that is adhered to in whole or in part by this Procedure.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: Option<Vec<String>>,

  /// A reference to a resource that contains details of the request for this
  /// procedure.
  #[serde(rename = "basedOn")]
  based_on: Option<Vec<Box<Reference>>>,

  /// A code specifying the state of the procedure. Generally, this will be the in-
  /// progress or completed state.
  status: Option<String>,

  /// Extensions for performedDateTime
  #[serde(rename = "_performedDateTime")]
  _performed_date_time: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Estimated or actual date, date-time, period, or age when the procedure was
  /// performed.  Allows a period to support complex procedures that span more than
  /// one date, and also allows for the length of the procedure to be captured.
  #[serde(rename = "performedDateTime")]
  performed_date_time: Option<String>,

  /// Estimated or actual date, date-time, period, or age when the procedure was
  /// performed.  Allows a period to support complex procedures that span more than
  /// one date, and also allows for the length of the procedure to be captured.
  #[serde(rename = "performedPeriod")]
  performed_period: Option<Period>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// A larger event of which this particular procedure is a component or step.
  #[serde(rename = "partOf")]
  part_of: Option<Vec<Box<Reference>>>,

  /// Extensions for performedString
  #[serde(rename = "_performedString")]
  _performed_string: Option<Element>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Option<Vec<Element>>,

  /// Business identifiers assigned to this procedure by the performer or other
  /// systems which remain constant as the resource is updated and is propagated from
  /// server to server.
  identifier: Option<Vec<Identifier>>,

  /// The person, animal or group on which the procedure was performed.
  subject: Box<Reference>,

  /// Estimated or actual date, date-time, period, or age when the procedure was
  /// performed.  Allows a period to support complex procedures that span more than
  /// one date, and also allows for the length of the procedure to be captured.
  #[serde(rename = "performedString")]
  performed_string: Option<String>,

  /// The location where the procedure actually happened.  E.g. a newborn at home, a
  /// tracheostomy at a restaurant.
  location: Option<Box<Reference>>,

  /// This could be a histology result, pathology report, surgical report, etc.
  report: Option<Vec<Box<Reference>>>,

  /// If the procedure required specific follow up - e.g. removal of sutures. The
  /// follow up may be represented as a simple note or could potentially be more
  /// complex, in which case the CarePlan resource can be used.
  #[serde(rename = "followUp")]
  follow_up: Option<Vec<CodeableConcept>>,

  /// A device that is implanted, removed or otherwise manipulated (calibration,
  /// battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as a
  /// focal portion of the Procedure.
  #[serde(rename = "focalDevice")]
  focal_device: Option<Vec<Procedure_FocalDevice>>,

  /// The URL pointing to an externally maintained protocol, guideline, order set or
  /// other definition that is adhered to in whole or in part by this Procedure.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: Option<Vec<String>>,

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

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Estimated or actual date, date-time, period, or age when the procedure was
  /// performed.  Allows a period to support complex procedures that span more than
  /// one date, and also allows for the length of the procedure to be captured.
  #[serde(rename = "performedRange")]
  performed_range: Option<Range>,

  /// The justification of why the procedure was performed.
  #[serde(rename = "reasonReference")]
  reason_reference: Option<Vec<Box<Reference>>>,

  /// Any complications that occurred during the procedure, or in the immediate post-
  /// performance period.
  #[serde(rename = "complicationDetail")]
  complication_detail: Option<Vec<Box<Reference>>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Estimated or actual date, date-time, period, or age when the procedure was
  /// performed.  Allows a period to support complex procedures that span more than
  /// one date, and also allows for the length of the procedure to be captured.
  #[serde(rename = "performedAge")]
  performed_age: Option<Age>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Detailed and structured anatomical location information. Multiple locations are
  /// allowed - e.g. multiple punch biopsies of a lesion.
  #[serde(rename = "bodySite")]
  body_site: Option<Vec<CodeableConcept>>,

  /// The outcome of the procedure - did it resolve the reasons for the procedure
  /// being performed?
  outcome: Option<CodeableConcept>,

  /// The Encounter during which this Procedure was created or performed or to which
  /// the creation of this record is tightly associated.
  encounter: Option<Box<Reference>>,

  /// A code that classifies the procedure for searching, sorting and display purposes
  /// (e.g. "Surgical Procedure").
  category: Option<CodeableConcept>,

  /// Identifies coded items that were used as part of the procedure.
  #[serde(rename = "usedCode")]
  used_code: Option<Vec<CodeableConcept>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Limited to "real" people rather than equipment.
  performer: Option<Vec<Procedure_Performer>>,

  /// Identifies medications, devices and any other substance used as part of the
  /// procedure.
  #[serde(rename = "usedReference")]
  used_reference: Option<Vec<Box<Reference>>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Any other notes and comments about the procedure.
  note: Option<Vec<Annotation>>,

  /// The specific procedure that is performed. Use text if the exact nature of the
  /// procedure cannot be coded (e.g. "Laparoscopic Appendectomy").
  code: Option<CodeableConcept>,

  /// Captures the reason for the current state of the procedure.
  #[serde(rename = "statusReason")]
  status_reason: Option<CodeableConcept>,

  /// The coded reason why the procedure was performed. This may be a coded entity of
  /// some type, or may simply be present as text.
  #[serde(rename = "reasonCode")]
  reason_code: Option<Vec<CodeableConcept>>,

  /// Individual who is making the procedure statement.
  asserter: Option<Box<Reference>>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Individual who recorded the record and takes responsibility for its content.
  recorder: Option<Box<Reference>>,

}
