// Copyright 2015  Emmanuele Bassi. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#![allow(non_camel_case_types)]
extern crate libc;

use libc::{c_int, c_uint, c_char, c_void, c_longlong, c_ulonglong};

// Public types; these are all opaque pointer types
pub type TagLib_File = c_void;
pub type TagLib_Tag = c_void;
pub type TagLib_AudioProperties = c_void;

pub type TagLib_Bool = c_int;
pub type TagLib_FileType = c_uint;

pub const TAGLIB_FILE_MPEG: TagLib_FileType = 0;
pub const TAGLIB_FILE_OGG_VORBIS: TagLib_FileType = 1;
pub const TAGLIB_FILE_FLAC: TagLib_FileType = 2;
pub const TAGLIB_FILE_MPC: TagLib_FileType = 3;
pub const TAGLIB_FILE_OGG_FLAC: TagLib_FileType = 4;
pub const TAGLIB_FILE_WAV_PACK: TagLib_FileType = 5;
pub const TAGLIB_FILE_SPEEX: TagLib_FileType = 6;
pub const TAGLIB_FILE_TRUE_AUDIO: TagLib_FileType = 7;
pub const TAGLIB_FILE_MP4: TagLib_FileType = 8;
pub const TAGLIB_FILE_ASF: TagLib_FileType = 9;
pub const TAGLIB_FILE_AIFF: TagLib_FileType = 10;
pub const TAGLIB_FILE_WAV: TagLib_FileType = 11;
pub const TAGLIB_FILE_APE: TagLib_FileType = 12;
pub const TAGLIB_FILE_IT: TagLib_FileType = 13;
pub const TAGLIB_FILE_MOD: TagLib_FileType = 14;
pub const TAGLIB_FILE_S3M: TagLib_FileType = 15;
pub const TAGLIB_FILE_XM: TagLib_FileType = 16;
pub const TAGLIB_FILE_OPUS: TagLib_FileType = 17;
pub const TAGLIB_FILE_DSF: TagLib_FileType = 18;
pub const TAGLIB_FILE_DSDIFF: TagLib_FileType = 19;

