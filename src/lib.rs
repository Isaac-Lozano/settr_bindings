extern crate sa2_set;

use std::ffi::CStr;
use std::fs::File;
use std::os::raw::c_char;

use sa2_set::{SetFile, SetObject, Position, Rotation, Object, Dreamcast, GameCube, Pc};

macro_rules! try_or_err {
    ($try:expr, $ret:expr) => {
        match $try {
            Ok(val) => val,
            Err(_) => return $ret,
        }
    }
}

#[repr(C)]
pub struct CObject(pub u16);

impl From<Object> for CObject {
    fn from(other: Object) -> CObject {
        CObject(other.0)
    }
}

#[repr(C)]
pub struct CRotation {
    x: u16,
    y: u16,
    z: u16,
}

impl From<Rotation> for CRotation {
    fn from(other: Rotation) -> CRotation {
        CRotation {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }
}

#[repr(C)]
pub struct CPosition {
    x: f32,
    y: f32,
    z: f32,
}

impl From<Position> for CPosition {
    fn from(other: Position) -> CPosition {
        CPosition {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }
}

#[repr(C)]
pub struct CSetObject {
    object: CObject,
    rotation: CRotation,
    position: CPosition,
    attr1: f32,
    attr2: f32,
    attr3: f32,
}

impl From<SetObject> for CSetObject {
    fn from(other: SetObject) -> CSetObject {
        CSetObject {
            object: other.object.into(),
            rotation: other.rotation.into(),
            position: other.position.into(),
            attr1: other.attr1,
            attr2: other.attr2,
            attr3: other.attr3,
        }
    }
}

#[no_mangle]
pub extern "C" fn set_file_from_file(filename_ptr: *const c_char, platform: u32, setfile_ptr: *mut *mut SetFile) -> u32 {
    let filename = unsafe {
        try_or_err!(CStr::from_ptr(filename_ptr).to_str(), 1)
    };
    let file = try_or_err!(File::open(filename), 2);

    let set_file = Box::new(match platform as u32 {
        0 => try_or_err!(SetFile::from_read::<Dreamcast, _>(file), 3),
        1 => try_or_err!(SetFile::from_read::<GameCube, _>(file), 3),
        2 => try_or_err!(SetFile::from_read::<Pc, _>(file), 3),
        _ => return 3,
    });

    unsafe {
        *setfile_ptr = Box::into_raw(set_file);
    }

    0
}

#[no_mangle]
pub extern "C" fn set_file_free(setfile_ptr: *mut SetFile) {
    if setfile_ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(setfile_ptr);
    }
}

#[no_mangle]
pub extern "C" fn set_file_get_size(setfile_ptr: *mut SetFile) -> u32 {
    let set_file = unsafe {
        &mut *setfile_ptr
    };

    set_file.0.len() as u32
}

#[no_mangle]
pub extern "C" fn set_file_get_nth(setfile_ptr: *mut SetFile, idx: u32, setobj_ptr: *mut CSetObject) -> u32 {
    let set_file = unsafe {
        &mut *setfile_ptr
    };
    let set_obj = match set_file.0.get(idx as usize) {
        Some(val) => val,
        None => return 1,
    };
    unsafe {
        *setobj_ptr = (*set_obj).into();
    }
    0
}