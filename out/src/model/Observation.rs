#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Period::Period;
use crate::model::Identifier::Identifier;
use crate::model::Range::Range;
use crate::model::Ratio::Ratio;
use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Timing::Timing;
use crate::model::Reference::Reference;
use crate::model::SampledData::SampledData;
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::Element::Element;
use crate::model::Observation_ReferenceRange::Observation_ReferenceRange;
use crate::model::Narrative::Narrative;
use crate::model::Observation_Component::Observation_Component;
use crate::model::Quantity::Quantity;
use crate::model::Extension::Extension;


/// Measurements and simple assertions made about a patient, device or other
/// subject.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
  /// The actual focus of an observation when it is not the patient of record
  /// representing something or someone associated with the patient such as a spouse,
  /// parent, fetus, or donor. For example, fetus observations in a mother's record.
  /// The focus of an observation could also be an existing condition,  an
  /// intervention, the subject's diet,  another observation of the subject,  or a
  /// body structure such as tumor or implanted device.   An example use case would be
  /// using the Observation resource to capture whether the mother is trained to
  /// change her child's tracheostomy tube. In this example, the child is the patient
  /// of record and the mother is the focus.
  focus: Option<Vec<Box<Reference>>>,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueDateTime")]
  value_date_time: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Describes what was observed. Sometimes this is called the observation "name".
  code: CodeableConcept,

  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Option<Element>,

  /// Indicates the site on the subject's body where the observation was made (i.e.
  /// the target site).
  #[serde(rename = "bodySite")]
  body_site: Option<CodeableConcept>,

  /// A larger event of which this particular Observation is a component or step.  For
  /// example,  an observation as part of a procedure.
  #[serde(rename = "partOf")]
  part_of: Option<Vec<Box<Reference>>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// This observation is a group observation (e.g. a battery, a panel of tests, a set
  /// of vital sign measurements) that includes the target as a member of the group.
  #[serde(rename = "hasMember")]
  has_member: Option<Vec<Box<Reference>>>,

  /// Some observations have multiple component observations.  These component
  /// observations are expressed as separate code value pairs that share the same
  /// attributes.  Examples include systolic and diastolic component observations for
  /// blood pressure measurement and multiple component observations for genetics
  /// observations.
  component: Option<Vec<Observation_Component>>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueTime")]
  value_time: Option<String>,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueInteger")]
  value_integer: Option<i32>,

  /// The time or time-period the observed value is asserted as being true. For
  /// biological subjects - e.g. human patients - this is usually called the
  /// "physiologically relevant time". This is usually either the time of the
  /// procedure or of specimen collection, but very often the source of the date/time
  /// is not known, only the date/time itself.
  #[serde(rename = "effectiveInstant")]
  effective_instant: Option<String>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The device used to generate the observation data.
  device: Option<Box<Reference>>,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueRatio")]
  value_ratio: Option<Ratio>,

  /// The specimen that was used when this observation was made.
  specimen: Option<Box<Reference>>,

  /// The date and time this version of the observation was made available to
  /// providers, typically after the results have been reviewed and verified.
  issued: Option<String>,

  /// A categorical assessment of an observation value.  For example, high, low,
  /// normal.
  interpretation: Option<Vec<CodeableConcept>>,

  /// The time or time-period the observed value is asserted as being true. For
  /// biological subjects - e.g. human patients - this is usually called the
  /// "physiologically relevant time". This is usually either the time of the
  /// procedure or of specimen collection, but very often the source of the date/time
  /// is not known, only the date/time itself.
  #[serde(rename = "effectivePeriod")]
  effective_period: Option<Period>,

  /// Extensions for effectiveDateTime
  #[serde(rename = "_effectiveDateTime")]
  _effective_date_time: Option<Element>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for issued
  #[serde(rename = "_issued")]
  _issued: Option<Element>,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Option<Element>,

  /// Indicates the mechanism used to perform the observation.
  method: Option<CodeableConcept>,

  /// A plan, proposal or order that is fulfilled in whole or in part by this event.
  /// For example, a MedicationRequest may require a patient to have laboratory test
  /// performed before  it is dispensed.
  #[serde(rename = "basedOn")]
  based_on: Option<Vec<Box<Reference>>>,

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

  /// A unique identifier assigned to this observation.
  identifier: Option<Vec<Identifier>>,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valuePeriod")]
  value_period: Option<Period>,

  /// The target resource that represents a measurement from which this observation
  /// value is derived. For example, a calculated anion gap or a fetal measurement
  /// based on an ultrasound image.
  #[serde(rename = "derivedFrom")]
  derived_from: Option<Vec<Box<Reference>>>,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Option<Element>,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueRange")]
  value_range: Option<Range>,

  /// A code that classifies the general type of observation being made.
  category: Option<Vec<CodeableConcept>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The time or time-period the observed value is asserted as being true. For
  /// biological subjects - e.g. human patients - this is usually called the
  /// "physiologically relevant time". This is usually either the time of the
  /// procedure or of specimen collection, but very often the source of the date/time
  /// is not known, only the date/time itself.
  #[serde(rename = "effectiveDateTime")]
  effective_date_time: Option<String>,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueBoolean")]
  value_boolean: Option<bool>,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueQuantity")]
  value_quantity: Option<Quantity>,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueCodeableConcept")]
  value_codeable_concept: Option<CodeableConcept>,

  /// Provides a reason why the expected value in the element Observation.value[x] is
  /// missing.
  #[serde(rename = "dataAbsentReason")]
  data_absent_reason: Option<CodeableConcept>,

  /// The healthcare event  (e.g. a patient and healthcare provider interaction)
  /// during which this observation is made.
  encounter: Option<Box<Reference>>,

  /// Comments about the observation or the results.
  note: Option<Vec<Annotation>>,

  /// The time or time-period the observed value is asserted as being true. For
  /// biological subjects - e.g. human patients - this is usually called the
  /// "physiologically relevant time". This is usually either the time of the
  /// procedure or of specimen collection, but very often the source of the date/time
  /// is not known, only the date/time itself.
  #[serde(rename = "effectiveTiming")]
  effective_timing: Option<Timing>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The status of the result value.
  status: Option<ObservationStatus>,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Option<Element>,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Option<Element>,

  /// Extensions for effectiveInstant
  #[serde(rename = "_effectiveInstant")]
  _effective_instant: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueSampledData")]
  value_sampled_data: Option<SampledData>,

  /// Who was responsible for asserting the observed value as "true".
  performer: Option<Vec<Box<Reference>>>,

  /// Guidance on how to interpret the value by comparison to a normal or recommended
  /// range.  Multiple reference ranges are interpreted as an "OR".   In other words,
  /// to represent two distinct target populations, two `referenceRange` elements
  /// would be used.
  #[serde(rename = "referenceRange")]
  reference_range: Option<Vec<Observation_ReferenceRange>>,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueString")]
  value_string: Option<String>,

  /// The patient, or group of patients, location, or device this observation is about
  /// and into whose record the observation is placed. If the actual focus of the
  /// observation is different from the subject (or a sample of, part, or region of
  /// the subject), the `focus` element or the `code` itself specifies the actual
  /// focus of the observation.
  subject: Option<Box<Reference>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ObservationStatus {
  #[serde(rename = "registered")]
  Registered,

  #[serde(rename = "preliminary")]
  Preliminary,

  #[serde(rename = "final")]
  Final,

  #[serde(rename = "amended")]
  Amended,

  #[serde(rename = "corrected")]
  Corrected,

  #[serde(rename = "cancelled")]
  Cancelled,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "unknown")]
  Unknown,

}
