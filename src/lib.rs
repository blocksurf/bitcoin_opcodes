#![cfg_attr(not(feature = "std"), no_std)]

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "std", derive(strum::EnumString))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[allow(non_camel_case_types)]
#[repr(u8)]

/**
 * Majority of this page is borrowed from
 *          bsv-wasm (https://github.com/Firaenix/bsv-wasm/blob/master/src/script/op_codes.rs)
 *          & rust-sv (https://github.com/brentongunning/rust-sv/blob/master/src/script/op_codes.rs)
 */

// --------------------------------------------------------------------------------------------
// Constants
// --------------------------------------------------------------------------------------------
pub enum OpCode {
    /// Pushes 0 onto the stack
    #[cfg_attr(feature = "std", strum(serialize = "OP_FALSE", serialize = "OP_0"))]
    OP_0 = 0,

    /// The next byte sets the number of bytes to push onto the stack
    OP_PUSHDATA1 = 76,
    /// The next two bytes sets the number of bytes to push onto the stack
    OP_PUSHDATA2 = 77,
    /// The next four bytes sets the number of bytes to push onto the stack
    OP_PUSHDATA4 = 78,

    /// Pushes -1 onto the stack
    OP_1NEGATE = 79,
    /// Pushes 1 onto the stack
    #[cfg_attr(feature = "std", strum(serialize = "OP_TRUE", serialize = "OP_1"))]
    OP_1 = 81,
    /// Pushes 2 onto the stack
    OP_2 = 82,
    /// Pushes 3 onto the stack
    OP_3 = 83,
    /// Pushes 4 onto the stack
    OP_4 = 84,
    /// Pushes 5 onto the stack
    OP_5 = 85,
    /// Pushes 6 onto the stack
    OP_6 = 86,
    /// Pushes 7 onto the stack
    OP_7 = 87,
    /// Pushes 8 onto the stack
    OP_8 = 88,
    /// Pushes 9 onto the stack
    OP_9 = 89,
    /// Pushes 10 onto the stack
    OP_10 = 90,
    /// Pushes 11 onto the stack
    OP_11 = 91,
    /// Pushes 12 onto the stack
    OP_12 = 92,
    /// Pushes 13 onto the stack
    OP_13 = 93,
    /// Pushes 14 onto the stack
    OP_14 = 94,
    /// Pushes 15 onto the stack
    OP_15 = 95,
    /// Pushes 16 onto the stack
    OP_16 = 96,

    // --------------------------------------------------------------------------------------------
    // Flow Control
    // --------------------------------------------------------------------------------------------
    /// Does nothing
    OP_NOP = 97,
    /// If the top stack is true, statements are executed. Top stack value is removed.
    OP_IF = 99,
    /// If the top stack is false, statements are executed. Top stack value is removed.
    OP_NOTIF = 100,
    /// If the preceding OP_IF or OP_NOTIF statemetns were not executed, then statements are executed.
    OP_ELSE = 103,
    /// Ends an if-else block
    OP_ENDIF = 104,
    /// Marks a statement as invalid if the top stack value is false. Top stack value is removed.
    OP_VERIFY = 105,
    /// Marks a statements as invalid
    OP_RETURN = 106,

    // --------------------------------------------------------------------------------------------
    // Stack
    // --------------------------------------------------------------------------------------------
    /// Moves the top item on the main stack to the alt stack
    OP_TOALTSTACK = 107,
    /// Moves the top item on the alt stack to the main stack
    OP_FROMALTSTACK = 108,
    /// Duplicates the top stack value if it is not zero
    OP_IFDUP = 115,
    /// Puts the number of stack items onto the stack
    OP_DEPTH = 116,
    /// Drops the top stack value
    OP_DROP = 117,
    /// Duplicates the top stack item
    OP_DUP = 118,
    /// Removes the second-to-top stack item
    OP_NIP = 119,
    /// Copies the second-to-top stack item to the top
    OP_OVER = 120,
    /// The item n back in the stack is copied to the top
    OP_PICK = 121,
    /// The item n back in the stack is moved to the top
    OP_ROLL = 122,
    /// The top three items on the stack are rotated to the left
    OP_ROT = 123,
    /// The top two items on the stack are swapped
    OP_SWAP = 124,
    /// The item at the top of the stack is copied and inserted before the second-to-top item
    OP_TUCK = 125,
    /// Removes the top two items from the stack
    OP_2DROP = 109,
    /// Duplicates the top two stack items
    OP_2DUP = 110,
    /// Duplicates the top three stack items
    OP_3DUP = 111,
    /// Copies the pair of items two spaces back to the front
    OP_2OVER = 112,
    /// The fifth and sixth items back are moved to the top of the stack
    OP_2ROT = 113,
    /// Swaps the top two pairs of items
    OP_2SWAP = 114,

