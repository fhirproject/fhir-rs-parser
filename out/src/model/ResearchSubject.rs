#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::Identifier::Identifier;
use crate::model::Period::Period;
use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;


/// A physical entity which is the primary unit of operational and/or administrative
/// interest in a study.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchSubject {
  /// Extensions for actualArm
  #[serde(rename = "_actualArm")]
  _actual_arm: Option<Element>,

  /// Identifiers assigned to this research subject for a study.
  identifier: Option<Vec<Identifier>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The name of the arm in the study the subject is expected to follow as part of
  /// this study.
  #[serde(rename = "assignedArm")]
  assigned_arm: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The dates the subject began and ended their participation in the study.
  period: Option<Period>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The record of the person or animal who is involved in the study.
  individual: Box<Reference>,

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

  /// Extensions for assignedArm
  #[serde(rename = "_assignedArm")]
  _assigned_arm: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The current state of the subject.
  status: Option<ResearchSubjectStatus>,

  /// Reference to the study the subject is participating in.
  study: Box<Reference>,

  /// The name of the arm in the study the subject actually followed as part of this
  /// study.
  #[serde(rename = "actualArm")]
  actual_arm: Option<String>,

  /// A record of the patient's informed agreement to participate in the study.
  consent: Option<Box<Reference>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResearchSubjectStatus {
  #[serde(rename = "candidate")]
  Candidate,

  #[serde(rename = "eligible")]
  Eligible,

  #[serde(rename = "follow-up")]
  FollowUp,

  #[serde(rename = "ineligible")]
  Ineligible,

  #[serde(rename = "not-registered")]
  NotRegistered,

  #[serde(rename = "off-study")]
  OffStudy,

  #[serde(rename = "on-study")]
  OnStudy,

  #[serde(rename = "on-study-intervention")]
  OnStudyIntervention,

  #[serde(rename = "on-study-observation")]
  OnStudyObservation,

  #[serde(rename = "pending-on-study")]
  PendingOnStudy,

  #[serde(rename = "potential-candidate")]
  PotentialCandidate,

  #[serde(rename = "screening")]
  Screening,

  #[serde(rename = "withdrawn")]
  Withdrawn,

}
