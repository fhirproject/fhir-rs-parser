#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Quantity::Quantity;
use crate::model::Element::Element;
use crate::model::Duration::Duration;
use crate::model::Period::Period;


/// A sample to be used for analysis.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Specimen_Collection {
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
  modifier_extension: Vec<Extension>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Abstinence or reduction from some or all food, drink, or both, for a period of
  /// time prior to sample collection.
  #[serde(rename = "fastingStatusDuration")]
  fasting_status_duration: Duration,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Time when specimen was collected from subject - the physiologically relevant
  /// time.
  #[serde(rename = "collectedPeriod")]
  collected_period: Period,

  /// The quantity of specimen collected; for instance the volume of a blood sample,
  /// or the physical measurement of an anatomic pathology sample.
  quantity: Quantity,

  /// Person who collected the specimen.
  collector: Box<Reference>,

  /// A coded value specifying the technique that is used to perform the procedure.
  method: CodeableConcept,

  /// The span of time over which the collection of a specimen occurred.
  duration: Duration,

  /// Abstinence or reduction from some or all food, drink, or both, for a period of
  /// time prior to sample collection.
  #[serde(rename = "fastingStatusCodeableConcept")]
  fasting_status_codeable_concept: CodeableConcept,

  /// Extensions for collectedDateTime
  #[serde(rename = "_collectedDateTime")]
  _collected_date_time: Element,

  /// Anatomical location from which the specimen was collected (if subject is a
  /// patient). This is the target site.  This element is not used for environmental
  /// specimens.
  #[serde(rename = "bodySite")]
  body_site: CodeableConcept,

  /// Time when specimen was collected from subject - the physiologically relevant
  /// time.
  #[serde(rename = "collectedDateTime")]
  collected_date_time: String,

}
