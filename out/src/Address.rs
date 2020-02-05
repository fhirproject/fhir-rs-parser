use serde::{Deserialize, Serialize};

/// An address expressed using postal conventions (as opposed to GPS or other
/// location definition formats).  This data type may be used to convey addresses
/// for use in delivering mail as well as for visiting locations which might not be
/// valid for mail delivery.  There are a variety of postal address formats defined
/// around the world.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Address {
  /// Specifies the entire address as it should be displayed e.g. on a postal label.
  /// This may be provided instead of or as well as the specific parts.
  text: String,

  /// The name of the city, town, suburb, village or other community or delivery
  /// center.
  city: String,

  /// Time period when address was/is in use.
  period: Period,

  /// The purpose of this address.
  use: AddressUse,

  /// Extensions for district
  _district: Element,

  /// A postal code designating a region defined by the postal service.
  #[serde(rename = "postalCode")]
  postal_code: String,

  /// Distinguishes between physical addresses (those you can visit) and mailing
  /// addresses (e.g. PO Boxes and care-of addresses). Most addresses are both.
  type: AddressType,

  /// Extensions for type
  _type: Element,

  /// Extensions for city
  _city: Element,

  /// Country - a nation as commonly understood or generally accepted.
  country: String,

  /// Extensions for use
  _use: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Sub-unit of a country with limited sovereignty in a federally organized country.
  /// A code may be used if codes are in common use (e.g. US 2 letter state codes).
  state: String,

  /// Extensions for postalCode
  #[serde(rename = "_postalCode")]
  _postal_code: Element,

  /// Extensions for country
  _country: Element,

  /// Extensions for state
  _state: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// This component contains the house number, apartment number, street name, street
  /// direction,  P.O. Box number, delivery hints, and similar address information.
  line: Vec<String>,

  /// The name of the administrative area (county).
  district: String,

  /// Extensions for line
  _line: Vec<Element>,

  /// Extensions for text
  _text: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum AddressUse {
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

#[derive(Debug, Serialize, Deserialize)]
enum AddressType {
  #[serde(rename = "postal")]
  Postal,

  #[serde(rename = "physical")]
  Physical,

  #[serde(rename = "both")]
  Both,

}
