use serde::{Deserialize, Serialize};

/// The regulatory authorization of a medicinal product.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicinalProductAuthorization_Procedure {
  /// Type of procedure.
  type: CodeableConcept,

  /// Date of procedure.
  #[serde(rename = "dateDateTime")]
  date_date_time: String,

  /// Date of procedure.
  #[serde(rename = "datePeriod")]
  date_period: Period,

  /// Extensions for dateDateTime
  #[serde(rename = "_dateDateTime")]
  _date_date_time: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Identifier for this procedure.
  identifier: Identifier,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Applcations submitted to obtain a marketing authorization.
  application: Vec<MedicinalProductAuthorization_Procedure>,

}
