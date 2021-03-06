/*
 * Secret Labs' Regular Expression Engine
 *
 * regular expression matching engine
 *
 * NOTE: This file is generated by sre_constants.py.  If you need
 * to change anything in here, edit sre_constants.py and run it.
 *
 * Copyright (c) 1997-2001 by Secret Labs AB.  All rights reserved.
 *
 * See the _sre.c file for information on usage and redistribution.
 */

use bitflags::bitflags;

pub const SRE_MAGIC: usize = 20171005;
#[derive(num_enum::TryFromPrimitive, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SreOpcode {
    FAILURE = 0,
    SUCCESS = 1,
    ANY = 2,
    ANY_ALL = 3,
    ASSERT = 4,
    ASSERT_NOT = 5,
    AT = 6,
    BRANCH = 7,
    CALL = 8,
    CATEGORY = 9,
    CHARSET = 10,
    BIGCHARSET = 11,
    GROUPREF = 12,
    GROUPREF_EXISTS = 13,
    IN = 14,
    INFO = 15,
    JUMP = 16,
    LITERAL = 17,
    MARK = 18,
    MAX_UNTIL = 19,
    MIN_UNTIL = 20,
    NOT_LITERAL = 21,
    NEGATE = 22,
    RANGE = 23,
    REPEAT = 24,
    REPEAT_ONE = 25,
    SUBPATTERN = 26,
    MIN_REPEAT_ONE = 27,
    GROUPREF_IGNORE = 28,
    IN_IGNORE = 29,
    LITERAL_IGNORE = 30,
    NOT_LITERAL_IGNORE = 31,
    GROUPREF_LOC_IGNORE = 32,
    IN_LOC_IGNORE = 33,
    LITERAL_LOC_IGNORE = 34,
    NOT_LITERAL_LOC_IGNORE = 35,
    GROUPREF_UNI_IGNORE = 36,
    IN_UNI_IGNORE = 37,
    LITERAL_UNI_IGNORE = 38,
    NOT_LITERAL_UNI_IGNORE = 39,
    RANGE_UNI_IGNORE = 40,
}
#[derive(num_enum::TryFromPrimitive, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SreAtCode {
    BEGINNING = 0,
    BEGINNING_LINE = 1,
    BEGINNING_STRING = 2,
    BOUNDARY = 3,
    NON_BOUNDARY = 4,
    END = 5,
    END_LINE = 6,
    END_STRING = 7,
    LOC_BOUNDARY = 8,
    LOC_NON_BOUNDARY = 9,
    UNI_BOUNDARY = 10,
    UNI_NON_BOUNDARY = 11,
}
#[derive(num_enum::TryFromPrimitive, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SreCatCode {
    DIGIT = 0,
    NOT_DIGIT = 1,
    SPACE = 2,
    NOT_SPACE = 3,
    WORD = 4,
    NOT_WORD = 5,
    LINEBREAK = 6,
    NOT_LINEBREAK = 7,
    LOC_WORD = 8,
    LOC_NOT_WORD = 9,
    UNI_DIGIT = 10,
    UNI_NOT_DIGIT = 11,
    UNI_SPACE = 12,
    UNI_NOT_SPACE = 13,
    UNI_WORD = 14,
    UNI_NOT_WORD = 15,
    UNI_LINEBREAK = 16,
    UNI_NOT_LINEBREAK = 17,
}
bitflags! {
    pub struct SreFlag: u16 {
        const TEMPLATE = 1;
        const IGNORECASE = 2;
        const LOCALE = 4;
        const MULTILINE = 8;
        const DOTALL = 16;
        const UNICODE = 32;
        const VERBOSE = 64;
        const DEBUG = 128;
        const ASCII = 256;
    }
}
bitflags! {
    pub struct SreInfo: u32 {
        const PREFIX = 1;
        const LITERAL = 2;
        const CHARSET = 4;
    }
}
