#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_ALREADYRUN: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_CHECKSTATUS: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_FULLPRESENT: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_FULL_SMARTSTART: u32 = 2048u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_LAUNCHEDFULL: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_LAUNCHEDMANUAL: u32 = 512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_LAUNCHFULL: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_LAUNCHMANUAL: u32 = 512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_MANUALPRESENT: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_MAX_ACCTNAME: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_MAX_EMAILADDR: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_MAX_EMAILNAME: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_MAX_LOGONNAME: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_MAX_PASSWORD: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_MAX_RASNAME: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_MAX_SERVERNAME: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_REGKEYCOMPLETED: &'static str = "Completed";
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_REGPATHSETTINGS: &'static str = "Software\\Microsoft\\Internet Connection Wizard";
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_USEDEFAULTS: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub const ICW_USE_SHELLNEXT: u32 = 1024u32;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard'*"]
pub type PFNCHECKCONNECTIONWIZARD = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: *mut u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_InternetConnectionWizard', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNSETSHELLNEXT = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PSTR) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
