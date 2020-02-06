#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::MedicationKnowledge_MaxDispense::MedicationKnowledge_MaxDispense;
use crate::model::Reference::Reference;
use crate::model::MedicationKnowledge_Substitution::MedicationKnowledge_Substitution;
use crate::model::MedicationKnowledge_Schedule::MedicationKnowledge_Schedule;
use crate::model::Extension::Extension;


/// Information about a medication that is used to support knowledge.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledge_Regulatory {
  /// Specifies the schedule of a medication in jurisdiction.
  schedule: Option<Vec<MedicationKnowledge_Schedule>>,

  /// The maximum number of units of the medication that can be dispensed in a period.
  #[serde(rename = "maxDispense")]
  max_dispense: Option<MedicationKnowledge_MaxDispense>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Extension>>,

  /// The authority that is specifying the regulations.
  #[serde(rename = "regulatoryAuthority")]
  regulatory_authority: Box<Reference>,

  /// Specifies if changes are allowed when dispensing a medication from a regulatory
  /// perspective.
  substitution: Option<Vec<MedicationKnowledge_Substitution>>,

}