    // --------------------------------------------------------------------------------------------
    // Splice
    // --------------------------------------------------------------------------------------------
    /// Concatenates two byte sequences
    OP_CAT = 126,
    /// Splits the byte sequence at position n
    OP_SPLIT = 127,
    /// Pushes the byte sequence length of the top stack item without popping it
    OP_SIZE = 130,

    // --------------------------------------------------------------------------------------------
    // Bitwise Logic
    // --------------------------------------------------------------------------------------------
    /// Flips all of the bits in the input
    OP_INVERT = 131,
    /// Boolean and between each bit in the inputs
    OP_AND = 132,
    /// Boolean or between each bit in the inputs
    OP_OR = 133,
    /// Boolean exclusive or between each bit in the inputs
    OP_XOR = 134,
    /// Returns 1 if the inputs are exactly equal, 0 otherwise
    OP_EQUAL = 135,
    /// Same as OP_EQUAL, but runs OP_VERIFY afterward
    OP_EQUALVERIFY = 136,

    // --------------------------------------------------------------------------------------------
    // Arithmetic
    // --------------------------------------------------------------------------------------------
    /// Adds 1 to the input
    OP_1ADD = 139,
    /// Subtracts 1 from the input
    OP_1SUB = 140,
    /// The sign of the input is flipped
    OP_NEGATE = 143,
    /// The input is made positive
    OP_ABS = 144,
    /// If the input is 0 or 1, it is flipped. Otherwise, the output will be 0.
    OP_NOT = 145,
    /// Returns 0 if the input is 0. 1 otherwise.
    OP_0NOTEQUAL = 146,
    /// Adds a to b
    OP_ADD = 147,
    /// Subtracts b from a
    OP_SUB = 148,
    /// Multiplies a by b
    OP_MUL = 149,
    /// Divides a by b
    OP_DIV = 150,
    /// Returns the remainder after dividing a by b
    OP_MOD = 151,
    /// Shifts a left b bits, preserving sign
    OP_LSHIFT = 152,
    /// Shifts a right b bits, preserving sign
    OP_RSHIFT = 153,
    /// If both a and b are not empty, the output is 1. Otherwise, 0.
    OP_BOOLAND = 154,
    /// If a or b is not empty, the output is 1. Otherwise, 0.
    OP_BOOLOR = 155,
    /// Returns 1 if the numbers are equal. Otherwise, 0.
    OP_NUMEQUAL = 156,
    /// Same as OP_NUMEQUAL, but runs OP_VERIFY afterward
    OP_NUMEQUALVERIFY = 157,
    /// Returns 1 if the numbers are not equal. Otherwise, 0.
    OP_NUMNOTEQUAL = 158,
    /// Returns 1 if a is less than b. Otherwise, 0.
    OP_LESSTHAN = 159,
    /// Returns 1 if a is greater than b. Otherwise, 0.
    OP_GREATERTHAN = 160,
    /// Returns 1 if a is less than or equal to b. Otherwise, 0.
    OP_LESSTHANOREQUAL = 161,
    /// Returns 1 if a is greater than or equal to b. Otherwise, 0.
    OP_GREATERTHANOREQUAL = 162,
    /// Returns the smaller of a and b
    OP_MIN = 163,
    /// Returns the larger of a and b
    OP_MAX = 164,
    /// Returns 1 if x is within the specified range, left inclusive. Otherwise, 0.
    OP_WITHIN = 165,
    /// Converts numeric value a into a byte sequence of length b
    OP_NUM2BIN = 128,
    /// Converts byte sequence x into a numeric value
    OP_BIN2NUM = 129,

