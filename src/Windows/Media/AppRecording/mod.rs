#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct AppRecordingContract(pub u8);
#[doc = "*Required features: `Media_AppRecording`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppRecordingManager(::windows::runtime::IInspectable);
impl AppRecordingManager {
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn GetStatus(&self) -> ::windows::runtime::Result<AppRecordingStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppRecordingStatus>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Media_AppRecording`, `Foundation`, `Storage`*"]
    pub fn StartRecordingToFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>>(&self, file: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<AppRecordingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppRecordingResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Media_AppRecording`, `Foundation`, `Storage`*"]
    pub fn RecordTimeSpanToFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>, Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>>(&self, starttime: Param0, duration: Param1, file: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<AppRecordingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), starttime.into_param().abi(), duration.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppRecordingResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_AppRecording`, `Foundation_Collections`*"]
    pub fn SupportedScreenshotMediaEncodingSubtypes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `Media_AppRecording`, `Foundation`, `Foundation_Collections`, `Storage`*"]
    pub fn SaveScreenshotToFilesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFolder>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(
        &self,
        folder: Param0,
        filenameprefix: Param1,
        option: AppRecordingSaveScreenshotOption,
        requestedformats: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<AppRecordingSaveScreenshotResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), folder.into_param().abi(), filenameprefix.into_param().abi(), option, requestedformats.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppRecordingSaveScreenshotResult>>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<AppRecordingManager> {
        Self::IAppRecordingManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppRecordingManager>(result__)
        })
    }
    pub fn IAppRecordingManagerStatics<R, F: FnOnce(&IAppRecordingManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppRecordingManager, IAppRecordingManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppRecordingManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingManager;{e7e26076-a044-48e2-a512-3094d574c7cc})");
}
unsafe impl ::windows::runtime::Interface for AppRecordingManager {
    type Vtable = IAppRecordingManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3890372726, 41028, 18658, [165, 18, 48, 148, 213, 116, 199, 204]);
}
impl ::windows::runtime::RuntimeName for AppRecordingManager {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingManager";
}
unsafe impl ::std::marker::Send for AppRecordingManager {}
unsafe impl ::std::marker::Sync for AppRecordingManager {}
#[doc = "*Required features: `Media_AppRecording`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppRecordingResult(::windows::runtime::IInspectable);
impl AppRecordingResult {
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn Succeeded(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_AppRecording`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn IsFileTruncated(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppRecordingResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingResult;{3a900864-c66d-46f9-b2d9-5bc2dad070d7})");
}
unsafe impl ::windows::runtime::Interface for AppRecordingResult {
    type Vtable = IAppRecordingResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(982517860, 50797, 18169, [178, 217, 91, 194, 218, 208, 112, 215]);
}
impl ::windows::runtime::RuntimeName for AppRecordingResult {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingResult";
}
unsafe impl ::std::marker::Send for AppRecordingResult {}
unsafe impl ::std::marker::Sync for AppRecordingResult {}
#[doc = "*Required features: `Media_AppRecording`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppRecordingSaveScreenshotOption(pub i32);
impl AppRecordingSaveScreenshotOption {
    pub const None: AppRecordingSaveScreenshotOption = AppRecordingSaveScreenshotOption(0i32);
    pub const HdrContentVisible: AppRecordingSaveScreenshotOption = AppRecordingSaveScreenshotOption(1i32);
}
impl ::std::convert::From<i32> for AppRecordingSaveScreenshotOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppRecordingSaveScreenshotOption {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppRecordingSaveScreenshotOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.AppRecording.AppRecordingSaveScreenshotOption;i4)");
}
#[doc = "*Required features: `Media_AppRecording`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppRecordingSaveScreenshotResult(::windows::runtime::IInspectable);
impl AppRecordingSaveScreenshotResult {
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn Succeeded(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_AppRecording`, `Foundation_Collections`*"]
    pub fn SavedScreenshotInfos(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<AppRecordingSavedScreenshotInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AppRecordingSavedScreenshotInfo>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppRecordingSaveScreenshotResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingSaveScreenshotResult;{9c5b8d0a-0abb-4457-aaee-24f9c12ec778})");
}
unsafe impl ::windows::runtime::Interface for AppRecordingSaveScreenshotResult {
    type Vtable = IAppRecordingSaveScreenshotResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2623245578, 2747, 17495, [170, 238, 36, 249, 193, 46, 199, 120]);
}
impl ::windows::runtime::RuntimeName for AppRecordingSaveScreenshotResult {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingSaveScreenshotResult";
}
unsafe impl ::std::marker::Send for AppRecordingSaveScreenshotResult {}
unsafe impl ::std::marker::Sync for AppRecordingSaveScreenshotResult {}
#[doc = "*Required features: `Media_AppRecording`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppRecordingSavedScreenshotInfo(::windows::runtime::IInspectable);
impl AppRecordingSavedScreenshotInfo {
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Media_AppRecording`, `Storage`*"]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn MediaEncodingSubtype(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppRecordingSavedScreenshotInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo;{9b642d0a-189a-4d00-bf25-e1bb1249d594})");
}
unsafe impl ::windows::runtime::Interface for AppRecordingSavedScreenshotInfo {
    type Vtable = IAppRecordingSavedScreenshotInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2607033610, 6298, 19712, [191, 37, 225, 187, 18, 73, 213, 148]);
}
impl ::windows::runtime::RuntimeName for AppRecordingSavedScreenshotInfo {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo";
}
unsafe impl ::std::marker::Send for AppRecordingSavedScreenshotInfo {}
unsafe impl ::std::marker::Sync for AppRecordingSavedScreenshotInfo {}
#[doc = "*Required features: `Media_AppRecording`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppRecordingStatus(::windows::runtime::IInspectable);
impl AppRecordingStatus {
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn CanRecord(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn CanRecordTimeSpan(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_AppRecording`, `Foundation`*"]
    pub fn HistoricalBufferDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn Details(&self) -> ::windows::runtime::Result<AppRecordingStatusDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppRecordingStatusDetails>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppRecordingStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingStatus;{1d0cc82c-bc18-4b8a-a6ef-127efab3b5d9})");
}
unsafe impl ::windows::runtime::Interface for AppRecordingStatus {
    type Vtable = IAppRecordingStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(487376940, 48152, 19338, [166, 239, 18, 126, 250, 179, 181, 217]);
}
impl ::windows::runtime::RuntimeName for AppRecordingStatus {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingStatus";
}
unsafe impl ::std::marker::Send for AppRecordingStatus {}
unsafe impl ::std::marker::Sync for AppRecordingStatus {}
#[doc = "*Required features: `Media_AppRecording`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppRecordingStatusDetails(::windows::runtime::IInspectable);
impl AppRecordingStatusDetails {
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn IsAnyAppBroadcasting(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn IsCaptureResourceUnavailable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn IsGameStreamInProgress(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn IsTimeSpanRecordingDisabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn IsGpuConstrained(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn IsAppInactive(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn IsBlockedForApp(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn IsDisabledByUser(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppRecording`*"]
    pub fn IsDisabledBySystem(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppRecordingStatusDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingStatusDetails;{b538a9b0-14ed-4412-ac45-6d672c9c9949})");
}
unsafe impl ::windows::runtime::Interface for AppRecordingStatusDetails {
    type Vtable = IAppRecordingStatusDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3040389552, 5357, 17426, [172, 69, 109, 103, 44, 156, 153, 73]);
}
impl ::windows::runtime::RuntimeName for AppRecordingStatusDetails {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingStatusDetails";
}
unsafe impl ::std::marker::Send for AppRecordingStatusDetails {}
unsafe impl ::std::marker::Sync for AppRecordingStatusDetails {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppRecordingManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppRecordingManager {
    type Vtable = IAppRecordingManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3890372726, 41028, 18658, [165, 18, 48, 148, 213, 116, 199, 204]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, folder: ::windows::runtime::RawPtr, filenameprefix: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, option: AppRecordingSaveScreenshotOption, requestedformats: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppRecordingManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppRecordingManagerStatics {
    type Vtable = IAppRecordingManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1357318647, 14542, 19411, [157, 178, 231, 43, 190, 157, 225, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppRecordingResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppRecordingResult {
    type Vtable = IAppRecordingResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(982517860, 50797, 18169, [178, 217, 91, 194, 218, 208, 112, 215]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppRecordingSaveScreenshotResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppRecordingSaveScreenshotResult {
    type Vtable = IAppRecordingSaveScreenshotResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2623245578, 2747, 17495, [170, 238, 36, 249, 193, 46, 199, 120]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingSaveScreenshotResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppRecordingSavedScreenshotInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppRecordingSavedScreenshotInfo {
    type Vtable = IAppRecordingSavedScreenshotInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2607033610, 6298, 19712, [191, 37, 225, 187, 18, 73, 213, 148]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingSavedScreenshotInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppRecordingStatus(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppRecordingStatus {
    type Vtable = IAppRecordingStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(487376940, 48152, 19338, [166, 239, 18, 126, 250, 179, 181, 217]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppRecordingStatusDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppRecordingStatusDetails {
    type Vtable = IAppRecordingStatusDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3040389552, 5357, 17426, [172, 69, 109, 103, 44, 156, 153, 73]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingStatusDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
