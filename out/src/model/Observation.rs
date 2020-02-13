#![allow(unused_imports, non_camel_case_types)]

use crate::model::Range::Range;
use crate::model::Ratio::Ratio;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::Observation_ReferenceRange::Observation_ReferenceRange;
use crate::model::Extension::Extension;
use crate::model::Timing::Timing;
use crate::model::Observation_Component::Observation_Component;
use crate::model::Quantity::Quantity;
use crate::model::Identifier::Identifier;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Period::Period;
use crate::model::ResourceList::ResourceList;
use crate::model::Reference::Reference;
use crate::model::Meta::Meta;
use crate::model::SampledData::SampledData;
use crate::model::Annotation::Annotation;
use serde_json::value::Value;



/// Measurements and simple assertions made about a patient, device or other
/// subject.

#[derive(Debug)]
pub struct Observation<'a> {
  pub value: &'a Value,
}

impl Observation<'_> {
  /// The time or time-period the observed value is asserted as being true. For
  /// biological subjects - e.g. human patients - this is usually called the
  /// "physiologically relevant time". This is usually either the time of the
  /// procedure or of specimen collection, but very often the source of the date/time
  /// is not known, only the date/time itself.
  pub fn effective_timing(&self) -> Option<Timing> {
    if let Some(val) = self.value.get("effectiveTiming") {
      return Some(Timing { value: val });
    }
    return None;
  }

  /// Extensions for implicitRules
  pub fn _implicit_rules(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_implicitRules") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  pub fn extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("extension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for status
  pub fn _status(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_status") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("valueCodeableConcept") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  pub fn value_time(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("valueTime") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  pub fn value_period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("valuePeriod") {
      return Some(Period { value: val });
    }
    return None;
  }

  /// The time or time-period the observed value is asserted as being true. For
  /// biological subjects - e.g. human patients - this is usually called the
  /// "physiologically relevant time". This is usually either the time of the
  /// procedure or of specimen collection, but very often the source of the date/time
  /// is not known, only the date/time itself.
  pub fn effective_instant(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("effectiveInstant") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for valueInteger
  pub fn _value_integer(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_valueInteger") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Provides a reason why the expected value in the element Observation.value[x] is
  /// missing.
  pub fn data_absent_reason(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("dataAbsentReason") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  pub fn value_quantity(&self) -> Option<Quantity> {
    if let Some(val) = self.value.get("valueQuantity") {
      return Some(Quantity { value: val });
    }
    return None;
  }

  /// Comments about the observation or the results.
  pub fn note(&self) -> Option<Vec<Annotation>> {
    if let Some(Value::Array(val)) = self.value.get("note") {
      return Some(val.into_iter().map(|e| Annotation { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for valueDateTime
  pub fn _value_date_time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_valueDateTime") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The target resource that represents a measurement from which this observation
  /// value is derived. For example, a calculated anion gap or a fetal measurement
  /// based on an ultrasound image.
  pub fn derived_from(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("derivedFrom") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

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
  pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  pub fn value_integer(&self) -> Option<i64> {
    if let Some(val) = self.value.get("valueInteger") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// A unique identifier assigned to this observation.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  pub fn meta(&self) -> Option<Meta> {
    if let Some(val) = self.value.get("meta") {
      return Some(Meta { value: val });
    }
    return None;
  }

  /// A larger event of which this particular Observation is a component or step.  For
  /// example,  an observation as part of a procedure.
  pub fn part_of(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("partOf") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// This observation is a group observation (e.g. a battery, a panel of tests, a set
  /// of vital sign measurements) that includes the target as a member of the group.
  pub fn has_member(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("hasMember") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for effectiveInstant
  pub fn _effective_instant(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_effectiveInstant") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for valueTime
  pub fn _value_time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_valueTime") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Indicates the site on the subject's body where the observation was made (i.e.
  /// the target site).
  pub fn body_site(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("bodySite") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The patient, or group of patients, location, or device this observation is about
  /// and into whose record the observation is placed. If the actual focus of the
  /// observation is different from the subject (or a sample of, part, or region of
  /// the subject), the `focus` element or the `code` itself specifies the actual
  /// focus of the observation.
  pub fn subject(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("subject") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  pub fn value_sampled_data(&self) -> Option<SampledData> {
    if let Some(val) = self.value.get("valueSampledData") {
      return Some(SampledData { value: val });
    }
    return None;
  }

  /// Some observations have multiple component observations.  These component
  /// observations are expressed as separate code value pairs that share the same
  /// attributes.  Examples include systolic and diastolic component observations for
  /// blood pressure measurement and multiple component observations for genetics
  /// observations.
  pub fn component(&self) -> Option<Vec<Observation_Component>> {
    if let Some(Value::Array(val)) = self.value.get("component") {
      return Some(val.into_iter().map(|e| Observation_Component { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for valueBoolean
  pub fn _value_boolean(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_valueBoolean") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for issued
  pub fn _issued(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_issued") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The time or time-period the observed value is asserted as being true. For
  /// biological subjects - e.g. human patients - this is usually called the
  /// "physiologically relevant time". This is usually either the time of the
  /// procedure or of specimen collection, but very often the source of the date/time
  /// is not known, only the date/time itself.
  pub fn effective_date_time(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("effectiveDateTime") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for language
  pub fn _language(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_language") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Who was responsible for asserting the observed value as "true".
  pub fn performer(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("performer") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for effectiveDateTime
  pub fn _effective_date_time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_effectiveDateTime") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A plan, proposal or order that is fulfilled in whole or in part by this event.
  /// For example, a MedicationRequest may require a patient to have laboratory test
  /// performed before  it is dispensed.
  pub fn based_on(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("basedOn") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for valueString
  pub fn _value_string(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_valueString") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A code that classifies the general type of observation being made.
  pub fn category(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("category") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The device used to generate the observation data.
  pub fn device(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("device") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Indicates the mechanism used to perform the observation.
  pub fn method(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("method") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Guidance on how to interpret the value by comparison to a normal or recommended
  /// range.  Multiple reference ranges are interpreted as an "OR".   In other words,
  /// to represent two distinct target populations, two `referenceRange` elements
  /// would be used.
  pub fn reference_range(&self) -> Option<Vec<Observation_ReferenceRange>> {
    if let Some(Value::Array(val)) = self.value.get("referenceRange") {
      return Some(val.into_iter().map(|e| Observation_ReferenceRange { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  pub fn value_date_time(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("valueDateTime") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  pub fn value_boolean(&self) -> Option<bool> {
    if let Some(val) = self.value.get("valueBoolean") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

  /// The actual focus of an observation when it is not the patient of record
  /// representing something or someone associated with the patient such as a spouse,
  /// parent, fetus, or donor. For example, fetus observations in a mother's record.
  /// The focus of an observation could also be an existing condition,  an
  /// intervention, the subject's diet,  another observation of the subject,  or a
  /// body structure such as tumor or implanted device.   An example use case would be
  /// using the Observation resource to capture whether the mother is trained to
  /// change her child's tracheostomy tube. In this example, the child is the patient
  /// of record and the mother is the focus.
  pub fn focus(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("focus") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  pub fn value_ratio(&self) -> Option<Ratio> {
    if let Some(val) = self.value.get("valueRatio") {
      return Some(Ratio { value: val });
    }
    return None;
  }

  /// The specimen that was used when this observation was made.
  pub fn specimen(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("specimen") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The date and time this version of the observation was made available to
  /// providers, typically after the results have been reviewed and verified.
  pub fn issued(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("issued") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A categorical assessment of an observation value.  For example, high, low,
  /// normal.
  pub fn interpretation(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("interpretation") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The healthcare event  (e.g. a patient and healthcare provider interaction)
  /// during which this observation is made.
  pub fn encounter(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("encounter") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  pub fn value_range(&self) -> Option<Range> {
    if let Some(val) = self.value.get("valueRange") {
      return Some(Range { value: val });
    }
    return None;
  }

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  pub fn value_string(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("valueString") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The status of the result value.
  pub fn status(&self) -> Option<ObservationStatus> {
    if let Some(Value::String(val)) = self.value.get("status") {
      return Some(ObservationStatus::from_string(&val).unwrap());
    }
    return None;
  }

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  pub fn implicit_rules(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("implicitRules") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Describes what was observed. Sometimes this is called the observation "name".
  pub fn code(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["code"],
    }
  }

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  pub fn text(&self) -> Option<Narrative> {
    if let Some(val) = self.value.get("text") {
      return Some(Narrative { value: val });
    }
    return None;
  }

  /// The base language in which the resource is written.
  pub fn language(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("language") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The time or time-period the observed value is asserted as being true. For
  /// biological subjects - e.g. human patients - this is usually called the
  /// "physiologically relevant time". This is usually either the time of the
  /// procedure or of specimen collection, but very often the source of the date/time
  /// is not known, only the date/time itself.
  pub fn effective_period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("effectivePeriod") {
      return Some(Period { value: val });
    }
    return None;
  }

}

#[derive(Debug)]
pub enum ObservationStatus {
  Registered,
  Preliminary,
  Final,
  Amended,
  Corrected,
  Cancelled,
  EnteredInError,
  Unknown,
}

impl ObservationStatus {
    pub fn from_string(string: &str) -> Option<ObservationStatus> {
      match string {
        "registered" => Some(ObservationStatus::Registered),
        "preliminary" => Some(ObservationStatus::Preliminary),
        "final" => Some(ObservationStatus::Final),
        "amended" => Some(ObservationStatus::Amended),
        "corrected" => Some(ObservationStatus::Corrected),
        "cancelled" => Some(ObservationStatus::Cancelled),
        "entered-in-error" => Some(ObservationStatus::EnteredInError),
        "unknown" => Some(ObservationStatus::Unknown),
        _ => None,
    }
  }
}

