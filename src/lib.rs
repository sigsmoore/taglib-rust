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

#![crate_name = "taglib"]
#![crate_type = "lib"]

extern crate lazy_static;
extern crate libc;
extern crate taglib_sys as sys;

#[cfg(target_os = "windows")]
extern crate windows_sys as windows;

#[cfg(target_os = "windows")]
extern crate codepage;

use lazy_static::lazy_static;
use libc::c_char;
use std::collections::HashSet;
use std::convert::TryInto;
use std::ffi::{CString, CStr};
use std::path::Path;
use std::{mem, ptr};
use std::str::Utf8Error;

use sys as ll;

fn c_str_to_str(c_str: *const c_char) -> Option<String> {
    if c_str.is_null() {
        None
    } else {
        let bytes = unsafe { CStr::from_ptr(c_str).to_bytes() };

        if bytes.is_empty() { None } else { Some(String::from_utf8_lossy(bytes).to_string()) }
    }
}

fn u32_to_option(n: u32) -> Option<u32> {
    if n == 0 { None } else { Some(n) }
}

/// A representation of an audio file, with meta-data and properties.
pub struct File {
    raw: *mut ll::TagLib_File,
}

/// The abstract meta-data container for audio files
///
/// Each `Tag` instance can only be created by the `taglib::File::tag()`
/// method.
#[allow(dead_code)]
pub struct Tag<'a> {
    raw: *mut ll::TagLib_Tag,
    file: &'a File,
}

/// Common audio file properties.
///
/// Instances of `AudioProperties` can only be created through the
/// `taglib::File::audioproperties()` method.
#[allow(dead_code)]
pub struct AudioProperties<'a> {
    raw: *const ll::TagLib_AudioProperties,
    file: &'a File,
}

impl<'a> Tag<'a> {
    /// Returns the track name, if any.
    pub fn title(&self) -> Option<String> {
        let res = unsafe { ll::taglib_tag_title(self.raw) };
        c_str_to_str(res)
    }

    /// Sets the track name.
    pub fn set_title(&mut self, title: &str) {
        let cs = CString::new(title).unwrap();
        let s = cs.as_ptr();
        unsafe {
            ll::taglib_tag_set_title(self.raw, s);
        }
    }

    /// Returns the artist name, if any.
    pub fn artist(&self) -> Option<String> {
        let res = unsafe { ll::taglib_tag_artist(self.raw) };
        c_str_to_str(res)
    }

    /// Sets the artist name.
    pub fn set_artist(&mut self, artist: &str) {
        let cs = CString::new(artist).unwrap();
        let s = cs.as_ptr();
        unsafe {
            ll::taglib_tag_set_artist(self.raw, s);
        }
    }

    /// Returns the album name, if any.
    pub fn album(&self) -> Option<String> {
        let res = unsafe { ll::taglib_tag_album(self.raw) };
        c_str_to_str(res)
    }

    /// Sets the album name.
    pub fn set_album(&mut self, album: &str) {
        let cs = CString::new(album).unwrap();
        let s = cs.as_ptr();
        unsafe {
            ll::taglib_tag_set_album(self.raw, s);
        }
    }

    /// Returns the track comment, if any.
    pub fn comment(&self) -> Option<String> {
        let res = unsafe { ll::taglib_tag_comment(self.raw) };
        c_str_to_str(res)
    }

    /// Sets the track comment.
    pub fn set_comment(&mut self, comment: &str) {
        let cs = CString::new(comment).unwrap();
        let s = cs.as_ptr();
        unsafe {
            ll::taglib_tag_set_comment(self.raw, s);
        }
    }

    /// Returns the genre name, if any.
    pub fn genre(&self) -> Option<String> {
        let res = unsafe { ll::taglib_tag_genre(self.raw) };
        c_str_to_str(res)
    }

    /// Sets the genre name.
    pub fn set_genre(&mut self, genre: &str) {
        let cs = CString::new(genre).unwrap();
        let s = cs.as_ptr();
        unsafe {
            ll::taglib_tag_set_genre(self.raw, s);
        }
    }

    /// Returns the year, if any.
    pub fn year(&self) -> Option<u32> {
        u32_to_option(unsafe { ll::taglib_tag_year(self.raw) as u32 })
    }

    /// Sets the year.
    pub fn set_year(&mut self, year: u32) {
        unsafe {
            ll::taglib_tag_set_year(self.raw, year);
        }
    }

