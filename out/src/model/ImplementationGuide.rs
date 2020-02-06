#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ImplementationGuide_DependsOn::ImplementationGuide_DependsOn;
use crate::model::Meta::Meta;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ImplementationGuide_Manifest::ImplementationGuide_Manifest;
use crate::model::ImplementationGuide_Global::ImplementationGuide_Global;
use crate::model::UsageContext::UsageContext;
use crate::model::Narrative::Narrative;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ResourceList::ResourceList;
use crate::model::ImplementationGuide_Definition::ImplementationGuide_Definition;
use crate::model::ContactDetail::ContactDetail;


/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuide {
  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for experimental
  #[serde(rename = "_experimental")]
  _experimental: Option<Element>,

  /// An absolute URI that is used to identify this implementation guide when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this implementation guide
  /// is (or will be) published. This URL can be the target of a canonical reference.
  /// It SHALL remain the same when the implementation guide is stored on different
  /// servers.
  url: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// A natural language name identifying the implementation guide. This name should
  /// be usable as an identifier for the module by machine processing applications
  /// such as code generation.
  name: Option<String>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Extensions for publisher
  #[serde(rename = "_publisher")]
  _publisher: Option<Element>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Another implementation guide that this implementation depends on. Typically, an
  /// implementation guide uses value sets, profiles etc.defined in other
  /// implementation guides.
  #[serde(rename = "dependsOn")]
  depends_on: Option<Vec<ImplementationGuide_DependsOn>>,

  /// The license that applies to this Implementation Guide, using an SPDX license
  /// code, or 'not-open-source'.
  license: Option<ImplementationGuideLicense>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for copyright
  #[serde(rename = "_copyright")]
  _copyright: Option<Element>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

  /// A legal or geographic region in which the implementation guide is intended to be
  /// used.
  jurisdiction: Option<Vec<CodeableConcept>>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Option<Vec<ContactDetail>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The identifier that is used to identify this version of the implementation guide
  /// when it is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the implementation guide author and is not expected
  /// to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence.
  version: Option<String>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// A copyright statement relating to the implementation guide and/or its contents.
  /// Copyright statements are generally legal restrictions on the use and publishing
  /// of the implementation guide.
  copyright: Option<String>,

  /// A set of profiles that all resources covered by this implementation guide must
  /// conform to.
  global: Option<Vec<ImplementationGuide_Global>>,

  /// The NPM package name for this Implementation Guide, used in the NPM package
  /// distribution, which is the primary mechanism by which FHIR based tooling manages
  /// IG dependencies. This value must be globally unique, and should be assigned with
  /// care.
  #[serde(rename = "packageId")]
  package_id: Option<String>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for license
  #[serde(rename = "_license")]
  _license: Option<Element>,

  /// Extensions for packageId
  #[serde(rename = "_packageId")]
  _package_id: Option<Element>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// A Boolean value to indicate that this implementation guide is authored for
  /// testing purposes (or education/evaluation/marketing) and is not intended to be
  /// used for genuine usage.
  experimental: Option<bool>,

  /// The date  (and optionally time) when the implementation guide was published. The
  /// date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the implementation guide changes.
  date: Option<String>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate implementation
  /// guide instances.
  #[serde(rename = "useContext")]
  use_context: Option<Vec<UsageContext>>,

  /// A short, descriptive, user-friendly title for the implementation guide.
  title: Option<String>,

  /// Extensions for fhirVersion
  #[serde(rename = "_fhirVersion")]
  _fhir_version: Option<Vec<Element>>,

  /// The information needed by an IG publisher tool to publish the whole
  /// implementation guide.
  definition: Option<ImplementationGuide_Definition>,

  /// The name of the organization or individual that published the implementation
  /// guide.
  publisher: Option<String>,

  /// The status of this implementation guide. Enables tracking the life-cycle of the
  /// content.
  status: Option<ImplementationGuideStatus>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// A free text natural language description of the implementation guide from a
  /// consumer's perspective.
  description: Option<String>,

  /// Information about an assembled implementation guide, created by the publication
  /// tooling.
  manifest: Option<ImplementationGuide_Manifest>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

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

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImplementationGuideLicense {
  #[serde(rename = "not-open-source")]
  NotOpenSource,

  #[serde(rename = "0BSD")]
  Fhir0bsd,

  #[serde(rename = "AAL")]
  AAL,

  #[serde(rename = "Abstyles")]
  Abstyles,

  #[serde(rename = "Adobe-2006")]
  Adobe2006,

  #[serde(rename = "Adobe-Glyph")]
  AdobeGlyph,

  #[serde(rename = "ADSL")]
  ADSL,

  #[serde(rename = "AFL-1.1")]
  Afl11,

  #[serde(rename = "AFL-1.2")]
  Afl12,

  #[serde(rename = "AFL-2.0")]
  Afl20,

  #[serde(rename = "AFL-2.1")]
  Afl21,

  #[serde(rename = "AFL-3.0")]
  Afl30,

  #[serde(rename = "Afmparse")]
  Afmparse,

  #[serde(rename = "AGPL-1.0-only")]
  Agpl10Only,

  #[serde(rename = "AGPL-1.0-or-later")]
  Agpl10OrLater,

  #[serde(rename = "AGPL-3.0-only")]
  Agpl30Only,

  #[serde(rename = "AGPL-3.0-or-later")]
  Agpl30OrLater,

  #[serde(rename = "Aladdin")]
  Aladdin,

  #[serde(rename = "AMDPLPA")]
  AMDPLPA,

  #[serde(rename = "AML")]
  AML,

  #[serde(rename = "AMPAS")]
  AMPAS,

  #[serde(rename = "ANTLR-PD")]
  AntlrPd,

  #[serde(rename = "Apache-1.0")]
  Apache10,

  #[serde(rename = "Apache-1.1")]
  Apache11,

  #[serde(rename = "Apache-2.0")]
  Apache20,

  #[serde(rename = "APAFML")]
  APAFML,

  #[serde(rename = "APL-1.0")]
  Apl10,

  #[serde(rename = "APSL-1.0")]
  Apsl10,

  #[serde(rename = "APSL-1.1")]
  Apsl11,

  #[serde(rename = "APSL-1.2")]
  Apsl12,

  #[serde(rename = "APSL-2.0")]
  Apsl20,

  #[serde(rename = "Artistic-1.0-cl8")]
  Artistic10Cl8,

  #[serde(rename = "Artistic-1.0-Perl")]
  Artistic10Perl,

  #[serde(rename = "Artistic-1.0")]
  Artistic10,

  #[serde(rename = "Artistic-2.0")]
  Artistic20,

  #[serde(rename = "Bahyph")]
  Bahyph,

  #[serde(rename = "Barr")]
  Barr,

  #[serde(rename = "Beerware")]
  Beerware,

  #[serde(rename = "BitTorrent-1.0")]
  Bittorrent10,

  #[serde(rename = "BitTorrent-1.1")]
  Bittorrent11,

  #[serde(rename = "Borceux")]
  Borceux,

  #[serde(rename = "BSD-1-Clause")]
  Bsd1Clause,

  #[serde(rename = "BSD-2-Clause-FreeBSD")]
  Bsd2ClauseFreebsd,

  #[serde(rename = "BSD-2-Clause-NetBSD")]
  Bsd2ClauseNetbsd,

  #[serde(rename = "BSD-2-Clause-Patent")]
  Bsd2ClausePatent,

  #[serde(rename = "BSD-2-Clause")]
  Bsd2Clause,

  #[serde(rename = "BSD-3-Clause-Attribution")]
  Bsd3ClauseAttribution,

  #[serde(rename = "BSD-3-Clause-Clear")]
  Bsd3ClauseClear,

  #[serde(rename = "BSD-3-Clause-LBNL")]
  Bsd3ClauseLbnl,

  #[serde(rename = "BSD-3-Clause-No-Nuclear-License-2014")]
  Bsd3ClauseNoNuclearLicense2014,

  #[serde(rename = "BSD-3-Clause-No-Nuclear-License")]
  Bsd3ClauseNoNuclearLicense,

  #[serde(rename = "BSD-3-Clause-No-Nuclear-Warranty")]
  Bsd3ClauseNoNuclearWarranty,

  #[serde(rename = "BSD-3-Clause")]
  Bsd3Clause,

  #[serde(rename = "BSD-4-Clause-UC")]
  Bsd4ClauseUc,

  #[serde(rename = "BSD-4-Clause")]
  Bsd4Clause,

  #[serde(rename = "BSD-Protection")]
  BsdProtection,

  #[serde(rename = "BSD-Source-Code")]
  BsdSourceCode,

  #[serde(rename = "BSL-1.0")]
  Bsl10,

  #[serde(rename = "bzip2-1.0.5")]
  Bzip2105,

  #[serde(rename = "bzip2-1.0.6")]
  Bzip2106,

  #[serde(rename = "Caldera")]
  Caldera,

  #[serde(rename = "CATOSL-1.1")]
  Catosl11,

  #[serde(rename = "CC-BY-1.0")]
  CcBy10,

  #[serde(rename = "CC-BY-2.0")]
  CcBy20,

  #[serde(rename = "CC-BY-2.5")]
  CcBy25,

  #[serde(rename = "CC-BY-3.0")]
  CcBy30,

  #[serde(rename = "CC-BY-4.0")]
  CcBy40,

  #[serde(rename = "CC-BY-NC-1.0")]
  CcByNc10,

  #[serde(rename = "CC-BY-NC-2.0")]
  CcByNc20,

  #[serde(rename = "CC-BY-NC-2.5")]
  CcByNc25,

  #[serde(rename = "CC-BY-NC-3.0")]
  CcByNc30,

  #[serde(rename = "CC-BY-NC-4.0")]
  CcByNc40,

  #[serde(rename = "CC-BY-NC-ND-1.0")]
  CcByNcNd10,

  #[serde(rename = "CC-BY-NC-ND-2.0")]
  CcByNcNd20,

  #[serde(rename = "CC-BY-NC-ND-2.5")]
  CcByNcNd25,

  #[serde(rename = "CC-BY-NC-ND-3.0")]
  CcByNcNd30,

  #[serde(rename = "CC-BY-NC-ND-4.0")]
  CcByNcNd40,

  #[serde(rename = "CC-BY-NC-SA-1.0")]
  CcByNcSa10,

  #[serde(rename = "CC-BY-NC-SA-2.0")]
  CcByNcSa20,

  #[serde(rename = "CC-BY-NC-SA-2.5")]
  CcByNcSa25,

  #[serde(rename = "CC-BY-NC-SA-3.0")]
  CcByNcSa30,

  #[serde(rename = "CC-BY-NC-SA-4.0")]
  CcByNcSa40,

  #[serde(rename = "CC-BY-ND-1.0")]
  CcByNd10,

  #[serde(rename = "CC-BY-ND-2.0")]
  CcByNd20,

  #[serde(rename = "CC-BY-ND-2.5")]
  CcByNd25,

  #[serde(rename = "CC-BY-ND-3.0")]
  CcByNd30,

  #[serde(rename = "CC-BY-ND-4.0")]
  CcByNd40,

  #[serde(rename = "CC-BY-SA-1.0")]
  CcBySa10,

  #[serde(rename = "CC-BY-SA-2.0")]
  CcBySa20,

  #[serde(rename = "CC-BY-SA-2.5")]
  CcBySa25,

  #[serde(rename = "CC-BY-SA-3.0")]
  CcBySa30,

  #[serde(rename = "CC-BY-SA-4.0")]
  CcBySa40,

  #[serde(rename = "CC0-1.0")]
  Cc010,

  #[serde(rename = "CDDL-1.0")]
  Cddl10,

  #[serde(rename = "CDDL-1.1")]
  Cddl11,

  #[serde(rename = "CDLA-Permissive-1.0")]
  CdlaPermissive10,

  #[serde(rename = "CDLA-Sharing-1.0")]
  CdlaSharing10,

  #[serde(rename = "CECILL-1.0")]
  Cecill10,

  #[serde(rename = "CECILL-1.1")]
  Cecill11,

  #[serde(rename = "CECILL-2.0")]
  Cecill20,

  #[serde(rename = "CECILL-2.1")]
  Cecill21,

  #[serde(rename = "CECILL-B")]
  CecillB,

  #[serde(rename = "CECILL-C")]
  CecillC,

  #[serde(rename = "ClArtistic")]
  ClArtistic,

  #[serde(rename = "CNRI-Jython")]
  CnriJython,

  #[serde(rename = "CNRI-Python-GPL-Compatible")]
  CnriPythonGplCompatible,

  #[serde(rename = "CNRI-Python")]
  CnriPython,

  #[serde(rename = "Condor-1.1")]
  Condor11,

  #[serde(rename = "CPAL-1.0")]
  Cpal10,

  #[serde(rename = "CPL-1.0")]
  Cpl10,

  #[serde(rename = "CPOL-1.02")]
  Cpol102,

  #[serde(rename = "Crossword")]
  Crossword,

  #[serde(rename = "CrystalStacker")]
  CrystalStacker,

  #[serde(rename = "CUA-OPL-1.0")]
  CuaOpl10,

  #[serde(rename = "Cube")]
  Cube,

  #[serde(rename = "curl")]
  Curl,

  #[serde(rename = "D-FSL-1.0")]
  DFsl10,

  #[serde(rename = "diffmark")]
  Diffmark,

  #[serde(rename = "DOC")]
  DOC,

  #[serde(rename = "Dotseqn")]
  Dotseqn,

  #[serde(rename = "DSDP")]
  DSDP,

  #[serde(rename = "dvipdfm")]
  Dvipdfm,

  #[serde(rename = "ECL-1.0")]
  Ecl10,

  #[serde(rename = "ECL-2.0")]
  Ecl20,

  #[serde(rename = "EFL-1.0")]
  Efl10,

  #[serde(rename = "EFL-2.0")]
  Efl20,

  #[serde(rename = "eGenix")]
  EGenix,

  #[serde(rename = "Entessa")]
  Entessa,

  #[serde(rename = "EPL-1.0")]
  Epl10,

  #[serde(rename = "EPL-2.0")]
  Epl20,

  #[serde(rename = "ErlPL-1.1")]
  Erlpl11,

  #[serde(rename = "EUDatagrid")]
  EUDatagrid,

  #[serde(rename = "EUPL-1.0")]
  Eupl10,

  #[serde(rename = "EUPL-1.1")]
  Eupl11,

  #[serde(rename = "EUPL-1.2")]
  Eupl12,

  #[serde(rename = "Eurosym")]
  Eurosym,

  #[serde(rename = "Fair")]
  Fair,

  #[serde(rename = "Frameworx-1.0")]
  Frameworx10,

  #[serde(rename = "FreeImage")]
  FreeImage,

  #[serde(rename = "FSFAP")]
  FSFAP,

  #[serde(rename = "FSFUL")]
  FSFUL,

  #[serde(rename = "FSFULLR")]
  FSFULLR,

  #[serde(rename = "FTL")]
  FTL,

  #[serde(rename = "GFDL-1.1-only")]
  Gfdl11Only,

  #[serde(rename = "GFDL-1.1-or-later")]
  Gfdl11OrLater,

  #[serde(rename = "GFDL-1.2-only")]
  Gfdl12Only,

  #[serde(rename = "GFDL-1.2-or-later")]
  Gfdl12OrLater,

  #[serde(rename = "GFDL-1.3-only")]
  Gfdl13Only,

  #[serde(rename = "GFDL-1.3-or-later")]
  Gfdl13OrLater,

  #[serde(rename = "Giftware")]
  Giftware,

  #[serde(rename = "GL2PS")]
  GL2PS,

  #[serde(rename = "Glide")]
  Glide,

  #[serde(rename = "Glulxe")]
  Glulxe,

  #[serde(rename = "gnuplot")]
  Gnuplot,

  #[serde(rename = "GPL-1.0-only")]
  Gpl10Only,

  #[serde(rename = "GPL-1.0-or-later")]
  Gpl10OrLater,

  #[serde(rename = "GPL-2.0-only")]
  Gpl20Only,

  #[serde(rename = "GPL-2.0-or-later")]
  Gpl20OrLater,

  #[serde(rename = "GPL-3.0-only")]
  Gpl30Only,

  #[serde(rename = "GPL-3.0-or-later")]
  Gpl30OrLater,

  #[serde(rename = "gSOAP-1.3b")]
  Gsoap13b,

  #[serde(rename = "HaskellReport")]
  HaskellReport,

  #[serde(rename = "HPND")]
  HPND,

  #[serde(rename = "IBM-pibs")]
  IbmPibs,

  #[serde(rename = "ICU")]
  ICU,

  #[serde(rename = "IJG")]
  IJG,

  #[serde(rename = "ImageMagick")]
  ImageMagick,

  #[serde(rename = "iMatix")]
  IMatix,

  #[serde(rename = "Imlib2")]
  Imlib2,

  #[serde(rename = "Info-ZIP")]
  InfoZip,

  #[serde(rename = "Intel-ACPI")]
  IntelAcpi,

  #[serde(rename = "Intel")]
  Intel,

  #[serde(rename = "Interbase-1.0")]
  Interbase10,

  #[serde(rename = "IPA")]
  IPA,

  #[serde(rename = "IPL-1.0")]
  Ipl10,

  #[serde(rename = "ISC")]
  ISC,

  #[serde(rename = "JasPer-2.0")]
  Jasper20,

  #[serde(rename = "JSON")]
  JSON,

  #[serde(rename = "LAL-1.2")]
  Lal12,

  #[serde(rename = "LAL-1.3")]
  Lal13,

  #[serde(rename = "Latex2e")]
  Latex2e,

  #[serde(rename = "Leptonica")]
  Leptonica,

  #[serde(rename = "LGPL-2.0-only")]
  Lgpl20Only,

  #[serde(rename = "LGPL-2.0-or-later")]
  Lgpl20OrLater,

  #[serde(rename = "LGPL-2.1-only")]
  Lgpl21Only,

  #[serde(rename = "LGPL-2.1-or-later")]
  Lgpl21OrLater,

  #[serde(rename = "LGPL-3.0-only")]
  Lgpl30Only,

  #[serde(rename = "LGPL-3.0-or-later")]
  Lgpl30OrLater,

  #[serde(rename = "LGPLLR")]
  LGPLLR,

  #[serde(rename = "Libpng")]
  Libpng,

  #[serde(rename = "libtiff")]
  Libtiff,

  #[serde(rename = "LiLiQ-P-1.1")]
  LiliqP11,

  #[serde(rename = "LiLiQ-R-1.1")]
  LiliqR11,

  #[serde(rename = "LiLiQ-Rplus-1.1")]
  LiliqRplus11,

  #[serde(rename = "Linux-OpenIB")]
  LinuxOpenib,

  #[serde(rename = "LPL-1.0")]
  Lpl10,

  #[serde(rename = "LPL-1.02")]
  Lpl102,

  #[serde(rename = "LPPL-1.0")]
  Lppl10,

  #[serde(rename = "LPPL-1.1")]
  Lppl11,

  #[serde(rename = "LPPL-1.2")]
  Lppl12,

  #[serde(rename = "LPPL-1.3a")]
  Lppl13a,

  #[serde(rename = "LPPL-1.3c")]
  Lppl13c,

  #[serde(rename = "MakeIndex")]
  MakeIndex,

  #[serde(rename = "MirOS")]
  MirOS,

  #[serde(rename = "MIT-0")]
  Mit0,

  #[serde(rename = "MIT-advertising")]
  MitAdvertising,

  #[serde(rename = "MIT-CMU")]
  MitCmu,

  #[serde(rename = "MIT-enna")]
  MitEnna,

  #[serde(rename = "MIT-feh")]
  MitFeh,

  #[serde(rename = "MIT")]
  MIT,

  #[serde(rename = "MITNFA")]
  MITNFA,

  #[serde(rename = "Motosoto")]
  Motosoto,

  #[serde(rename = "mpich2")]
  Mpich2,

  #[serde(rename = "MPL-1.0")]
  Mpl10,

  #[serde(rename = "MPL-1.1")]
  Mpl11,

  #[serde(rename = "MPL-2.0-no-copyleft-exception")]
  Mpl20NoCopyleftException,

  #[serde(rename = "MPL-2.0")]
  Mpl20,

  #[serde(rename = "MS-PL")]
  MsPl,

  #[serde(rename = "MS-RL")]
  MsRl,

  #[serde(rename = "MTLL")]
  MTLL,

  #[serde(rename = "Multics")]
  Multics,

  #[serde(rename = "Mup")]
  Mup,

  #[serde(rename = "NASA-1.3")]
  Nasa13,

  #[serde(rename = "Naumen")]
  Naumen,

  #[serde(rename = "NBPL-1.0")]
  Nbpl10,

  #[serde(rename = "NCSA")]
  NCSA,

  #[serde(rename = "Net-SNMP")]
  NetSnmp,

  #[serde(rename = "NetCDF")]
  NetCDF,

  #[serde(rename = "Newsletr")]
  Newsletr,

  #[serde(rename = "NGPL")]
  NGPL,

  #[serde(rename = "NLOD-1.0")]
  Nlod10,

  #[serde(rename = "NLPL")]
  NLPL,

  #[serde(rename = "Nokia")]
  Nokia,

  #[serde(rename = "NOSL")]
  NOSL,

  #[serde(rename = "Noweb")]
  Noweb,

  #[serde(rename = "NPL-1.0")]
  Npl10,

  #[serde(rename = "NPL-1.1")]
  Npl11,

  #[serde(rename = "NPOSL-3.0")]
  Nposl30,

  #[serde(rename = "NRL")]
  NRL,

  #[serde(rename = "NTP")]
  NTP,

  #[serde(rename = "OCCT-PL")]
  OcctPl,

  #[serde(rename = "OCLC-2.0")]
  Oclc20,

  #[serde(rename = "ODbL-1.0")]
  Odbl10,

  #[serde(rename = "OFL-1.0")]
  Ofl10,

  #[serde(rename = "OFL-1.1")]
  Ofl11,

  #[serde(rename = "OGTSL")]
  OGTSL,

  #[serde(rename = "OLDAP-1.1")]
  Oldap11,

  #[serde(rename = "OLDAP-1.2")]
  Oldap12,

  #[serde(rename = "OLDAP-1.3")]
  Oldap13,

  #[serde(rename = "OLDAP-1.4")]
  Oldap14,

  #[serde(rename = "OLDAP-2.0.1")]
  Oldap201,

  #[serde(rename = "OLDAP-2.0")]
  Oldap20,

  #[serde(rename = "OLDAP-2.1")]
  Oldap21,

  #[serde(rename = "OLDAP-2.2.1")]
  Oldap221,

  #[serde(rename = "OLDAP-2.2.2")]
  Oldap222,

  #[serde(rename = "OLDAP-2.2")]
  Oldap22,

  #[serde(rename = "OLDAP-2.3")]
  Oldap23,

  #[serde(rename = "OLDAP-2.4")]
  Oldap24,

  #[serde(rename = "OLDAP-2.5")]
  Oldap25,

  #[serde(rename = "OLDAP-2.6")]
  Oldap26,

  #[serde(rename = "OLDAP-2.7")]
  Oldap27,

  #[serde(rename = "OLDAP-2.8")]
  Oldap28,

  #[serde(rename = "OML")]
  OML,

  #[serde(rename = "OpenSSL")]
  OpenSSL,

  #[serde(rename = "OPL-1.0")]
  Opl10,

  #[serde(rename = "OSET-PL-2.1")]
  OsetPl21,

  #[serde(rename = "OSL-1.0")]
  Osl10,

  #[serde(rename = "OSL-1.1")]
  Osl11,

  #[serde(rename = "OSL-2.0")]
  Osl20,

  #[serde(rename = "OSL-2.1")]
  Osl21,

  #[serde(rename = "OSL-3.0")]
  Osl30,

  #[serde(rename = "PDDL-1.0")]
  Pddl10,

  #[serde(rename = "PHP-3.0")]
  Php30,

  #[serde(rename = "PHP-3.01")]
  Php301,

  #[serde(rename = "Plexus")]
  Plexus,

  #[serde(rename = "PostgreSQL")]
  PostgreSQL,

  #[serde(rename = "psfrag")]
  Psfrag,

  #[serde(rename = "psutils")]
  Psutils,

  #[serde(rename = "Python-2.0")]
  Python20,

  #[serde(rename = "Qhull")]
  Qhull,

  #[serde(rename = "QPL-1.0")]
  Qpl10,

  #[serde(rename = "Rdisc")]
  Rdisc,

  #[serde(rename = "RHeCos-1.1")]
  Rhecos11,

  #[serde(rename = "RPL-1.1")]
  Rpl11,

  #[serde(rename = "RPL-1.5")]
  Rpl15,

  #[serde(rename = "RPSL-1.0")]
  Rpsl10,

  #[serde(rename = "RSA-MD")]
  RsaMd,

  #[serde(rename = "RSCPL")]
  RSCPL,

  #[serde(rename = "Ruby")]
  Ruby,

  #[serde(rename = "SAX-PD")]
  SaxPd,

  #[serde(rename = "Saxpath")]
  Saxpath,

  #[serde(rename = "SCEA")]
  SCEA,

  #[serde(rename = "Sendmail")]
  Sendmail,

  #[serde(rename = "SGI-B-1.0")]
  SgiB10,

  #[serde(rename = "SGI-B-1.1")]
  SgiB11,

  #[serde(rename = "SGI-B-2.0")]
  SgiB20,

  #[serde(rename = "SimPL-2.0")]
  Simpl20,

  #[serde(rename = "SISSL-1.2")]
  Sissl12,

  #[serde(rename = "SISSL")]
  SISSL,

  #[serde(rename = "Sleepycat")]
  Sleepycat,

  #[serde(rename = "SMLNJ")]
  SMLNJ,

  #[serde(rename = "SMPPL")]
  SMPPL,

  #[serde(rename = "SNIA")]
  SNIA,

  #[serde(rename = "Spencer-86")]
  Spencer86,

  #[serde(rename = "Spencer-94")]
  Spencer94,

  #[serde(rename = "Spencer-99")]
  Spencer99,

  #[serde(rename = "SPL-1.0")]
  Spl10,

  #[serde(rename = "SugarCRM-1.1.3")]
  Sugarcrm113,

  #[serde(rename = "SWL")]
  SWL,

  #[serde(rename = "TCL")]
  TCL,

  #[serde(rename = "TCP-wrappers")]
  TcpWrappers,

  #[serde(rename = "TMate")]
  TMate,

  #[serde(rename = "TORQUE-1.1")]
  Torque11,

  #[serde(rename = "TOSL")]
  TOSL,

  #[serde(rename = "Unicode-DFS-2015")]
  UnicodeDfs2015,

  #[serde(rename = "Unicode-DFS-2016")]
  UnicodeDfs2016,

  #[serde(rename = "Unicode-TOU")]
  UnicodeTou,

  #[serde(rename = "Unlicense")]
  Unlicense,

  #[serde(rename = "UPL-1.0")]
  Upl10,

  #[serde(rename = "Vim")]
  Vim,

  #[serde(rename = "VOSTROM")]
  VOSTROM,

  #[serde(rename = "VSL-1.0")]
  Vsl10,

  #[serde(rename = "W3C-19980720")]
  W3c19980720,

  #[serde(rename = "W3C-20150513")]
  W3c20150513,

  #[serde(rename = "W3C")]
  W3C,

  #[serde(rename = "Watcom-1.0")]
  Watcom10,

  #[serde(rename = "Wsuipa")]
  Wsuipa,

  #[serde(rename = "WTFPL")]
  WTFPL,

  #[serde(rename = "X11")]
  X11,

  #[serde(rename = "Xerox")]
  Xerox,

  #[serde(rename = "XFree86-1.1")]
  Xfree8611,

  #[serde(rename = "xinetd")]
  Xinetd,

  #[serde(rename = "Xnet")]
  Xnet,

  #[serde(rename = "xpp")]
  Xpp,

  #[serde(rename = "XSkat")]
  XSkat,

  #[serde(rename = "YPL-1.0")]
  Ypl10,

  #[serde(rename = "YPL-1.1")]
  Ypl11,

  #[serde(rename = "Zed")]
  Zed,

  #[serde(rename = "Zend-2.0")]
  Zend20,

  #[serde(rename = "Zimbra-1.3")]
  Zimbra13,

  #[serde(rename = "Zimbra-1.4")]
  Zimbra14,

  #[serde(rename = "zlib-acknowledgement")]
  ZlibAcknowledgement,

  #[serde(rename = "Zlib")]
  Zlib,

  #[serde(rename = "ZPL-1.1")]
  Zpl11,

  #[serde(rename = "ZPL-2.0")]
  Zpl20,

  #[serde(rename = "ZPL-2.1")]
  Zpl21,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImplementationGuideStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
