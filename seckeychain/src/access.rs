//! Keychain item access control types: ACLs and policies around usage of
//! private keys stored in the keychain.

use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFOptionFlags, CFTypeRef},
    error::CFErrorRef,
    string::CFStringRef,
};
use std::{os::raw::c_void, ptr};

use error::Error;

/// Constraints on keychain item access.
///
/// See "Constraints" topic under the "Topics" section of the
/// `SecAccessControlCreateFlags` documentation at:
/// <https://developer.apple.com/documentation/security/secaccesscontrolcreateflags>
#[repr(u32)]
pub enum AccessConstraint {
    /// Require either passcode or biometric auth (TouchID/FaceID).
    ///
    /// Wrapper for `kSecAccessControlUserPresence`. See:
    /// <https://developer.apple.com/documentation/security/secaccesscontrolcreateflags/ksecaccesscontroluserpresence>
    UserPresence = 1,

    /// Require biometric auth (TouchID/FaceID) from any enrolled user for this device.
    ///
    /// Wrapper for `kSecAccessControlBiometryAny`. See:
    /// <https://developer.apple.com/documentation/security/secaccesscontrolcreateflags/ksecaccesscontrolbiometryany>
    BiometryAny = 1 << 1,

    /// Require biometric auth (TouchID/FaceID) from the current user.
    ///
    /// Wrapper for `kSecAccessControlBiometryCurrentSet`. See:
    /// <https://developer.apple.com/documentation/security/secaccesscontrolcreateflags/ksecaccesscontrolbiometrycurrentset>
    BiometryCurrentSet = 1 << 3,

    /// Require device passcode.
    ///
    /// Wrapper for `kSecAccessControlDevicePasscode`. See:
    /// <https://developer.apple.com/documentation/security/secaccesscontrolcreateflags/ksecaccesscontroldevicepasscode>
    DevicePasscode = 1 << 4,
}

/// Conjunctions (and/or) on keychain item access.
///
/// See "Conjunctions" topic under the "Topics" section of the
/// `SecAccessControlCreateFlags` documentation at:
/// <https://developer.apple.com/documentation/security/secaccesscontrolcreateflags>
#[repr(u32)]
pub enum AccessConjunction {
    /// Require *at least one* constraint must be satisfied.
    ///
    /// Wrapper for `kSecAccessControlAnd`. See:
    /// <https://developer.apple.com/documentation/security/secaccesscontrolcreateflags/ksecaccesscontrolor>
    Or = 1 << 14,

    /// Require *all* constraints be satisfied.
    ///
    /// Wrapper for `kSecAccessControlOr`. See:
    /// <https://developer.apple.com/documentation/security/secaccesscontrolcreateflags/ksecaccesscontroland>
    And = 1 << 15,
}

/// Options for keychain item access.
///
/// See "Additional Options" topic under the "Topics" section of the
/// `SecAccessControlCreateFlags` documentation at:
/// <https://developer.apple.com/documentation/security/secaccesscontrolcreateflags>
#[repr(u32)]
pub enum AccessOption {
    /// Require private key be stored in the device's Secure Enclave.
    ///
    /// Wrapper for `kSecAccessControlApplicationPassword`. See:
    /// <https://developer.apple.com/documentation/security/secaccesscontrolcreateflags/ksecaccesscontrolprivatekeyusage>
    PrivateKeyUsage = 1 << 30,

    /// Generate encryption-key from an application-provided password.
    ///
    /// Wrapper for `kSecAccessControlPrivateKeyUsage`. See:
    /// <https://developer.apple.com/documentation/security/secaccesscontrolcreateflags/ksecaccesscontrolapplicationpassword>
    ApplicationPassword = 1 << 31,
}

/// Access control policy for a particular keychain item.
///
/// More information about restricting keychain items can be found at:
/// <https://developer.apple.com/documentation/security/keychain_services/keychain_items/restricting_keychain_item_accessibility>
///
/// Wrapper for the `SecAccessControlCreateFlags` type:
/// <https://developer.apple.com/documentation/security/secaccesscontrolcreateflags>
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct AccessControlFlags(CFOptionFlags);

impl AccessControlFlags {
    /// Create `AccessControlFlags` with no policy set
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an access flag to the set of flags
    // TODO: handle illegal combinations of flags?
    pub fn add<T: Into<CFOptionFlags>>(&mut self, flag: T) {
        self.0 |= flag.into();
    }
}

