// impls/mod.rs

#[cfg(all(
    feature = "implement-Truthy-for-AsStr",
    any(
        feature = "implement-Truthy-for-bool",
        feature = "implement-Truthy-for-CStr",
        feature = "implement-Truthy-for-CString",
        feature = "implement-Truthy-for-OsStr",
        feature = "implement-Truthy-for-OsString",
        feature = "implement-Truthy-for-str",
        feature = "implement-Truthy-for-String",
    ),
))]
compile_error!(
    "feature `implement-Truthy-for-AsStr` is incompatible with the other \
     `implement-Truthy-for-*` features; enable at most one style of \
     `Truthy` implementation"
);

#[cfg(feature = "implement-Truthy-for-AsStr")]
#[allow(non_snake_case)]
mod implement_Truthy_for_AsStr;
#[cfg(feature = "implement-Truthy-for-CStr")]
#[allow(non_snake_case)]
mod implement_Truthy_for_CStr;
#[cfg(feature = "implement-Truthy-for-CString")]
#[allow(non_snake_case)]
mod implement_Truthy_for_CString;
#[cfg(feature = "implement-Truthy-for-OsStr")]
#[allow(non_snake_case)]
mod implement_Truthy_for_OsStr;
#[cfg(feature = "implement-Truthy-for-OsString")]
#[allow(non_snake_case)]
mod implement_Truthy_for_OsString;
#[cfg(feature = "implement-Truthy-for-String")]
#[allow(non_snake_case)]
mod implement_Truthy_for_String;
#[cfg(feature = "implement-Truthy-for-bool")]
#[allow(non_snake_case)]
mod implement_Truthy_for_bool;
#[cfg(feature = "implement-Truthy-for-str")]
#[allow(non_snake_case)]
mod implement_Truthy_for_str;


// ///////////////////////////// end of file //////////////////////////// //