    // --------------------------------------------------------------------------------------------
    // Cryptography
    // --------------------------------------------------------------------------------------------
    /// The input is hashed using RIPEMD-160
    OP_RIPEMD160 = 166,
    /// The input is hashed using SHA-1
    OP_SHA1 = 167,
    /// The input is hashed using SHA-256
    OP_SHA256 = 168,
    /// The input is hashed twice: first with SHA-256 and then with RIPEMD-160
    OP_HASH160 = 169,
    /// The input is hashed two times with SHA-256
    OP_HASH256 = 170,
    /// Marks the part of the script after which the signature will begin matching
    OP_CODESEPARATOR = 171,
    /// Puts 1 on the stack if the signature authorizes the public key and transaction hash. Otherwise 0.
    OP_CHECKSIG = 172,
    /// Same as OP_CHECKSIG, but OP_VERIFY is executed afterward
    OP_CHECKSIGVERIFY = 173,
    /// Puts 1 on the stack if m of n signatures authorize the public key and transaction hash. Otherwise 0.
    OP_CHECKMULTISIG = 174,
    /// Same as OP_CHECKMULTISIG, but OP_VERIFY is executed afterward
    OP_CHECKMULTISIGVERIFY = 175,

    // --------------------------------------------------------------------------------------------
    // Locktime
    // --------------------------------------------------------------------------------------------
    /// Marks transaction as invalid if the top stack item is greater than the transaction's lock_time
    OP_CHECKLOCKTIMEVERIFY = 177,
    /// Marks transaction as invalid if the top stack item is less than the transaction's sequence used for relative lock time
    OP_CHECKSEQUENCEVERIFY = 178,

    // --------------------------------------------------------------------------------------------
    // Pseudo-words
    // --------------------------------------------------------------------------------------------
    /// OP_DATA followed by a varint represents arbitrary data on chain. Used for matching Script Templates.
    OP_DATA = 251,
    /// Represents a secp256k1 signature
    OP_SIG = 252,
    /// Represents a public key hashed with OP_HASH160
    OP_PUBKEYHASH = 253,
    /// Represents a public key compatible with OP_CHECKSIG
    OP_PUBKEY = 254,
    /// Matches any opcode that is not yet assigned
    OP_INVALIDOPCODE = 255,

    // --------------------------------------------------------------------------------------------
    // Reserved words
    // --------------------------------------------------------------------------------------------
    /// Transaction is invalid unless occuring in an unexecuted OP_IF branch
    OP_RESERVED = 80,
    /// Transaction is invalid unless occuring in an unexecuted OP_IF branch
    OP_VER = 98,
    /// Transaction is invalid even when occuring in an unexecuted OP_IF branch
    OP_VERIF = 101,
    /// Transaction is invalid even when occuring in an unexecuted OP_IF branch
    OP_VERNOTIF = 102,
    /// Transaction is invalid unless occuring in an unexecuted OP_IF branch
    OP_RESERVED1 = 137,
    /// Transaction is invalid unless occuring in an unexecuted OP_IF branch
    OP_RESERVED2 = 138,
    /// The word is ignored. Does not mark transaction as invalid.
    OP_NOP1 = 176,
    /// The word is ignored. Does not mark transaction as invalid.
    OP_NOP4 = 179,
    /// The word is ignored. Does not mark transaction as invalid.
    OP_NOP5 = 180,
    /// The word is ignored. Does not mark transaction as invalid.
    OP_NOP6 = 181,
    /// The word is ignored. Does not mark transaction as invalid.
    OP_NOP7 = 182,
    /// The word is ignored. Does not mark transaction as invalid.
    OP_NOP8 = 183,
    /// The word is ignored. Does not mark transaction as invalid.
    OP_NOP9 = 184,
    /// The word is ignored. Does not mark transaction as invalid.
    OP_NOP10 = 185,

    /// Words at or above this number are invalid
    // #[num_enum(default)]
    OP_INVALID_ABOVE = 186,

