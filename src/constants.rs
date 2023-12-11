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

pub const SRE_MAGIC: usize = 20221023;
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
    CATEGORY = 8,
    CHARSET = 9,
    BIGCHARSET = 10,
    GROUPREF = 11,
    GROUPREF_EXISTS = 12,
    IN = 13,
    INFO = 14,
    JUMP = 15,
    LITERAL = 16,
    MARK = 17,
    MAX_UNTIL = 18,
    MIN_UNTIL = 19,
    NOT_LITERAL = 20,
    NEGATE = 21,
    RANGE = 22,
    REPEAT = 23,
    REPEAT_ONE = 24,
    SUBPATTERN = 25,
    MIN_REPEAT_ONE = 26,
    ATOMIC_GROUP = 27,
    POSSESSIVE_REPEAT = 28,
    POSSESSIVE_REPEAT_ONE = 29,
    GROUPREF_IGNORE = 30,
    IN_IGNORE = 31,
    LITERAL_IGNORE = 32,
    NOT_LITERAL_IGNORE = 33,
    GROUPREF_LOC_IGNORE = 34,
    IN_LOC_IGNORE = 35,
    LITERAL_LOC_IGNORE = 36,
    NOT_LITERAL_LOC_IGNORE = 37,
    GROUPREF_UNI_IGNORE = 38,
    IN_UNI_IGNORE = 39,
    LITERAL_UNI_IGNORE = 40,
    NOT_LITERAL_UNI_IGNORE = 41,
    RANGE_UNI_IGNORE = 42,
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
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
