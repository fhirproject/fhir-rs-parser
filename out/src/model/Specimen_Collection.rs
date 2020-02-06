#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Duration::Duration;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::Period::Period;
use crate::model::Extension::Extension;


/// A sample to be used for analysis.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Specimen_Collection {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Person who collected the specimen.
  collector: Option<Box<Reference>>,

  /// Time when specimen was collected from subject - the physiologically relevant
  /// time.
  #[serde(rename = "collectedPeriod")]
  collected_period: Option<Period>,

  /// Time when specimen was collected from subject - the physiologically relevant
  /// time.
  #[serde(rename = "collectedDateTime")]
  collected_date_time: Option<String>,

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

  /// Extensions for collectedDateTime
  #[serde(rename = "_collectedDateTime")]
  _collected_date_time: Option<Element>,

  /// Anatomical location from which the specimen was collected (if subject is a
  /// patient). This is the target site.  This element is not used for environmental
  /// specimens.
  #[serde(rename = "bodySite")]
  body_site: Option<CodeableConcept>,

  /// Abstinence or reduction from some or all food, drink, or both, for a period of
  /// time prior to sample collection.
  #[serde(rename = "fastingStatusCodeableConcept")]
  fasting_status_codeable_concept: Option<CodeableConcept>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// A coded value specifying the technique that is used to perform the procedure.
  method: Option<CodeableConcept>,

  /// The span of time over which the collection of a specimen occurred.
  duration: Option<Duration>,

  /// The quantity of specimen collected; for instance the volume of a blood sample,
  /// or the physical measurement of an anatomic pathology sample.
  quantity: Option<Quantity>,

  /// Abstinence or reduction from some or all food, drink, or both, for a period of
  /// time prior to sample collection.
  #[serde(rename = "fastingStatusDuration")]
  fasting_status_duration: Option<Duration>,

}