    // --------------------------------------------------------------------------------------------
    // Disabled words
    // --------------------------------------------------------------------------------------------
    /// The input is multiplied by 2
    OP_2MUL = 141,
    /// The input is divided by 2
    OP_2DIV = 142,
}

impl core::fmt::Display for OpCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

macro_rules! impl_opcode {
    ($($type:ty),+) => {
        $(

            impl PartialEq<$type> for OpCode {
                fn eq(&self, other: &$type) -> bool {
                    *self as $type == *other
                }
            }

            impl From<OpCode> for $type {
                fn from(val: OpCode) -> Self {
                    val as $type
                }
            }

            impl From<$type> for OpCode {
                fn from(value: $type) -> Self {
                    match value {
                        0 => OpCode::OP_0,
                        76 => OpCode::OP_PUSHDATA1,
                        77 => OpCode::OP_PUSHDATA2,
                        78 => OpCode::OP_PUSHDATA4,
                        79 => OpCode::OP_1NEGATE,
                        81 => OpCode::OP_1,
                        82 => OpCode::OP_2,
                        83 => OpCode::OP_3,
                        84 => OpCode::OP_4,
                        85 => OpCode::OP_5,
                        86 => OpCode::OP_6,
                        87 => OpCode::OP_7,
                        88 => OpCode::OP_8,
                        89 => OpCode::OP_9,
                        90 => OpCode::OP_10,
                        91 => OpCode::OP_11,
                        92 => OpCode::OP_12,
                        93 => OpCode::OP_13,
                        94 => OpCode::OP_14,
                        95 => OpCode::OP_15,
                        96 => OpCode::OP_16,
                        97 => OpCode::OP_NOP,
                        99 => OpCode::OP_IF,
                        100 => OpCode::OP_NOTIF,
                        103 => OpCode::OP_ELSE,
                        104 => OpCode::OP_ENDIF,
                        105 => OpCode::OP_VERIFY,
                        106 => OpCode::OP_RETURN,
                        107 => OpCode::OP_TOALTSTACK,
                        108 => OpCode::OP_FROMALTSTACK,
                        109 => OpCode::OP_2DROP,
                        110 => OpCode::OP_2DUP,
                        111 => OpCode::OP_3DUP,
                        112 => OpCode::OP_2OVER,
                        113 => OpCode::OP_2ROT,
                        114 => OpCode::OP_2SWAP,
                        115 => OpCode::OP_IFDUP,
                        116 => OpCode::OP_DEPTH,
                        117 => OpCode::OP_DROP,
                        118 => OpCode::OP_DUP,
                        119 => OpCode::OP_NIP,
                        120 => OpCode::OP_OVER,
                        121 => OpCode::OP_PICK,
                        122 => OpCode::OP_ROLL,
                        123 => OpCode::OP_ROT,
                        124 => OpCode::OP_SWAP,
                        125 => OpCode::OP_TUCK,
                        126 => OpCode::OP_CAT,
                        127 => OpCode::OP_SPLIT,
                        130 => OpCode::OP_SIZE,
                        131 => OpCode::OP_INVERT,
                        132 => OpCode::OP_AND,
                        133 => OpCode::OP_OR,
                        134 => OpCode::OP_XOR,
                        135 => OpCode::OP_EQUAL,
                        136 => OpCode::OP_EQUALVERIFY,
                        139 => OpCode::OP_1ADD,
                        140 => OpCode::OP_1SUB,
                        143 => OpCode::OP_NEGATE,
                        144 => OpCode::OP_ABS,
                        145 => OpCode::OP_NOT,
                        146 => OpCode::OP_0NOTEQUAL,
                        147 => OpCode::OP_ADD,
                        148 => OpCode::OP_SUB,
                        149 => OpCode::OP_MUL,
                        150 => OpCode::OP_DIV,
                        151 => OpCode::OP_MOD,
                        152 => OpCode::OP_LSHIFT,
                        153 => OpCode::OP_RSHIFT,
                        154 => OpCode::OP_BOOLAND,
                        155 => OpCode::OP_BOOLOR,
                        156 => OpCode::OP_NUMEQUAL,
                        157 => OpCode::OP_NUMEQUALVERIFY,
                        158 => OpCode::OP_NUMNOTEQUAL,
                        159 => OpCode::OP_LESSTHAN,
                        160 => OpCode::OP_GREATERTHAN,
                        161 => OpCode::OP_LESSTHANOREQUAL,
                        162 => OpCode::OP_GREATERTHANOREQUAL,
                        163 => OpCode::OP_MIN,
                        164 => OpCode::OP_MAX,
                        165 => OpCode::OP_WITHIN,
                        128 => OpCode::OP_NUM2BIN,
                        129 => OpCode::OP_BIN2NUM,
                        166 => OpCode::OP_RIPEMD160,
                        167 => OpCode::OP_SHA1,
                        168 => OpCode::OP_SHA256,
                        169 => OpCode::OP_HASH160,
                        170 => OpCode::OP_HASH256,
                        171 => OpCode::OP_CODESEPARATOR,
                        172 => OpCode::OP_CHECKSIG,
                        173 => OpCode::OP_CHECKSIGVERIFY,
                        174 => OpCode::OP_CHECKMULTISIG,
                        175 => OpCode::OP_CHECKMULTISIGVERIFY,
                        177 => OpCode::OP_CHECKLOCKTIMEVERIFY,
                        178 => OpCode::OP_CHECKSEQUENCEVERIFY,
                        179 => OpCode::OP_NOP4,
                        180 => OpCode::OP_NOP5,
                        181 => OpCode::OP_NOP6,
                        182 => OpCode::OP_NOP7,
                        183 => OpCode::OP_NOP8,
                        184 => OpCode::OP_NOP9,
                        185 => OpCode::OP_NOP10,
                        186..=250 => OpCode::OP_INVALID_ABOVE,
                        251 => OpCode::OP_DATA,
                        252 => OpCode::OP_SIG,
                        253 => OpCode::OP_PUBKEYHASH,
                        254 => OpCode::OP_PUBKEY,
                        255 => OpCode::OP_INVALIDOPCODE,
                        _ => OpCode::OP_INVALIDOPCODE,
                    }
                }
            }
        )+
    }
}