/// Keychain item accessibility restrictions (from most to least restrictive).
///
/// More information about restricting keychain items can be found at:
/// <https://developer.apple.com/documentation/security/keychain_services/keychain_items/restricting_keychain_item_accessibility>
///
/// Wrapper for the `kSecAttrAccessible` attribute key. See
/// "Accessibility Values" section of "Item Attribute Keys and Values":
/// <https://developer.apple.com/documentation/security/keychain_services/keychain_items/item_attribute_keys_and_values>
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Accessible {
    /// Device is unlocked and a passcode has been set on the device.
    /// <https://developer.apple.com/documentation/security/ksecattraccessiblewhenpasscodesetthisdeviceonly>
    WhenPasscodeSetThisDeviceOnly,

    /// The device is unlocked (no passcode mandatory). Non-exportable.
    /// <https://developer.apple.com/documentation/security/ksecattraccessiblewhenunlockedthisdeviceonly>
    WhenUnlockedThisDeviceOnly,

    /// The device is unlocked (no passcode mandatory).
    /// <https://developer.apple.com/documentation/security/ksecattraccessiblewhenunlocked>
    WhenUnlocked,

    /// Permanently accessible after the device is first unlocked after boot.
    /// Non-exportable.
    /// <https://developer.apple.com/documentation/security/ksecattraccessibleafterfirstunlockthisdeviceonly>
    AfterFirstUnlockThisDeviceOnly,

    /// Permanently accessible after the device is first unlocked after boot.
    /// <https://developer.apple.com/documentation/security/ksecattraccessibleafterfirstunlock>
    AfterFirstUnlock,

    /// Item is always accessible on this device. Non-exportable.
    /// <https://developer.apple.com/documentation/security/ksecattraccessiblealwaysthisdeviceonly>
    AlwaysThisDeviceOnly,

    /// Item is always accessible.
    /// <https://developer.apple.com/documentation/security/ksecattraccessiblealways>
    Always,
}

impl Accessible {
    /// Get pointer to an accessibility value to associate with the
    /// `kSecAttrAccessible` key for a keychain item
    fn as_ptr(self) -> *const c_void {
        extern "C" {
            static kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly: CFStringRef;
            static kSecAttrAccessibleWhenUnlockedThisDeviceOnly: CFStringRef;
            static kSecAttrAccessibleWhenUnlocked: CFStringRef;
            static kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly: CFStringRef;
            static kSecAttrAccessibleAfterFirstUnlock: CFStringRef;
            static kSecAttrAccessibleAlwaysThisDeviceOnly: CFStringRef;
            static kSecAttrAccessibleAlways: CFStringRef;
        }

        let attr = unsafe {
            match self {
                Accessible::WhenPasscodeSetThisDeviceOnly => {
                    kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly
                }
                Accessible::WhenUnlockedThisDeviceOnly => {
                    kSecAttrAccessibleWhenUnlockedThisDeviceOnly
                }
                Accessible::WhenUnlocked => kSecAttrAccessibleWhenUnlocked,
                Accessible::AfterFirstUnlockThisDeviceOnly => {
                    kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly
                }
                Accessible::AfterFirstUnlock => kSecAttrAccessibleAfterFirstUnlock,
                Accessible::AlwaysThisDeviceOnly => kSecAttrAccessibleAlwaysThisDeviceOnly,
                Accessible::Always => kSecAttrAccessibleAlways,
            }
        };

        attr as *const c_void
    }
}

/// Access control policy (a.k.a. ACL) for a keychain item.
///
/// Wrapper for the `SecAccessControlRef` type:
/// <https://developer.apple.com/documentation/security/secaccesscontrolref>
pub struct AccessControl(CFTypeRef);

impl AccessControl {
    /// Create a new `AccessControl` policy/ACL.
    ///
    /// Wrapper for the `SecAccessControlCreateWithFlags()` function:
    /// <https://developer.apple.com/documentation/security/1394452-secaccesscontrolcreatewithflags>
    pub fn create_with_flags(
        protection: Accessible,
        flags: AccessControlFlags,
    ) -> Result<AccessControl, Error> {
        extern "C" {
            fn SecAccessControlCreateWithFlags(
                allocator: CFAllocatorRef,
                protection: CFTypeRef,
                flags: CFOptionFlags,
                error: *mut CFErrorRef,
            ) -> CFTypeRef;
        }

        let mut error: CFErrorRef = ptr::null_mut();
        let result = unsafe {
            SecAccessControlCreateWithFlags(
                kCFAllocatorDefault,
                protection.as_ptr(),
                flags.0,
                &mut error,
            )
        };

        if error.is_null() {
            Ok(AccessControl(result))
        } else {
            Err(error.into())
        }
    }
}
