#![feature(prelude_import)]
#![allow(unused_unsafe)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use lazy_static::lazy_static;
use log::*;
use std::os::raw::c_void;
use std::sync::Mutex;
use vst3_sys::base::{
    kInvalidArgument, kResultOk, tresult, FactoryFlags, IBStream, IPlugin, IPluginFactory,
    IUnknown, TBool,
};
use vst3_sys::vst::{
    BusDirection, BusDirections, BusFlags, BusInfo, IAudioPresentationLatency, IAudioProcessor,
    IAutomationState, IComponent, MediaTypes, ProcessData, ProcessSetup,
};
use vst3_sys::{REFIID, VST3};
#[repr(C)]
pub struct PassthruPlugin {
    __iaudioprocessorvptr: *const <dyn IAudioProcessor as com::ComInterface>::VTable,
    __iaudiopresentationlatencyvptr:
        *const <dyn IAudioPresentationLatency as com::ComInterface>::VTable,
    __iautomationstatevptr: *const <dyn IAutomationState as com::ComInterface>::VTable,
    __ipluginvptr: *const <dyn IPlugin as com::ComInterface>::VTable,
    __refcnt: std::cell::Cell<u32>,
}
impl PassthruPlugin {
    fn allocate() -> Box<PassthruPlugin> {
        let iaudioprocessor_vtable = <dyn IAudioProcessor as ::com::ProductionComInterface<
            PassthruPlugin,
        >>::vtable::<::com::offset::Zero>();
        let __iaudioprocessorvptr = Box::into_raw(Box::new(iaudioprocessor_vtable));
        let iaudiopresentationlatency_vtable = < dyn IAudioPresentationLatency as :: com :: ProductionComInterface < PassthruPlugin > > :: vtable :: < :: com :: offset :: One > ( ) ;
        let __iaudiopresentationlatencyvptr =
            Box::into_raw(Box::new(iaudiopresentationlatency_vtable));
        let iautomationstate_vtable = <dyn IAutomationState as ::com::ProductionComInterface<
            PassthruPlugin,
        >>::vtable::<::com::offset::Two>();
        let __iautomationstatevptr = Box::into_raw(Box::new(iautomationstate_vtable));
        let iplugin_vtable = <dyn IPlugin as ::com::ProductionComInterface<PassthruPlugin>>::vtable::<
            ::com::offset::Three,
        >();
        let __ipluginvptr = Box::into_raw(Box::new(iplugin_vtable));
        let out = PassthruPlugin {
            __iaudioprocessorvptr,
            __iaudiopresentationlatencyvptr,
            __iautomationstatevptr,
            __ipluginvptr,
            __refcnt: std::cell::Cell::new(0),
        };
        Box::new(out)
    }
    pub fn get_class_object() -> Box<PassthruPluginClassFactory> {
        <PassthruPluginClassFactory>::new()
    }
}
unsafe impl com::CoClass for PassthruPlugin {}
impl com::interfaces::IUnknown for PassthruPlugin {
    unsafe fn query_interface(
        &self,
        riid: *const com::sys::IID,
        ppv: *mut *mut std::ffi::c_void,
    ) -> com::sys::HRESULT {
        let riid = &*riid;
        if riid == &com::interfaces::iunknown::IID_IUNKNOWN {
            *ppv = &self.__iaudioprocessorvptr as *const _ as *mut std::ffi::c_void;
        } else if <dyn IAudioProcessor as com::ComInterface>::is_iid_in_inheritance_chain(riid) {
            *ppv = &self.__iaudioprocessorvptr as *const _ as *mut std::ffi::c_void;
        } else if <dyn IAudioPresentationLatency as com::ComInterface>::is_iid_in_inheritance_chain(
            riid,
        ) {
            *ppv = &self.__iaudiopresentationlatencyvptr as *const _ as *mut std::ffi::c_void;
        } else if <dyn IAutomationState as com::ComInterface>::is_iid_in_inheritance_chain(riid) {
            *ppv = &self.__iautomationstatevptr as *const _ as *mut std::ffi::c_void;
        } else if <dyn IPlugin as com::ComInterface>::is_iid_in_inheritance_chain(riid) {
            *ppv = &self.__ipluginvptr as *const _ as *mut std::ffi::c_void;
        } else {
            *ppv = std::ptr::null_mut::<std::ffi::c_void>();
            return com::sys::E_NOINTERFACE;
        }
        self.add_ref();
        com::sys::NOERROR
    }
    unsafe fn add_ref(&self) -> u32 {
        let value = self
            .__refcnt
            .get()
            .checked_add(1)
            .expect("Overflow of reference count");
        self.__refcnt.set(value);
        value
    }
    unsafe fn release(&self) -> u32 {
        let value = self
            .__refcnt
            .get()
            .checked_sub(1)
            .expect("Underflow of reference count");
        self.__refcnt.set(value);
        let __refcnt = self.__refcnt.get();
        if __refcnt == 0 {
            Box::from_raw(
                self.__iaudioprocessorvptr
                    as *mut <dyn IAudioProcessor as com::ComInterface>::VTable,
            );
            Box::from_raw(
                self.__iaudiopresentationlatencyvptr
                    as *mut <dyn IAudioPresentationLatency as com::ComInterface>::VTable,
            );
            Box::from_raw(
                self.__iautomationstatevptr
                    as *mut <dyn IAutomationState as com::ComInterface>::VTable,
            );
            Box::from_raw(self.__ipluginvptr as *mut <dyn IPlugin as com::ComInterface>::VTable);
            Box::from_raw(self as *const _ as *mut PassthruPlugin);
        }
        __refcnt
    }
}
#[repr(C)]
pub struct PassthruPluginClassFactory {
    __iclassfactoryvptr:
        *const <dyn com::interfaces::iclass_factory::IClassFactory as com::ComInterface>::VTable,
    __refcnt: std::cell::Cell<u32>,
}
impl com::interfaces::IClassFactory for PassthruPluginClassFactory {
    unsafe fn create_instance(
        &self,
        aggr: *mut *const <dyn com::interfaces::iunknown::IUnknown as com::ComInterface>::VTable,
        riid: *const com::sys::IID,
        ppv: *mut *mut std::ffi::c_void,
    ) -> com::sys::HRESULT {
        use com::interfaces::iunknown::IUnknown;
        if aggr != std::ptr::null_mut() {
            return com::sys::CLASS_E_NOAGGREGATION;
        }
        let mut instance = PassthruPlugin::new();
        instance.add_ref();
        let hr = instance.query_interface(riid, ppv);
        instance.release();
        core::mem::forget(instance);
        hr
    }
    unsafe fn lock_server(&self, _increment: com::sys::BOOL) -> com::sys::HRESULT {
        com::sys::S_OK
    }
}
impl com::interfaces::IUnknown for PassthruPluginClassFactory {
    unsafe fn query_interface(
        &self,
        riid: *const com::sys::IID,
        ppv: *mut *mut std::ffi::c_void,
    ) -> com::sys::HRESULT {
        use com::interfaces::iunknown::IUnknown;
        let riid = &*riid;
        if riid == &<dyn com::interfaces::iunknown::IUnknown as com::ComInterface>::IID
            || riid
                == &<dyn com::interfaces::iclass_factory::IClassFactory as com::ComInterface>::IID
        {
            *ppv = &self.__iclassfactoryvptr as *const _ as *mut std::ffi::c_void;
            self.add_ref();
            com::sys::NOERROR
        } else {
            *ppv = std::ptr::null_mut::<std::ffi::c_void>();
            com::sys::E_NOINTERFACE
        }
    }
    unsafe fn add_ref(&self) -> u32 {
        let value = self
            .__refcnt
            .get()
            .checked_add(1)
            .expect("Overflow of reference count");
        self.__refcnt.set(value);
        value
    }
    unsafe fn release(&self) -> u32 {
        use com::interfaces::iclass_factory::IClassFactory;
        let value = self
            .__refcnt
            .get()
            .checked_sub(1)
            .expect("Underflow of reference count");
        self.__refcnt.set(value);
        let __refcnt = self.__refcnt.get();
        if __refcnt == 0 {
            Box::from_raw(
                self.__iclassfactoryvptr as *mut <dyn IClassFactory as com::ComInterface>::VTable,
            );
            Box::from_raw(self as *const _ as *mut PassthruPluginClassFactory);
        }
        __refcnt
    }
}
impl PassthruPluginClassFactory {
    pub(crate) fn new() -> Box<PassthruPluginClassFactory> {
        use com::interfaces::iclass_factory::IClassFactory;
        let iclassfactory_vtable = <dyn IClassFactory as ::com::ProductionComInterface<
            PassthruPluginClassFactory,
        >>::vtable::<::com::offset::Zero>();
        let __iclassfactoryvptr = Box::into_raw(Box::new(iclassfactory_vtable));
        let out = PassthruPluginClassFactory {
            __iclassfactoryvptr,
            __refcnt: std::cell::Cell::new(0),
        };
        Box::new(out)
    }
}
pub struct PassthruController {}
impl PassthruPlugin {
    pub fn new() -> Box<Self> {
        PassthruPlugin::allocate()
    }
}
#[repr(C)]
pub struct Factory {
    __ipluginfactoryvptr: *const <dyn IPluginFactory as com::ComInterface>::VTable,
    __refcnt: std::cell::Cell<u32>,
}
impl Factory {
    fn allocate() -> Box<Factory> {
        let ipluginfactory_vtable = <dyn IPluginFactory as ::com::ProductionComInterface<
            Factory,
        >>::vtable::<::com::offset::Zero>();
        let __ipluginfactoryvptr = Box::into_raw(Box::new(ipluginfactory_vtable));
        let out = Factory {
            __ipluginfactoryvptr,
            __refcnt: std::cell::Cell::new(0),
        };
        Box::new(out)
    }
    pub fn get_class_object() -> Box<FactoryClassFactory> {
        <FactoryClassFactory>::new()
    }
}
unsafe impl com::CoClass for Factory {}
impl com::interfaces::IUnknown for Factory {
    unsafe fn query_interface(
        &self,
        riid: *const com::sys::IID,
        ppv: *mut *mut std::ffi::c_void,
    ) -> com::sys::HRESULT {
        let riid = &*riid;
        if riid == &com::interfaces::iunknown::IID_IUNKNOWN {
            *ppv = &self.__ipluginfactoryvptr as *const _ as *mut std::ffi::c_void;
        } else if <dyn IPluginFactory as com::ComInterface>::is_iid_in_inheritance_chain(riid) {
            *ppv = &self.__ipluginfactoryvptr as *const _ as *mut std::ffi::c_void;
        } else {
            *ppv = std::ptr::null_mut::<std::ffi::c_void>();
            return com::sys::E_NOINTERFACE;
        }
        self.add_ref();
        com::sys::NOERROR
    }
    unsafe fn add_ref(&self) -> u32 {
        let value = self
            .__refcnt
            .get()
            .checked_add(1)
            .expect("Overflow of reference count");
        self.__refcnt.set(value);
        value
    }
    unsafe fn release(&self) -> u32 {
        let value = self
            .__refcnt
            .get()
            .checked_sub(1)
            .expect("Underflow of reference count");
        self.__refcnt.set(value);
        let __refcnt = self.__refcnt.get();
        if __refcnt == 0 {
            Box::from_raw(
                self.__ipluginfactoryvptr as *mut <dyn IPluginFactory as com::ComInterface>::VTable,
            );
            Box::from_raw(self as *const _ as *mut Factory);
        }
        __refcnt
    }
}
#[repr(C)]
pub struct FactoryClassFactory {
    __iclassfactoryvptr:
        *const <dyn com::interfaces::iclass_factory::IClassFactory as com::ComInterface>::VTable,
    __refcnt: std::cell::Cell<u32>,
}
impl com::interfaces::IClassFactory for FactoryClassFactory {
    unsafe fn create_instance(
        &self,
        aggr: *mut *const <dyn com::interfaces::iunknown::IUnknown as com::ComInterface>::VTable,
        riid: *const com::sys::IID,
        ppv: *mut *mut std::ffi::c_void,
    ) -> com::sys::HRESULT {
        use com::interfaces::iunknown::IUnknown;
        if aggr != std::ptr::null_mut() {
            return com::sys::CLASS_E_NOAGGREGATION;
        }
        let mut instance = Factory::new();
        instance.add_ref();
        let hr = instance.query_interface(riid, ppv);
        instance.release();
        core::mem::forget(instance);
        hr
    }
    unsafe fn lock_server(&self, _increment: com::sys::BOOL) -> com::sys::HRESULT {
        com::sys::S_OK
    }
}
impl com::interfaces::IUnknown for FactoryClassFactory {
    unsafe fn query_interface(
        &self,
        riid: *const com::sys::IID,
        ppv: *mut *mut std::ffi::c_void,
    ) -> com::sys::HRESULT {
        use com::interfaces::iunknown::IUnknown;
        let riid = &*riid;
        if riid == &<dyn com::interfaces::iunknown::IUnknown as com::ComInterface>::IID
            || riid
                == &<dyn com::interfaces::iclass_factory::IClassFactory as com::ComInterface>::IID
        {
            *ppv = &self.__iclassfactoryvptr as *const _ as *mut std::ffi::c_void;
            self.add_ref();
            com::sys::NOERROR
        } else {
            *ppv = std::ptr::null_mut::<std::ffi::c_void>();
            com::sys::E_NOINTERFACE
        }
    }
    unsafe fn add_ref(&self) -> u32 {
        let value = self
            .__refcnt
            .get()
            .checked_add(1)
            .expect("Overflow of reference count");
        self.__refcnt.set(value);
        value
    }
    unsafe fn release(&self) -> u32 {
        use com::interfaces::iclass_factory::IClassFactory;
        let value = self
            .__refcnt
            .get()
            .checked_sub(1)
            .expect("Underflow of reference count");
        self.__refcnt.set(value);
        let __refcnt = self.__refcnt.get();
        if __refcnt == 0 {
            Box::from_raw(
                self.__iclassfactoryvptr as *mut <dyn IClassFactory as com::ComInterface>::VTable,
            );
            Box::from_raw(self as *const _ as *mut FactoryClassFactory);
        }
        __refcnt
    }
}
impl FactoryClassFactory {
    pub(crate) fn new() -> Box<FactoryClassFactory> {
        use com::interfaces::iclass_factory::IClassFactory;
        let iclassfactory_vtable = <dyn IClassFactory as ::com::ProductionComInterface<
            FactoryClassFactory,
        >>::vtable::<::com::offset::Zero>();
        let __iclassfactoryvptr = Box::into_raw(Box::new(iclassfactory_vtable));
        let out = FactoryClassFactory {
            __iclassfactoryvptr,
            __refcnt: std::cell::Cell::new(0),
        };
        Box::new(out)
    }
}
impl IAudioProcessor for PassthruPlugin {
    unsafe fn get_latency_sample(&self) -> u32 {
        0
    }
    unsafe fn setup_processing(&self, _setup: *mut ProcessSetup) -> tresult {
        kResultOk
    }
    unsafe fn set_processing(&self, _state: TBool) -> tresult {
        kResultOk
    }
    unsafe fn process(&self, _data: *mut ProcessData) -> tresult {
        kResultOk
    }
    unsafe fn get_tail_samples(&self) -> u32 {
        0
    }
}
impl IAudioPresentationLatency for PassthruPlugin {
    unsafe fn set_audio_presentation_latency_sample(
        &self,
        _dir: BusDirection,
        _bus_idx: i32,
        _latency_samples: u32,
    ) -> tresult {
        kResultOk
    }
}
impl IAutomationState for PassthruPlugin {
    unsafe fn set_automation_state(&self, _state: i32) -> tresult {
        kResultOk
    }
}
impl IPlugin for PassthruPlugin {
    unsafe fn initialize(&self, _host_context: *mut dyn IUnknown) -> tresult {
        kResultOk
    }
    unsafe fn terminate(&self) -> tresult {
        kResultOk
    }
}
impl IComponent for PassthruPlugin {
    unsafe fn get_controller_class_id(&self, _tuid: REFIID) -> tresult {
        kResultOk
    }
    unsafe fn set_io_mode(&self, _mode: i32) -> tresult {
        kResultOk
    }
    unsafe fn get_bus_count(&self, type_: i32, _dir: i32) -> i32 {
        if type_ == MediaTypes::kAudio as i32 {
            1
        } else {
            0
        }
    }
    unsafe fn get_bus_info(&self, type_: i32, dir: i32, _idx: i32, info: *mut BusInfo) -> tresult {
        if type_ == MediaTypes::kAudio as i32 {
            let info = unsafe { &mut *info };
            if dir == BusDirections::kInput as i32 {
                info.direction = dir;
                info.bus_type = MediaTypes::kAudio as i32;
                info.channel_count = 2;
                info.flags = BusFlags::kDefaultActive.bits();
            } else {
                info.direction = dir;
                info.bus_type = MediaTypes::kAudio as i32;
                info.channel_count = 2;
                info.flags = BusFlags::kDefaultActive.bits();
            }
            kResultOk
        } else {
            kInvalidArgument
        }
    }
    unsafe fn activate_bus(&self, _type_: i32, _dir: i32, _idx: i32, _state: TBool) -> tresult {
        kResultOk
    }
    unsafe fn set_active(&self, _state: TBool) -> tresult {
        kResultOk
    }
    unsafe fn set_state(&self, _state: *mut dyn IBStream) -> tresult {
        kResultOk
    }
    unsafe fn get_state(&self, _state: *mut dyn IBStream) -> tresult {
        kResultOk
    }
}
impl Factory {
    fn new() -> Box<Self> {
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(
                        &["instantiating factory..."],
                        &match () {
                            () => [],
                        },
                    ),
                    lvl,
                    &("passthru", "passthru", "src/lib.rs", 151u32),
                );
            }
        };
        Self::allocate()
    }
}
impl IPluginFactory for Factory {
    unsafe fn get_factory_info(&self, info: *mut vst3_sys::base::PFactoryInfo) -> i32 {
        let info = &mut *info;
        info.flags = 8;
        kResultOk
    }
    unsafe fn count_classes(&self) -> i32 {
        1
    }
    unsafe fn get_class_info(&self, idx: i32, _: *mut vst3_sys::base::PClassInfo) -> i32 {
        match idx {
            0 => {}
            _ => {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(
                                &["invalid class info ID "],
                                &match (&idx,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ),
                            lvl,
                            &("passthru", "passthru", "src/lib.rs", 176u32),
                        );
                    }
                };
                return kInvalidArgument;
            }
        }
        kResultOk
    }
    unsafe fn create_instance(
        &self,
        _: *mut com::sys::GUID,
        _: *mut com::sys::GUID,
        _: *mut *mut core::ffi::c_void,
    ) -> i32 {
        {
            ::std::rt::begin_panic("not implemented")
        }
    }
}
struct FactoryWrapper {
    factory: Box<Factory>,
}
unsafe impl Send for FactoryWrapper {}
unsafe impl Sync for FactoryWrapper {}
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct WRAPPER {
    __private_field: (),
}
#[doc(hidden)]
static WRAPPER: WRAPPER = WRAPPER {
    __private_field: (),
};
impl ::lazy_static::__Deref for WRAPPER {
    type Target = Mutex<FactoryWrapper>;
    fn deref(&self) -> &Mutex<FactoryWrapper> {
        #[inline(always)]
        fn __static_ref_initialize() -> Mutex<FactoryWrapper> {
            Mutex::new(FactoryWrapper {
                factory: Factory::new(),
            })
        }
        #[inline(always)]
        fn __stability() -> &'static Mutex<FactoryWrapper> {
            static LAZY: ::lazy_static::lazy::Lazy<Mutex<FactoryWrapper>> =
                ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for WRAPPER {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn GetPluginFactory() -> *mut c_void {
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                ::core::fmt::Arguments::new_v1(
                    &["calling plugin factory"],
                    &match () {
                        () => [],
                    },
                ),
                lvl,
                &("passthru", "passthru", "src/lib.rs", 204u32),
            );
        }
    };
    let factory = &mut *WRAPPER.lock().unwrap().factory;
    factory.add_ref();
    factory as *mut _ as *mut _
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn ModuleEntry(_: *mut c_void) -> bool {
    if let Err(e) = simple_logger::init() {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["", "\n"],
                &match (&e,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                },
            ));
        };
    }
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                ::core::fmt::Arguments::new_v1(
                    &["Module entered"],
                    &match () {
                        () => [],
                    },
                ),
                lvl,
                &("passthru", "passthru", "src/lib.rs", 216u32),
            );
        }
    };
    true
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn ModuleExit() -> bool {
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                ::core::fmt::Arguments::new_v1(
                    &["Module exited"],
                    &match () {
                        () => [],
                    },
                ),
                lvl,
                &("passthru", "passthru", "src/lib.rs", 223u32),
            );
        }
    };
    true
}
