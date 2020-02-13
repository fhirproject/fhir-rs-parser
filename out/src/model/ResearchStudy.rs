#![allow(unused_imports, non_camel_case_types)]

use crate::model::Period::Period;
use crate::model::Meta::Meta;
use crate::model::ResourceList::ResourceList;
use crate::model::Reference::Reference;
use crate::model::Narrative::Narrative;
use crate::model::ResearchStudy_Arm::ResearchStudy_Arm;
use crate::model::Annotation::Annotation;
use crate::model::ResearchStudy_Objective::ResearchStudy_Objective;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Identifier::Identifier;
use crate::model::Element::Element;
use crate::model::ContactDetail::ContactDetail;
use serde_json::value::Value;



/// A process where a researcher or organization plans and then executes a series of
/// steps intended to increase the field of healthcare-related knowledge.  This
/// includes studies of safety, efficacy, comparative effectiveness and other
/// information about medications, devices, therapies and other interventional and
/// investigative techniques.  A ResearchStudy involves the gathering of information
/// about human or animal subjects.

#[derive(Debug)]
pub struct ResearchStudy<'a> {
  pub value: &'a Value,
}

impl ResearchStudy<'_> {
  /// Identifiers assigned to this research study by the sponsor or other systems.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Indicates a country, state or other region where the study is taking place.
  pub fn location(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("location") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
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

  /// The set of steps expected to be performed as part of the execution of the study.
  pub fn protocol(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("protocol") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A full description of how the study is being conducted.
  pub fn description(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("description") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A researcher in a study who oversees multiple aspects of the study, such as
  /// concept development, protocol writing, protocol submission for IRB approval,
  /// participant recruitment, informed consent, data collection, analysis,
  /// interpretation and presentation.
  pub fn principal_investigator(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("principalInvestigator") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// A facility in which study activities are conducted.
  pub fn site(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("site") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Codes categorizing the type of study such as investigational vs. observational,
  /// type of blinding, type of randomization, safety vs. efficacy, etc.
  pub fn category(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("category") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A goal that the study is aiming to achieve in terms of a scientific question to
  /// be answered by the analysis of data collected during the study.
  pub fn objective(&self) -> Option<Vec<ResearchStudy_Objective>> {
    if let Some(Value::Array(val)) = self.value.get("objective") {
      return Some(val.into_iter().map(|e| ResearchStudy_Objective { value: e }).collect::<Vec<_>>());
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

  /// Extensions for title
  pub fn _title(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_title") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Contact details to assist a user in learning more about or engaging with the
  /// study.
  pub fn contact(&self) -> Option<Vec<ContactDetail>> {
    if let Some(Value::Array(val)) = self.value.get("contact") {
      return Some(val.into_iter().map(|e| ContactDetail { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Key terms to aid in searching for or filtering the study.
  pub fn keyword(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("keyword") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The current state of the study.
  pub fn status(&self) -> Option<ResearchStudyStatus> {
    if let Some(Value::String(val)) = self.value.get("status") {
      return Some(ResearchStudyStatus::from_string(&val).unwrap());
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

  /// A description and/or code explaining the premature termination of the study.
  pub fn reason_stopped(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("reasonStopped") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Comments made about the study by the performer, subject or other participants.
  pub fn note(&self) -> Option<Vec<Annotation>> {
    if let Some(Value::Array(val)) = self.value.get("note") {
      return Some(val.into_iter().map(|e| Annotation { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Describes an expected sequence of events for one of the participants of a study.
  /// E.g. Exposure to drug A, wash-out, exposure to drug B, wash-out, follow-up.
  pub fn arm(&self) -> Option<Vec<ResearchStudy_Arm>> {
    if let Some(Value::Array(val)) = self.value.get("arm") {
      return Some(val.into_iter().map(|e| ResearchStudy_Arm { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Identifies the start date and the expected (or actual, depending on status) end
  /// date for the study.
  pub fn period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("period") {
      return Some(Period { value: val });
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

  /// An organization that initiates the investigation and is legally responsible for
  /// the study.
  pub fn sponsor(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("sponsor") {
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

  /// Citations, references and other related documents.
  pub fn related_artifact(&self) -> Option<Vec<RelatedArtifact>> {
    if let Some(Value::Array(val)) = self.value.get("relatedArtifact") {
      return Some(val.into_iter().map(|e| RelatedArtifact { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The medication(s), food(s), therapy(ies), device(s) or other concerns or
  /// interventions that the study is seeking to gain more information about.
  pub fn focus(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("focus") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A larger research study of which this particular study is a component or step.
  pub fn part_of(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("partOf") {
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

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  pub fn meta(&self) -> Option<Meta> {
    if let Some(val) = self.value.get("meta") {
      return Some(Meta { value: val });
    }
    return None;
  }

  /// The stage in the progression of a therapy from initial experimental use in
  /// humans in clinical trials to post-market evaluation.
  pub fn phase(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("phase") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// A short, descriptive user-friendly label for the study.
  pub fn title(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("title") {
      return Some(string.to_string());
    }
    return None;
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

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The condition that is the focus of the study.  For example, In a study to
  /// examine risk factors for Lupus, might have as an inclusion criterion "healthy
  /// volunteer", but the target condition code would be a Lupus SNOMED code.
  pub fn condition(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("condition") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
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

  /// Reference to a Group that defines the criteria for and quantity of subjects
  /// participating in the study.  E.g. " 200 female Europeans between the ages of 20
  /// and 45 with early onset diabetes".
  pub fn enrollment(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("enrollment") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The type of study based upon the intent of the study's activities. A
  /// classification of the intent of the study.
  pub fn primary_purpose_type(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("primaryPurposeType") {
      return Some(CodeableConcept { value: val });
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

}

#[derive(Debug)]
pub enum ResearchStudyStatus {
  Active,
  AdministrativelyCompleted,
  Approved,
  ClosedToAccrual,
  ClosedToAccrualAndIntervention,
  Completed,
  Disapproved,
  InReview,
  TemporarilyClosedToAccrual,
  TemporarilyClosedToAccrualAndIntervention,
  Withdrawn,
}

impl ResearchStudyStatus {
    pub fn from_string(string: &str) -> Option<ResearchStudyStatus> {
      match string {
        "active" => Some(ResearchStudyStatus::Active),
        "administratively-completed" => Some(ResearchStudyStatus::AdministrativelyCompleted),
        "approved" => Some(ResearchStudyStatus::Approved),
        "closed-to-accrual" => Some(ResearchStudyStatus::ClosedToAccrual),
        "closed-to-accrual-and-intervention" => Some(ResearchStudyStatus::ClosedToAccrualAndIntervention),
        "completed" => Some(ResearchStudyStatus::Completed),
        "disapproved" => Some(ResearchStudyStatus::Disapproved),
        "in-review" => Some(ResearchStudyStatus::InReview),
        "temporarily-closed-to-accrual" => Some(ResearchStudyStatus::TemporarilyClosedToAccrual),
        "temporarily-closed-to-accrual-and-intervention" => Some(ResearchStudyStatus::TemporarilyClosedToAccrualAndIntervention),
        "withdrawn" => Some(ResearchStudyStatus::Withdrawn),
        _ => None,
    }
  }
}

