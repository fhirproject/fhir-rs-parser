#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Element::Element;


/// An address expressed using postal conventions (as opposed to GPS or other
/// location definition formats).  This data type may be used to convey addresses
/// for use in delivering mail as well as for visiting locations which might not be
/// valid for mail delivery.  There are a variety of postal address formats defined
/// around the world.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// Specifies the entire address as it should be displayed e.g. on a postal label.
  /// This may be provided instead of or as well as the specific parts.
  text: Option<String>,

  /// Extensions for line
  #[serde(rename = "_line")]
  _line: Option<Vec<Element>>,

  /// This component contains the house number, apartment number, street name, street
  /// direction,  P.O. Box number, delivery hints, and similar address information.
  line: Option<Vec<String>>,

  /// Extensions for postalCode
  #[serde(rename = "_postalCode")]
  _postal_code: Option<Element>,

  /// Extensions for country
  #[serde(rename = "_country")]
  _country: Option<Element>,

  /// Time period when address was/is in use.
  period: Option<Period>,

  /// Distinguishes between physical addresses (those you can visit) and mailing
  /// addresses (e.g. PO Boxes and care-of addresses). Most addresses are both.
  #[serde(rename = "type")]
  fhir_type: Option<AddressType>,

  /// Extensions for district
  #[serde(rename = "_district")]
  _district: Option<Element>,

  /// Extensions for use
  #[serde(rename = "_use")]
  _use: Option<Element>,

  /// The name of the city, town, suburb, village or other community or delivery
  /// center.
  city: Option<String>,

  /// Extensions for state
  #[serde(rename = "_state")]
  _state: Option<Element>,

  /// The name of the administrative area (county).
  district: Option<String>,

  /// Extensions for city
  #[serde(rename = "_city")]
  _city: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The purpose of this address.
  #[serde(rename = "use")]
  fhir_use: Option<AddressUse>,

  /// Extensions for text
  #[serde(rename = "_text")]
  _text: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Country - a nation as commonly understood or generally accepted.
  country: Option<String>,

  /// A postal code designating a region defined by the postal service.
  #[serde(rename = "postalCode")]
  postal_code: Option<String>,

  /// Sub-unit of a country with limited sovereignty in a federally organized country.
  /// A code may be used if codes are in common use (e.g. US 2 letter state codes).
  state: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum AddressType {
  #[serde(rename = "postal")]
  Postal,

  #[serde(rename = "physical")]
  Physical,

  #[serde(rename = "both")]
  Both,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum AddressUse {
  #[serde(rename = "home")]
  Home,

  #[serde(rename = "work")]
  Work,

  #[serde(rename = "temp")]
  Temp,

  #[serde(rename = "old")]
  Old,

  #[serde(rename = "billing")]
  Billing,

}