// tag_c.h
extern "C" {
    pub fn taglib_file_new(filename: *const c_char) -> *mut TagLib_File;
    pub fn taglib_file_new_type(
        filename: *const c_char,
        filetype: TagLib_FileType,
    ) -> *mut TagLib_File;
    pub fn taglib_file_is_valid(file: *mut TagLib_File) -> TagLib_Bool;
    pub fn taglib_file_free(file: *mut TagLib_File);
    pub fn taglib_file_save(file: *mut TagLib_File) -> TagLib_Bool;
    pub fn taglib_file_tag(file: *mut TagLib_File) -> *mut TagLib_Tag;
    pub fn taglib_file_audioproperties(file: *mut TagLib_File) -> *const TagLib_AudioProperties;

    pub fn taglib_tag_title(tag: *const TagLib_Tag) -> *const c_char;
    pub fn taglib_tag_artist(tag: *const TagLib_Tag) -> *const c_char;
    pub fn taglib_tag_album(tag: *const TagLib_Tag) -> *const c_char;
    pub fn taglib_tag_comment(tag: *const TagLib_Tag) -> *const c_char;
    pub fn taglib_tag_genre(tag: *const TagLib_Tag) -> *const c_char;
    pub fn taglib_tag_year(tag: *const TagLib_Tag) -> c_uint;
    pub fn taglib_tag_track(tag: *const TagLib_Tag) -> c_uint;
    pub fn taglib_tag_set_title(tag: *mut TagLib_Tag, title: *const c_char);
    pub fn taglib_tag_set_artist(tag: *mut TagLib_Tag, artist: *const c_char);
    pub fn taglib_tag_set_album(tag: *mut TagLib_Tag, album: *const c_char);
    pub fn taglib_tag_set_comment(tag: *mut TagLib_Tag, comment: *const c_char);
    pub fn taglib_tag_set_genre(tag: *mut TagLib_Tag, genre: *const c_char);
    pub fn taglib_tag_set_year(tag: *mut TagLib_Tag, year: c_uint);
    pub fn taglib_tag_set_track(tag: *mut TagLib_Tag, track: c_uint);
    pub fn taglib_tag_free_strings();

    #[doc = " Get the keys of the property map.\n\n \
    \\return NULL terminated array of C-strings (char *), only NULL if empty.\n \
    It must be freed by the client using taglib_property_free()."]
    pub fn taglib_property_keys(file: *const TagLib_File) -> *mut *mut c_char;

    #[doc = " Get value(s) of property \\a prop.\n\n \
    \\return NULL terminated array of C-strings (char *), only NULL if empty.\n \
    It must be freed by the client using taglib_property_free()."]
    pub fn taglib_property_get(file: *const TagLib_File,
                               prop: *const c_char, ) -> *mut *mut c_char;

    #[doc = " Sets the property \\a prop with \\a value. \
    Use \\a value = NULL to remove\n the property, otherwise it will be replaced."]
    pub fn taglib_property_set(file: *mut TagLib_File,
                               prop: *const c_char,
                               value: *const c_char);

    #[doc = " Appends \\a value to the property \\a prop (sets it if non-existing).\n \
    Use \\a value = NULL to remove all values associated with the property."]
    pub fn taglib_property_set_append(file: *mut TagLib_File,
                                      prop: *const c_char,
                                      value: *const c_char);

    #[doc = " Frees the NULL terminated array \\a props and the C-strings it contains."]
    pub fn taglib_property_free(props: *mut *mut c_char);

    pub fn taglib_audioproperties_length(properties: *const TagLib_AudioProperties) -> c_int;
    pub fn taglib_audioproperties_bitrate(properties: *const TagLib_AudioProperties) -> c_int;
    pub fn taglib_audioproperties_samplerate(properties: *const TagLib_AudioProperties) -> c_int;
    pub fn taglib_audioproperties_channels(properties: *const TagLib_AudioProperties) -> c_int;
}

pub const TAGLIB_VARIANT_TYPE_TAGLIB_VARIANT_VOID: TagLib_Variant_Type = 0;
pub const TAGLIB_VARIANT_TYPE_TAGLIB_VARIANT_BOOL: TagLib_Variant_Type = 1;
pub const TAGLIB_VARIANT_TYPE_TAGLIB_VARIANT_INT: TagLib_Variant_Type = 2;
pub const TAGLIB_VARIANT_TYPE_TAGLIB_VARIANT_UINT: TagLib_Variant_Type = 3;
pub const TAGLIB_VARIANT_TYPE_TAGLIB_VARIANT_LONG_LONG: TagLib_Variant_Type = 4;
pub const TAGLIB_VARIANT_TYPE_TAGLIB_VARIANT_ULONG_LONG: TagLib_Variant_Type = 5;
pub const TAGLIB_VARIANT_TYPE_TAGLIB_VARIANT_DOUBLE: TagLib_Variant_Type = 6;
pub const TAGLIB_VARIANT_TYPE_TAGLIB_VARIANT_STRING: TagLib_Variant_Type = 7;
pub const TAGLIB_VARIANT_TYPE_TAGLIB_VARIANT_STRING_LIST: TagLib_Variant_Type = 8;
pub const TAGLIB_VARIANT_TYPE_TAGLIB_VARIANT_BYTE_VECTOR: TagLib_Variant_Type = 9;

#[doc = " Types which can be stored in a TagLib_Variant.\n\n \
related TagLib::Variant::Type\n \
These correspond to TagLib::Variant::Type, but ByteVectorList, VariantList,\n \
VariantMap are not supported and will be returned as their string\n \
representation."]
pub type TagLib_Variant_Type = c_uint;

