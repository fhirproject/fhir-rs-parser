#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use crate::model::ImagingStudy_Series::ImagingStudy_Series;
use crate::model::Coding::Coding;
use serde_json::value::Value;



/// Representation of the content produced in a DICOM imaging study. A study
/// comprises a set of series, each of which includes a set of Service-Object Pair
/// Instances (SOP Instances - images or other data) acquired or produced in a
/// common context.  A series is of only one modality (e.g. X-ray, CT, MR,
/// ultrasound), but a study may have multiple series of different modalities.

#[derive(Debug)]
pub struct ImagingStudy<'a> {
  pub value: &'a Value,
}

impl ImagingStudy<'_> {
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

  /// Number of Series in the Study. This value given may be larger than the number of
  /// series elements this Resource contains due to resource availability, security,
  /// or other factors. This element should be present if any series elements are
  /// present.
  pub fn number_of_series(&self) -> Option<u64> {
    if let Some(val) = self.value.get("numberOfSeries") {
      return Some(val.as_u64().unwrap());
    }
    return None;
  }

  /// Description of clinical condition indicating why the ImagingStudy was requested.
  pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("reasonCode") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The procedure which this ImagingStudy was part of.
  pub fn procedure_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("procedureReference") {
      return Some(Reference { value: val });
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

  /// The base language in which the resource is written.
  pub fn language(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("language") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A list of all the series.modality values that are actual acquisition modalities,
  /// i.e. those in the DICOM Context Group 29 (value set OID 1.2.840.10008.6.1.19).
  pub fn modality(&self) -> Option<Vec<Coding>> {
    if let Some(Value::Array(val)) = self.value.get("modality") {
      return Some(val.into_iter().map(|e| Coding { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for numberOfSeries
  pub fn _number_of_series(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_numberOfSeries") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The Imaging Manager description of the study. Institution-generated description
  /// or classification of the Study (component) performed.
  pub fn description(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("description") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The requesting/referring physician.
  pub fn referrer(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("referrer") {
      return Some(Reference { value: val });
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

  /// The code for the performed procedure type.
  pub fn procedure_code(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("procedureCode") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for description
  pub fn _description(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_description") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Indicates another resource whose existence justifies this Study.
  pub fn reason_reference(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("reasonReference") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for started
  pub fn _started(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_started") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Identifiers for the ImagingStudy such as DICOM Study Instance UID, and Accession
  /// Number.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The healthcare event (e.g. a patient and healthcare provider interaction) during
  /// which this ImagingStudy is made.
  pub fn encounter(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("encounter") {
      return Some(Reference { value: val });
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

  /// The principal physical location where the ImagingStudy was performed.
  pub fn location(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("location") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Extensions for numberOfInstances
  pub fn _number_of_instances(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_numberOfInstances") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The current state of the ImagingStudy.
  pub fn status(&self) -> Option<ImagingStudyStatus> {
    if let Some(Value::String(val)) = self.value.get("status") {
      return Some(ImagingStudyStatus::from_string(&val).unwrap());
    }
    return None;
  }

  /// Number of SOP Instances in Study. This value given may be larger than the number
  /// of instance elements this resource contains due to resource availability,
  /// security, or other factors. This element should be present if any instance
  /// elements are present.
  pub fn number_of_instances(&self) -> Option<u64> {
    if let Some(val) = self.value.get("numberOfInstances") {
      return Some(val.as_u64().unwrap());
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

  /// Per the recommended DICOM mapping, this element is derived from the Study
  /// Description attribute (0008,1030). Observations or findings about the imaging
  /// study should be recorded in another resource, e.g. Observation, and not in this
  /// element.
  pub fn note(&self) -> Option<Vec<Annotation>> {
    if let Some(Value::Array(val)) = self.value.get("note") {
      return Some(val.into_iter().map(|e| Annotation { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A list of the diagnostic requests that resulted in this imaging study being
  /// performed.
  pub fn based_on(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("basedOn") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The network service providing access (e.g., query, view, or retrieval) for the
  /// study. See implementation notes for information about using DICOM endpoints. A
  /// study-level endpoint applies to each series in the study, unless overridden by a
  /// series-level endpoint with the same Endpoint.connectionType.
  pub fn endpoint(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("endpoint") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Who read the study and interpreted the images or other content.
  pub fn interpreter(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("interpreter") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
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

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
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

  /// Each study has one or more series of images or other content.
  pub fn series(&self) -> Option<Vec<ImagingStudy_Series>> {
    if let Some(Value::Array(val)) = self.value.get("series") {
      return Some(val.into_iter().map(|e| ImagingStudy_Series { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Date and time the study started.
  pub fn started(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("started") {
      return Some(string.to_string());
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

  /// Extensions for status
  pub fn _status(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_status") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The subject, typically a patient, of the imaging study.
  pub fn subject(&self) -> Reference {
    Reference {
      value: &self.value["subject"],
    }
  }

}

#[derive(Debug)]
pub enum ImagingStudyStatus {
  Registered,
  Available,
  Cancelled,
  EnteredInError,
  Unknown,
}

impl ImagingStudyStatus {
    pub fn from_string(string: &str) -> Option<ImagingStudyStatus> {
      match string {
        "registered" => Some(ImagingStudyStatus::Registered),
        "available" => Some(ImagingStudyStatus::Available),
        "cancelled" => Some(ImagingStudyStatus::Cancelled),
        "entered-in-error" => Some(ImagingStudyStatus::EnteredInError),
        "unknown" => Some(ImagingStudyStatus::Unknown),
        _ => None,
    }
  }
}

