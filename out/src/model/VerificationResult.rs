#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Timing::Timing;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::VerificationResult_Attestation::VerificationResult_Attestation;
use crate::model::VerificationResult_PrimarySource::VerificationResult_PrimarySource;
use crate::model::VerificationResult_Validator::VerificationResult_Validator;
use crate::model::ResourceList::ResourceList;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;


/// Describes validation requirements, source(s), status and dates for one or more
/// elements.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResult {
  /// The validation status of the target (attested; validated; in process; requires
  /// revalidation; validation failed; revalidation failed).
  status: String,

  /// A resource that was validated.
  target: Vec<Box<Reference>>,

  /// Extensions for lastPerformed
  #[serde(rename = "_lastPerformed")]
  _last_performed: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// The result if validation fails (fatal; warning; record only; none).
  #[serde(rename = "failureAction")]
  failure_action: CodeableConcept,

  /// Extensions for status
  _status: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The primary process by which the target is validated (edit check; value set;
  /// primary source; multiple sources; standalone; in context).
  #[serde(rename = "validationProcess")]
  validation_process: Vec<CodeableConcept>,

  /// Information about the entity attesting to information.
  attestation: VerificationResult_Attestation,

  /// Information about the entity validating information.
  validator: Vec<VerificationResult_Validator>,

  /// The date/time validation was last completed (including failed validations).
  #[serde(rename = "lastPerformed")]
  last_performed: String,

  /// When the validation status was updated.
  #[serde(rename = "statusDate")]
  status_date: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Information about the primary source(s) involved in validation.
  #[serde(rename = "primarySource")]
  primary_source: Vec<VerificationResult_PrimarySource>,

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

  /// What the target is validated against (nothing; primary source; multiple
  /// sources).
  #[serde(rename = "validationType")]
  validation_type: CodeableConcept,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Frequency of revalidation.
  frequency: Timing,

  /// The date when target is next validated, if appropriate.
  #[serde(rename = "nextScheduled")]
  next_scheduled: i32,

  /// Extensions for targetLocation
  #[serde(rename = "_targetLocation")]
  _target_location: Vec<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for language
  _language: Element,

  /// Extensions for nextScheduled
  #[serde(rename = "_nextScheduled")]
  _next_scheduled: Element,

  /// The fhirpath location(s) within the resource that was validated.
  #[serde(rename = "targetLocation")]
  target_location: Vec<String>,

  /// The frequency with which the target must be validated (none; initial; periodic).
  need: CodeableConcept,

  /// Extensions for statusDate
  #[serde(rename = "_statusDate")]
  _status_date: Element,

  /// The base language in which the resource is written.
  language: String,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

}