#[doc = " Discriminated union used in complex property attributes.\n\n \
e type must be set according to the \
e value union used.\n \
e size is only required for TagLib_Variant_ByteVector and must contain\n \
the number of bytes.\n\n \
related TagLib::Variant."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TagLib_Variant {
    pub r#type: TagLib_Variant_Type,
    pub size: c_uint,
    pub value: TagLib_Variant_Value_Union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union TagLib_Variant_Value_Union {
    pub string_value: *mut c_char,
    pub byte_vector_value: *mut c_char,
    pub string_list_value: *mut *mut c_char,
    pub bool_value: c_int,
    pub int_value: c_int,
    pub u_int_value: c_uint,
    pub long_long_value: c_longlong,
    pub u_long_long_value: c_ulonglong,
    pub double_value: f64,
}

#[doc = " Attribute of a complex property.\n \
Complex properties consist of a NULL-terminated array of pointers to\n \
this structure with \\e key and \\e value."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TagLib_Complex_Property_Attribute {
    pub key: *mut c_char,
    pub value: TagLib_Variant,
}

#[doc = " Picture data extracted from a complex property by the convenience function\n \
taglib_picture_from_complex_property()."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TagLib_Complex_Property_Picture_Data {
    pub mime_type: *mut c_char,
    pub description: *mut c_char,
    pub picture_type: *mut c_char,
    pub data: *mut c_char,
    pub size: c_uint,
}

extern "C" {
    #[doc = " Sets the complex property \\a key with \\a value.  Use \\a value = NULL to\n \
    remove the property, otherwise it will be replaced with the NULL\n \
    terminated array of attributes in \\a value.\n\n \
    A picture can be set with the TAGLIB_COMPLEX_PROPERTY_PICTURE macro"]
    pub fn taglib_complex_property_set(
        file: *mut TagLib_File,
        key: *const c_char,
        value: *mut *const TagLib_Complex_Property_Attribute,
    ) -> c_int;

    #[doc = " Appends \\a value to the complex property \\a key (sets it if non-existing).\n \
    Use \\a value = NULL to remove all values associated with the \\a key."]
    pub fn taglib_complex_property_set_append(
        file: *mut TagLib_File,
        key: *const c_char,
        value: *mut *const TagLib_Complex_Property_Attribute,
    ) -> c_int;

    #[doc = " Get the keys of the complex properties.\n\n \
    return NULL terminated array of C-strings (char *), only NULL if empty.\n \
    It must be freed by the client using taglib_complex_property_free_keys()."]
    pub fn taglib_complex_property_keys(
        file: *const TagLib_File,
    ) -> *mut *mut c_char;

    #[doc = " Get value(s) of complex property \\a key.\n\n \
    return NULL terminated array of property values, which are themselves an\n \
    array of property attributes, only NULL if empty.\n \
    It must be freed by the client using taglib_complex_property_free()."]
    pub fn taglib_complex_property_get(
        file: *const TagLib_File,
        key: *const c_char,
    ) -> *mut *mut *mut TagLib_Complex_Property_Attribute;

    #[doc = " Extract the complex property values of a picture.\n\n \
    This function can be used to get the data from a \"PICTURE\" complex property\n \
    without having to traverse the whole variant map. A picture can be\n retrieved."]
    pub fn taglib_picture_from_complex_property(
        properties: *mut *mut *mut TagLib_Complex_Property_Attribute,
        picture: *mut TagLib_Complex_Property_Picture_Data,
    );

    #[doc = " Frees the NULL terminated array \\a keys (as returned by\n \
    taglib_complex_property_keys()) and the C-strings it contains."]
    pub fn taglib_complex_property_free_keys(keys: *mut *mut c_char);

    #[doc = " Frees the NULL terminated array \\a props of property attribute arrays\n \
    (as returned by taglib_complex_property_get()) and the data such as\n \
    C-strings and byte vectors contained in these attributes."]
    pub fn taglib_complex_property_free(props: *mut *mut *mut TagLib_Complex_Property_Attribute);
}
