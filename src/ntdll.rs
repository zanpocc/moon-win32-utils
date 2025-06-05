#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(clippy::all)]

use std::ffi::c_void;
use std::os::raw::{c_ulong};

use windows::core::PCWSTR;
use windows::Win32::Foundation::{BOOLEAN, HANDLE, NTSTATUS, UNICODE_STRING};

pub type PVOID = *mut c_void;
pub type ULONG = c_ulong;

#[link(name = "ntdll")]
extern "system" {
    pub fn NtQueryInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: ULONG,
        ProcessInformation: PVOID,
        ProcessInformationLength: ULONG,
        ReturnLength: *mut ULONG,
    ) -> NTSTATUS;

    pub fn NtQuerySystemInformation(
        SystemInformationClass: ULONG,
        SystemInformation: PVOID,
        SystemInformationLength: ULONG,
        ReturnLength: *mut ULONG,
    ) -> NTSTATUS;

    pub fn NtAllocateVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        ZeroBits: ULONG,
        RegionSize: *mut ULONG,
        AllocationType: ULONG,
        Protect: ULONG,
    ) -> NTSTATUS;

    pub fn NtFreeVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: *mut ULONG,
        FreeType: ULONG,
    ) -> NTSTATUS;

    pub fn RtlInitUnicodeString(
        DestinationString: *mut UNICODE_STRING,
        SourceString: PCWSTR,
    );

    pub fn RtlFreeUnicodeString(
        UnicodeString: *mut UNICODE_STRING,
    );

    pub fn RtlAdjustPrivilege(
        Privilege: ULONG,
        Enable: BOOLEAN,
        CurrentThread: BOOLEAN,
        Enabled: *mut BOOLEAN,
    ) -> NTSTATUS;

    pub fn NtLoadDriver(
        DriverServiceName: *mut UNICODE_STRING,
    ) -> NTSTATUS;

    pub fn NtUnloadDriver(
        DriverServiceName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

