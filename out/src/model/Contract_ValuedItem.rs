#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Money::Money;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Identifier::Identifier;
use crate::model::Quantity::Quantity;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;


/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract_ValuedItem {
  /// Indicates the time during which this Contract ValuedItem information is
  /// effective.
  #[serde(rename = "effectiveTime")]
  effective_time: String,

  /// Extensions for payment
  _payment: Element,

  /// Specific type of Contract Valued Item that may be priced.
  #[serde(rename = "entityCodeableConcept")]
  entity_codeable_concept: CodeableConcept,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Who will receive payment.
  recipient: Box<Reference>,

  /// Extensions for points
  _points: Element,

  /// When payment is due.
  #[serde(rename = "paymentDate")]
  payment_date: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

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

  /// Expresses the product of the Contract Valued Item unitQuantity and the
  /// unitPriceAmt. For example, the formula: unit Quantity * unit Price (Cost per
  /// Point) * factor Number  * points = net Amount. Quantity, factor and points are
  /// assumed to be 1 if not supplied.
  net: Money,

  /// Extensions for paymentDate
  #[serde(rename = "_paymentDate")]
  _payment_date: Element,

  /// Specifies the units by which the Contract Valued Item is measured or counted,
  /// and quantifies the countable or measurable Contract Valued Item instances.
  quantity: Quantity,

  /// Who will make payment.
  responsible: Box<Reference>,

  /// Extensions for effectiveTime
  #[serde(rename = "_effectiveTime")]
  _effective_time: Element,

  /// Extensions for factor
  _factor: Element,

  /// An amount that expresses the weighting (based on difficulty, cost and/or
  /// resource intensiveness) associated with the Contract Valued Item delivered. The
  /// concept of Points allows for assignment of point values for a Contract Valued
  /// Item, such that a monetary amount can be assigned to each point.
  points: f32,

  /// Identifies a Contract Valued Item instance.
  identifier: Identifier,

  /// Id  of the clause or question text related to the context of this valuedItem in
  /// the referenced form or QuestionnaireResponse.
  #[serde(rename = "linkId")]
  link_id: Vec<String>,

  /// A real number that represents a multiplier used in determining the overall value
  /// of the Contract Valued Item delivered. The concept of a Factor allows for a
  /// discount or surcharge multiplier to be applied to a monetary amount.
  factor: f32,

  /// Extensions for linkId
  #[serde(rename = "_linkId")]
  _link_id: Vec<Element>,

  /// Extensions for securityLabelNumber
  #[serde(rename = "_securityLabelNumber")]
  _security_label_number: Vec<Element>,

  /// Terms of valuation.
  payment: String,

  /// Specific type of Contract Valued Item that may be priced.
  #[serde(rename = "entityReference")]
  entity_reference: Box<Reference>,

  /// A set of security labels that define which terms are controlled by this
  /// condition.
  #[serde(rename = "securityLabelNumber")]
  security_label_number: Vec<u32>,

  /// A Contract Valued Item unit valuation measure.
  #[serde(rename = "unitPrice")]
  unit_price: Money,

}