    /// Returns the track number, if any.
    pub fn track(&self) -> Option<u32> {
        u32_to_option(unsafe { ll::taglib_tag_track(self.raw) as u32 })
    }

    /// Sets the track number.
    pub fn set_track(&mut self, track: u32) {
        unsafe {
            ll::taglib_tag_set_track(self.raw, track);
        }
    }

    pub fn album_artist(&self) -> Option<String> {
        self.file.album_artist()
    }

    pub fn composer(&self) -> Option<String> {
        self.file.composer()
    }

    pub fn copyright(&self) -> Option<String> {
        self.file.copyright()
    }

    pub fn lyrics(&self) -> Option<String> {
        self.file.lyrics()
    }

    pub fn track_number(&self) -> Option<u32> {
        self.file.track_number()
    }

    pub fn track_number_string(&self) -> Option<String> {
        self.file.track_number_string()
    }

    pub fn track_total(&self) -> Option<u32> {
        self.file.track_total()
    }

    pub fn track_total_string(&self) -> Option<String> {
        self.file.track_total_string()
    }

    pub fn disc_number(&self) -> Option<u32> {
        self.file.disc_number()
    }

    pub fn disc_number_string(&self) -> Option<String> {
        self.file.disc_number_string()
    }

    pub fn disc_total(&self) -> Option<u32> {
        self.file.disc_total()
    }

    pub fn disc_total_string(&self) -> Option<String> {
        self.file.disc_total_string()
    }

    pub fn date(&self) -> Option<String> {
        self.file.date()
    }
}

impl<'a> AudioProperties<'a> {
    /// Returns the length, in seconds, of the track.
    pub fn length(&self) -> u32 {
        unsafe { ll::taglib_audioproperties_length(self.raw) as u32 }
    }

    /// Returns the most appropriate bit rate for the track, in kB/s.
    /// For constant bit rate formats, the returned value is the bit
    /// rate of the file; for variable bit rate formats this is either
    /// the average or the nominal bit rate.
    pub fn bitrate(&self) -> u32 {
        unsafe { ll::taglib_audioproperties_bitrate(self.raw) as u32 }
    }

    /// Returns the sample rate, in Hz.
    pub fn samplerate(&self) -> u32 {
        unsafe { ll::taglib_audioproperties_samplerate(self.raw) as u32 }
    }

    /// Returns the number of audio channels.
    pub fn channels(&self) -> u32 {
        unsafe { ll::taglib_audioproperties_channels(self.raw) as u32 }
    }
}

const MUT_PTR_C_CHAR_LEN: usize = mem::size_of::<*mut c_char>();

#[derive(Copy, Clone, PartialEq)]
pub enum FileType {
    /// MPEG file
    MPEG = ll::TAGLIB_FILE_MPEG as isize,
    /// Ogg/Vorbis file
    OggVorbis = ll::TAGLIB_FILE_OGG_VORBIS as isize,
    /// FLAC file
    FLAC = ll::TAGLIB_FILE_FLAC as isize,
    /// MPC file
    MPC = ll::TAGLIB_FILE_MPC as isize,
    /// Ogg/FLAC file
    OggFlac = ll::TAGLIB_FILE_OGG_FLAC as isize,
    /// WavPack file
    WavPack = ll::TAGLIB_FILE_WAV_PACK as isize,
    /// Ogg/Speex file
    Speex = ll::TAGLIB_FILE_SPEEX as isize,
    /// TrueAudio file
    TrueAudio = ll::TAGLIB_FILE_TRUE_AUDIO as isize,
    /// MP4 file
    MP4 = ll::TAGLIB_FILE_MP4 as isize,
    /// ASF file
    ASF = ll::TAGLIB_FILE_ASF as isize,
    /// AIFF file
    AIFF = ll::TAGLIB_FILE_AIFF as isize,
    /// WAV file
    WAV = ll::TAGLIB_FILE_WAV as isize,
    /// APE file
    APE = ll::TAGLIB_FILE_APE as isize,
    /// IT file
    IT = ll::TAGLIB_FILE_IT as isize,
    /// MOD file
    MOD = ll::TAGLIB_FILE_MOD as isize,
    /// S3M file
    S3M = ll::TAGLIB_FILE_S3M as isize,
    /// XM file
    XM = ll::TAGLIB_FILE_XM as isize,
    /// OPUS file
    OPUS = ll::TAGLIB_FILE_OPUS as isize,
    /// DSF file
    DSF = ll::TAGLIB_FILE_DSF as isize,
    /// DSDIFF file
    DFF = ll::TAGLIB_FILE_DSDIFF as isize,
}