impl_opcode!(u8, u16, u32, u64);

#[cfg(test)]
mod tests {
    use super::OpCode;
    use OpCode::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_op_code_deserialization() {
        use std::str::FromStr;

        let op_code = OpCode::from_str("OP_0").unwrap();
        assert_eq!(op_code, OP_0);

        let op_code = OpCode::from_str("OP_FALSE").unwrap();
        assert_eq!(op_code, OP_0);

        let op_code = OpCode::from_str("OP_1").unwrap();
        assert_eq!(op_code, OP_1);

        let op_code = OpCode::from_str("OP_TRUE").unwrap();
        assert_eq!(op_code, OP_1);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_op_code_serialization() {
        use std::str::FromStr;

        assert_eq!(OP_0.to_string(), "OP_0");
        assert_eq!(OP_1.to_string(), "OP_1");
        assert_eq!(OpCode::from_str("OP_FALSE").unwrap().to_string(), "OP_0");
        assert_eq!(OpCode::from_str("OP_TRUE").unwrap().to_string(), "OP_1");
    }

    #[test]
    fn test_invalid_opcodes_for_u16() {
        for i in 0x100..u16::MAX {
            let op = OpCode::from(i);
            assert_eq!(op, OP_INVALIDOPCODE);
        }
    }

    #[test]
    fn test_invalid_opcodes_for_u32() {
        // lowest range
        for i in 0x100..0x110u32 {
            let op = OpCode::from(i);
            assert_eq!(op, OP_INVALIDOPCODE);
        }

        // upper range
        for i in u32::MAX - 10..u32::MAX {
            let op = OpCode::from(i);
            assert_eq!(op, OP_INVALIDOPCODE);
        }
    }

    #[test]
    fn test_invalid_opcodes_for_u64() {
        // lowest range
        for i in 0x100..0x110u64 {
            let op = OpCode::from(i);
            assert_eq!(op, OP_INVALIDOPCODE);
        }

        // upper range
        for i in u64::MAX - 10..u64::MAX {
            let op = OpCode::from(i);
            assert_eq!(op, OP_INVALIDOPCODE);
        }
    }
}
