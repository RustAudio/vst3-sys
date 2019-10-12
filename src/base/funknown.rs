use crate::base::tresult;
pub use com::interfaces::iunknown::IUnknown;

pub const kNoInterface: tresult = -1;
pub const kResultOk: tresult = 0;
pub const kResultTrue: tresult = kResultOk;
pub const kResultFalse: tresult = 1;
pub const kInvalidArgument: tresult = 2;
pub const kNotImplemented: tresult = 3;
pub const kInternalError: tresult = 4;
pub const kNotInitialized: tresult = 5;
pub const kOutOfMemory: tresult = 6;