lazy_static! {
    static ref MPEG_SUFFIX: Vec<&'static str> = vec![".mp3", ".aac"];
    static ref OGG_VORBIS_SUFFIX: Vec<&'static str> = vec![".ogg"];
    static ref FLAC_SUFFIX: Vec<&'static str> = vec![".flac"];
    static ref MPC_SUFFIX: Vec<&'static str> = vec![".mpc"];
    static ref OGG_FLAC_SUFFIX: Vec<&'static str> = vec![".flac", ".oga"];

    static ref WAV_PACK_SUFFIX: Vec<&'static str> = vec![".wv"];
    static ref SPEEX_SUFFIX: Vec<&'static str> = vec![".spx"];
    static ref TRUE_AUDIO_SUFFIX: Vec<&'static str> = vec![".tta"];
    static ref MP4_SUFFIX: Vec<&'static str> = vec![".mp4", ".m4a", "m4b", "m4p", "m4v", "isom", "3g2"];
    static ref ASF_SUFFIX: Vec<&'static str> = vec![".asf", ".wma"];

    static ref AIFF_SUFFIX: Vec<&'static str> = vec![".aif", ".aiff", ".aifc"];
    static ref WAV_SUFFIX: Vec<&'static str> = vec![".wav"];
    static ref APE_SUFFIX: Vec<&'static str> = vec![".ape"];
    static ref IT_SUFFIX: Vec<&'static str> = vec![".it"];
    static ref MOD_SUFFIX: Vec<&'static str> = vec![".mod"];

    static ref S3M_SUFFIX: Vec<&'static str> = vec![".s3m"];
    static ref XM_SUFFIX: Vec<&'static str> = vec![".xm"];
    static ref OPUS_SUFFIX: Vec<&'static str> = vec![".opus"];
    static ref DSF_SUFFIX: Vec<&'static str> = vec![".dsf"];
    static ref DFF_SUFFIX: Vec<&'static str> = vec![".dff"];

    static ref EMPTY_SUFFIX: Vec<&'static str> = vec![];

    static ref ALL_SUFFIX: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.extend(MPEG_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(OGG_VORBIS_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(FLAC_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(MPC_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(OGG_FLAC_SUFFIX.iter().cloned().collect::<HashSet<&str>>());

        m.extend(WAV_PACK_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(SPEEX_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(TRUE_AUDIO_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(MP4_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(ASF_SUFFIX.iter().cloned().collect::<HashSet<&str>>());

        m.extend(AIFF_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(WAV_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(APE_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(IT_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(MOD_SUFFIX.iter().cloned().collect::<HashSet<&str>>());

        m.extend(S3M_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(XM_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(OPUS_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(DSF_SUFFIX.iter().cloned().collect::<HashSet<&str>>());
        m.extend(DFF_SUFFIX.iter().cloned().collect::<HashSet<&str>>());

        m
    };
}

impl FileType {
    pub fn name(&self) -> &'static str {
        if self == &FileType::MPEG {
            "MPEG"
        } else if self == &FileType::OggVorbis {
            "OggVorbis"
        } else if self == &FileType::FLAC {
            "FLAC"
        } else if self == &FileType::MPC {
            "MPC"
        } else if self == &FileType::OggFlac {
            "OggFlac"
        } else if self == &FileType::WavPack {
            "WavPack"
        } else if self == &FileType::Speex {
            "Speex"
        } else if self == &FileType::TrueAudio {
            "TrueAudio"
        } else if self == &FileType::MP4 {
            "MP4"
        } else if self == &FileType::ASF {
            "ASF"
        } else if self == &FileType::AIFF {
            "AIFF"
        } else if self == &FileType::WAV {
            "WAV"
        } else if self == &FileType::APE {
            "APE"
        } else if self == &FileType::IT {
            "IT"
        } else if self == &FileType::MOD {
            "MOD"
        } else if self == &FileType::S3M {
            "S3M"
        } else if self == &FileType::XM {
            "XM"
        } else if self == &FileType::OPUS {
            "OPUS"
        } else if self == &FileType::DSF {
            "DSF"
        } else if self == &FileType::DFF {
            "DFF"
        } else {
            ""
        }
    }

    pub fn suffix(&self) -> &'static Vec<&str> {
        if self == &FileType::MPEG {
            &MPEG_SUFFIX
        } else if self == &FileType::OggVorbis {
            &OGG_VORBIS_SUFFIX
        } else if self == &FileType::FLAC {
            &FLAC_SUFFIX
        } else if self == &FileType::MPC {
            &MPC_SUFFIX
        } else if self == &FileType::OggFlac {
            &OGG_FLAC_SUFFIX
        } else if self == &FileType::WavPack {
            &WAV_PACK_SUFFIX
        } else if self == &FileType::Speex {
            &SPEEX_SUFFIX
        } else if self == &FileType::TrueAudio {
            &TRUE_AUDIO_SUFFIX
        } else if self == &FileType::MP4 {
            &MP4_SUFFIX
        } else if self == &FileType::ASF {
            &ASF_SUFFIX
        } else if self == &FileType::AIFF {
            &AIFF_SUFFIX
        } else if self == &FileType::WAV {
            &WAV_SUFFIX
        } else if self == &FileType::APE {
            &APE_SUFFIX
        } else if self == &FileType::IT {
            &IT_SUFFIX
        } else if self == &FileType::MOD {
            &MOD_SUFFIX
        } else if self == &FileType::S3M {
            &S3M_SUFFIX
        } else if self == &FileType::XM {
            &XM_SUFFIX
        } else if self == &FileType::OPUS {
            &OPUS_SUFFIX
        } else if self == &FileType::DSF {
            &DSF_SUFFIX
        } else if self == &FileType::DFF {
            &DSF_SUFFIX
        } else {
            &EMPTY_SUFFIX
        }
    }

    pub fn all_suffix() -> &'static HashSet<&'static str> {
        &*ALL_SUFFIX
    }
}

#[derive(Debug)]
pub enum FileError {
    /// The file is an invalid or an unrecognized audio container
    InvalidFile,
    /// The file name is invalid
    InvalidFileName,
    /// No meta-data is available
    NoAvailableTag,
    /// No audio properties are available
    NoAvailableAudioProperties,
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            ll::taglib_file_free(self.raw);
        }
    }
}

// Define keys for get / set properties
const KEY_ALBUM_ARTIST: &'static str = "ALBUMARTIST";
const KEY_COMPOSER: &'static str = "COMPOSER";
const KEY_COPYRIGHT: &'static str = "COPYRIGHT";
const KEY_LYRICS: &'static str = "LYRICS";
const KEY_DATE: &'static str = "DATE";

// key for property, value like 01/02, first is disc_number, last is disc_total
const KEY_DISC_NUMBER: &'static str = "DISCNUMBER";
// key for property, value like 01/10, first is track_number, last is track_total
const KEY_TRACK_NUMBER: &'static str = "TRACKNUMBER";
// key for property, value like 10, only contains track_total
const KEY_TRACK_TOTAL: &'static str = "TRACKTOTAL";

#[cfg(target_os = "windows")]
fn acp_encode(s: &str) -> Option<Vec<u8>> {
    let acp = unsafe { windows::Win32::Globalization::GetACP() };
    let e = codepage::to_encoding(acp as u16)?;

    let (res, _e, has_error) = e.encode(s);
    if !has_error {
        let vec = res.iter().cloned().collect();
        Some(vec)
    } else {
        None
    }
}

#[cfg(not(target_os = "windows"))]
fn acp_encode(_s: &str) -> Option<Vec<u8>> {
    None
}

fn get_filename_c(filename: &str) -> Result<CString, FileError> {
    acp_encode(filename)
        .map_or_else(|| CString::new(filename),
                     |v| {
                         let from_vec = unsafe { CString::from_vec_unchecked(v) };
                         Ok(from_vec)
                     })
        .map_err(|_| FileError::InvalidFileName)
}

impl File {
    /// Creates a new `taglib::File` for the given `filename`.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<File, FileError> {
        let filename = path.as_ref().to_str().ok_or(FileError::InvalidFileName)?;
        let filename_c = get_filename_c(filename)?;
        let filename_c_ptr = filename_c.as_ptr();

        let f = unsafe { ll::taglib_file_new(filename_c_ptr) };
        if f.is_null() {
            return Err(FileError::InvalidFile);
        }

        Ok(File { raw: f })
    }

    /// Creates a new `taglib::File` for the given `filename` and type of file.
    pub fn new_type(filename: &str, filetype: FileType) -> Result<File, FileError> {
        let filename_c = get_filename_c(filename)?;
        let filename_c_ptr = filename_c.as_ptr();

        let f = unsafe {
            ll::taglib_file_new_type(filename_c_ptr, (filetype as u32).try_into().unwrap())
        };
        if f.is_null() {
            return Err(FileError::InvalidFile);
        }

        Ok(File { raw: f })
    }

    /// Returns the `taglib::Tag` instance for the given file.
    pub fn tag(&self) -> Result<Tag, FileError> {
        let res = unsafe { ll::taglib_file_tag(self.raw) };

        if res.is_null() {
            Err(FileError::NoAvailableTag)
        } else {
            Ok(Tag {
                raw: res,
                file: self,
            })
        }
    }

    /// Returns whether the file is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { ll::taglib_file_is_valid(self.raw) != 0 }
    }

    /// Returns the `taglib::AudioProperties` instance for the given file.
    pub fn audioproperties(&self) -> Result<AudioProperties, FileError> {
        let res = unsafe { ll::taglib_file_audioproperties(self.raw) };

        if res.is_null() {
            Err(FileError::NoAvailableAudioProperties)
        } else {
            Ok(AudioProperties {
                raw: res,
                file: self,
            })
        }
    }

    pub fn album_artist(&self) -> Option<String> {
        self.get_first_property(KEY_ALBUM_ARTIST)
    }

    pub fn set_album_artist(&mut self, value: &str) {
        self.set_property(KEY_ALBUM_ARTIST, value);
    }

    pub fn remove_album_artist(&mut self) {
        self.remove_property(KEY_ALBUM_ARTIST);
    }

    pub fn composer(&self) -> Option<String> {
        self.get_first_property(KEY_COMPOSER)
    }

    pub fn set_composer(&mut self, value: &str) {
        self.set_property(KEY_COMPOSER, value);
    }

    pub fn remove_composer(&mut self) {
        self.remove_property(KEY_COMPOSER);
    }

    pub fn copyright(&self) -> Option<String> {
        self.get_first_property(KEY_COPYRIGHT)
    }

    pub fn set_copyright(&mut self, value: &str) {
        self.set_property(KEY_COPYRIGHT, value);
    }

    pub fn remove_copyright(&mut self) {
        self.remove_property(KEY_COPYRIGHT);
    }

    pub fn lyrics(&self) -> Option<String> {
        self.get_first_property(KEY_LYRICS)
    }

    pub fn set_lyrics(&mut self, value: &str) {
        self.set_property(KEY_LYRICS, value);
    }

    pub fn remove_lyrics(&mut self) {
        self.remove_property(KEY_LYRICS);
    }

    pub fn date(&self) -> Option<String> {
        self.get_first_property(KEY_DATE)
    }

    pub fn set_date(&mut self, value: &str) {
        self.set_property(KEY_DATE, value);
    }

    pub fn remove_date(&mut self) {
        self.remove_property(KEY_DATE);
    }

    pub fn track_number(&self) -> Option<u32> {
        self.tag().unwrap().track()
    }

    pub fn track_number_string(&self) -> Option<String> {
        if let Some(track) = self.tag().unwrap().track() {
            if let Some(track_string_from_prop) = self.track_number_string_from_prop() {
                if let Some(track_from_prop) = track_string_from_prop.parse::<u32>().ok() {
                    if track_from_prop == track {
                        return Some(track_string_from_prop);
                    }
                }
            }
            return Some(track.to_string());
        }
        None
    }

    pub fn set_track_number(&mut self, value: u32, padding: usize) {
        let t = self.tag().unwrap();
        unsafe {
            ll::taglib_tag_set_track(t.raw, value);
        }

        if let Some(track_total_string) = self.get_first_property(KEY_TRACK_TOTAL) {
            if let Some(track_total) = track_total_string.parse::<u32>().ok() {
                self.set_property(KEY_TRACK_TOTAL,
                                  &decimal_to_padding_string(track_total, padding));
                self.set_property_split_num(KEY_TRACK_NUMBER,
                                            &Some(value),
                                            &Some(track_total),
                                            padding);
                return;
            }
        }

        let track_total = self.track_total_from_prop_track_number();
        if let Some(t) = track_total {
            self.set_property(KEY_TRACK_TOTAL, &decimal_to_padding_string(t, padding));
        }
        self.set_property_split_num(KEY_TRACK_NUMBER, &Some(value), &track_total, padding);
    }

    pub fn remove_track_number(&mut self) {
        let t = self.tag().unwrap();
        unsafe {
            ll::taglib_tag_set_track(t.raw, 0);
        }

        let track_total = self.track_total_string();
        self.set_property_split_text(KEY_TRACK_NUMBER, &None, &track_total);
    }

    pub fn track_total(&self) -> Option<u32> {
        if let Some(track_total_string) = self.get_first_property(KEY_TRACK_TOTAL) {
            let track_total = track_total_string.parse::<u32>().ok();
            if track_total.is_some() {
                return track_total;
            }
        }
        self.track_total_from_prop_track_number()
    }

    pub fn track_total_string(&self) -> Option<String> {
        let track_total = self.get_first_property(KEY_TRACK_TOTAL);
        if track_total.is_some() {
            track_total
        } else {
            self.track_total_string_from_prop_track_number()
        }
    }

    pub fn set_track_total(&mut self, value: u32, padding: usize) {
        self.set_property(KEY_TRACK_TOTAL, &decimal_to_padding_string(value, padding));

        let track_number = self.track_number_from_prop();
        self.set_property_split_num(KEY_TRACK_NUMBER, &track_number, &Some(value), padding);
    }

    pub fn remove_track_total(&mut self) {
        self.remove_property(KEY_TRACK_TOTAL);

        let track_number = self.track_number_string();
        self.set_property_split_text(KEY_TRACK_NUMBER, &track_number, &None);
    }

    fn track_number_from_prop(&mut self) -> Option<u32> {
        let (track_number, _) = self.number_pair_by_key(KEY_TRACK_NUMBER);
        track_number
    }

    fn track_number_string_from_prop(&self) -> Option<String> {
        let (track_number, _) = self.text_pair_by_key(KEY_TRACK_NUMBER);
        track_number
    }

    fn track_total_from_prop_track_number(&self) -> Option<u32> {
        let (_, track_total) = self.number_pair_by_key(KEY_TRACK_NUMBER);
        track_total
    }

    fn track_total_string_from_prop_track_number(&self) -> Option<String> {
        let (_, track_total) = self.text_pair_by_key(KEY_TRACK_NUMBER);
        track_total
    }

    pub fn disc_number(&self) -> Option<u32> {
        let (disc_number, _) = self.number_pair_by_key(KEY_DISC_NUMBER);
        disc_number
    }

    pub fn disc_number_string(&self) -> Option<String> {
        let (disc_number, _) = self.text_pair_by_key(KEY_DISC_NUMBER);
        disc_number
    }

    pub fn set_disc_number(&mut self, value: u32, padding: usize) {
        let disc_total = self.disc_total();
        self.set_property_split_num(KEY_DISC_NUMBER, &Some(value), &disc_total, padding);
    }

    pub fn remove_disc_number(&mut self) {
        let (_, disc_total) = self.text_pair_by_key(KEY_DISC_NUMBER);
        self.set_property_split_text(KEY_DISC_NUMBER, &None, &disc_total);
    }

    pub fn disc_total(&self) -> Option<u32> {
        let (_, disc_total) = self.number_pair_by_key(KEY_DISC_NUMBER);
        disc_total
    }

    pub fn disc_total_string(&self) -> Option<String> {
        let (_, disc_total) = self.text_pair_by_key(KEY_DISC_NUMBER);
        disc_total
    }

    pub fn set_disc_total(&mut self, total_disc: u32, padding: usize) {
        let disc_number = self.disc_number();
        self.set_property_split_num(KEY_DISC_NUMBER, &disc_number, &Some(total_disc), padding);
    }

    pub fn remove_disc_total(&mut self) {
        let (disc_number, _) = self.text_pair_by_key(KEY_DISC_NUMBER);
        self.set_property_split_text(KEY_DISC_NUMBER, &disc_number, &None);
    }

    fn set_property_split_text(&mut self,
                               key: &str,
                               first: &Option<String>,
                               last: &Option<String>) {
        self.remove_property(key);
        if let Some(ref value) = text_pair_to_string(first, last) {
            self.set_property(key, value);
        }
    }

    fn set_property_split_num(&mut self,
                              key: &str,
                              first: &Option<u32>,
                              last: &Option<u32>,
                              padding: usize) {
        self.remove_property(key);
        if let Some(ref value) = num_pair_to_string(first, last, padding) {
            self.set_property(key, value);
        }
    }

    fn number_pair_by_key(&self, key: &str) -> (Option<u32>, Option<u32>) {
        if let Some(ref text) = self.get_first_property(key) {
            get_number_pair(text)
        } else {
            (None, None)
        }
    }

    fn text_pair_by_key(&self, key: &str) -> (Option<String>, Option<String>) {
        if let Some(ref text) = self.get_first_property(key) {
            get_text_pair(text)
        } else {
            (None, None)
        }
    }

    pub fn get_first_property(&self, key: &str) -> Option<String> {
        let vec = self.get_property(key).ok()?;
        if !vec.is_empty() {
            Some(vec.first().unwrap().clone())
        } else {
            None
        }
    }

    pub fn get_property(&self, key: &str) -> Result<Vec<String>, Utf8Error> {
        let cs = CString::new(key).unwrap();
        let s = cs.as_ptr();
        let call_res = unsafe {
            ll::taglib_property_get(self.raw, s)
        };
        c_char_to_vec_string_free(call_res)
    }

    pub fn keys(&self) -> Result<Vec<String>, Utf8Error> {
        let call_res = unsafe {
            ll::taglib_property_keys(self.raw)
        };
        c_char_to_vec_string_free(call_res)
    }

    pub fn set_property(&mut self, key: &str, value: &str) {
        let cs = CString::new(key).unwrap();
        let s = cs.as_ptr();

        let vs = CString::new(value).unwrap();
        let v = vs.as_ptr();
        unsafe {
            ll::taglib_property_set(self.raw, s, v);
        }
    }

    pub fn set_append_property(&mut self, key: &str, value: &str) {
        let cs = CString::new(key).unwrap();
        let s = cs.as_ptr();

        let vs = CString::new(value).unwrap();
        let v = vs.as_ptr();
        unsafe {
            ll::taglib_property_set_append(self.raw, s, v);
        }
    }

    pub fn remove_property(&mut self, key: &str) {
        let cs = CString::new(key).unwrap();
        let s = cs.as_ptr();
        unsafe {
            ll::taglib_property_set(self.raw, s, ptr::null());
        }
    }

    /// Updates the meta-data of the file.
    pub fn save(&self) -> bool {
        unsafe { ll::taglib_file_save(self.raw) != 0 }
    }
}

fn text_pair_to_string(first: &Option<String>, last: &Option<String>) -> Option<String> {
    match (first, last) {
        (None, None) => None,
        (None, Some(b)) => Some("/".to_owned() + b),
        (Some(a), None) => Some(a.to_owned()),
        (Some(a), Some(b)) => Some(a.to_owned() + "/" + b),
    }
}

fn num_pair_to_string(first: &Option<u32>,
                      second: &Option<u32>,
                      padding: usize) -> Option<String> {
    match (first, second) {
        (None, None) => None,
        (None, Some(b)) => Some("/".to_owned() + &decimal_to_padding_string(*b, padding)),
        (Some(a), None) => Some(decimal_to_padding_string(*a, padding)),
        (Some(a), Some(b)) => {
            Some(decimal_to_padding_string(*a, padding) + "/"
                + &decimal_to_padding_string(*b, padding))
        }
    }
}

fn get_text_pair(text: &str) -> (Option<String>, Option<String>) {
    let mut split = text.split('/');
    let first = get_text(&split.next());
    let second = get_text(&split.next());
    (first, second)
}

fn get_text(input: &Option<&str>) -> Option<String> {
    if let Some(string) = input {
        let string = string.trim();
        if !string.is_empty() {
            return Some(string.to_owned());
        }
    }
    None
}

fn get_number_pair(text: &str) -> (Option<u32>, Option<u32>) {
    let mut split = text.split('/');
    let first = get_num(&split.next());
    let second = get_num(&split.next());
    (first, second)
}

fn get_num(input: &Option<&str>) -> Option<u32> {
    if let Some(string) = input {
        string.trim().parse::<u32>().ok()
    } else {
        None
    }
}

fn c_char_to_vec_string_free(ptr: *mut *mut c_char) -> Result<Vec<String>, Utf8Error> {
    if ptr.is_null() {
        Ok(Vec::new())
    } else {
        unsafe {
            let res = convert_double_pointer_to_vec(ptr);
            ll::taglib_property_free(ptr);
            res
        }
    }
}

unsafe fn convert_double_pointer_to_vec(data: *mut *mut c_char) -> Result<Vec<String>, Utf8Error> {
    let mut p = data;
    let mut res: Vec<String> = vec![];
    while p.as_ref().unwrap().as_ref().is_some() {
        let ele = CStr::from_ptr(p.as_ref().unwrap().as_ref().unwrap())
            .to_str().map(ToString::to_string)?;
        res.push(ele);
        p = p.byte_offset(MUT_PTR_C_CHAR_LEN as isize);
    }
    Ok(res)
}

fn decimal_to_padding_string(decimal: u32, padding: usize) -> String {
    format!("{:0width$}", decimal, width = padding)
}

/// Fixture creation:
/// ffmpeg -t 0.01 -f s16le -i /dev/zero test.mp3
/// kid3-cli -c 'set artist "Artist"' test.mp3
#[cfg(test)]
mod test {
    const TEST_MP3: &'static str = "fixtures/test.mp3";

    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_get_number_pair() {
        assert_eq!(get_number_pair(""), (None, None));
        assert_eq!(get_number_pair("/"), (None, None));
        assert_eq!(get_number_pair(" /  "), (None, None));

        assert_eq!(get_number_pair("2"), (Some(2), None));
        assert_eq!(get_number_pair("02"), (Some(2), None));
        assert_eq!(get_number_pair(" 2 "), (Some(2), None));
        assert_eq!(get_number_pair("12"), (Some(12), None));

        assert_eq!(get_number_pair("/15"), (None, Some(15)));
        assert_eq!(get_number_pair(" /15"), (None, Some(15)));

        assert_eq!(get_number_pair("02/15"), (Some(2), Some(15)));
    }

    #[test]
    fn test_get_text_pair() {
        assert_eq!(get_text_pair(""), (None, None));
        assert_eq!(get_text_pair("/"), (None, None));
        assert_eq!(get_text_pair(" /  "), (None, None));

        assert_eq!(get_text_pair("2"), (Some("2".to_owned()), None));
        assert_eq!(get_text_pair("02"), (Some("02".to_owned()), None));
        assert_eq!(get_text_pair(" 2 "), (Some("2".to_owned()), None));
        assert_eq!(get_text_pair("12"), (Some("12".to_owned()), None));

        assert_eq!(get_text_pair("/15"), (None, Some("15".to_owned())));
        assert_eq!(get_text_pair(" /15"), (None, Some("15".to_owned())));

        assert_eq!(get_text_pair("02/15"), (Some("02".to_owned()), Some("15".to_owned())));
    }

    #[test]
    fn test_get_tag() {
        let file = File::new(TEST_MP3).unwrap();
        let tag = file.tag().unwrap();
        assert_eq!(tag.artist().unwrap(), "Artist");
    }

    #[test]
    fn test_get_pathbuf() {
        let file = File::new(PathBuf::from(TEST_MP3)).unwrap();
        let tag = file.tag().unwrap();
        assert_eq!(tag.artist().unwrap(), "Artist");
    }

    #[test]
    fn test_get_no_tag() {
        let file = File::new(TEST_MP3).unwrap();
        let tag = file.tag().unwrap();
        assert_eq!(tag.album(), None);
    }

    #[test]
    fn test_get_tag_new_type() {
        let file = File::new_type(TEST_MP3, FileType::MPEG).unwrap();
        let tag = file.tag().unwrap();
        assert_eq!(tag.artist().unwrap(), "Artist");
    }

    #[test]
    fn test_get_audioproperties() {
        let file = File::new(TEST_MP3).unwrap();
        let ap = file.audioproperties().unwrap();
        assert_eq!(ap.length(), 0);
    }

    #[test]
    fn test_set_tag() {
        let temp_fn = "fixtures/temp.mp3";
        fs::copy(TEST_MP3, temp_fn).unwrap();
        let file = File::new(temp_fn).unwrap();
        let mut tag = file.tag().unwrap();
        tag.set_artist("Not Artist");
        assert_eq!(tag.artist().unwrap(), "Not Artist");

        file.save();

        let file = File::new(temp_fn).unwrap();
        let tag = file.tag().unwrap();
        assert_eq!(tag.artist().unwrap(), "Not Artist");

        fs::remove_file(temp_fn).unwrap();
    }
}
