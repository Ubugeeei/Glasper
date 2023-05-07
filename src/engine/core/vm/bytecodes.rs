#![allow(non_upper_case_globals)]
#![allow(dead_code)]

#[allow(non_snake_case)]
pub(crate) mod Bytecodes {
    pub(crate) const Star0: u8 = 0x00;
    pub(crate) const Star1: u8 = 0x01;
    pub(crate) const Star2: u8 = 0x02;
    pub(crate) const Star3: u8 = 0x03;
    pub(crate) const Star4: u8 = 0x04;
    pub(crate) const Star5: u8 = 0x05;
    pub(crate) const Star6: u8 = 0x06;
    pub(crate) const Star7: u8 = 0x07;

    /* - [Loading the accumulator] */
    pub(crate) const Ldar: u8 = 0x10;
    pub(crate) const LdaZero: u8 = 0x11;
    pub(crate) const LdaSmi: u8 = 0x12;
    pub(crate) const LdaUndefined: u8 = 0x13;
    pub(crate) const LdaNull: u8 = 0x14;
    pub(crate) const LdaTheHole: u8 = 0x15;
    pub(crate) const LdaTrue: u8 = 0x16;
    pub(crate) const LdaFalse: u8 = 0x17;
    pub(crate) const LdaConstant: u8 = 0x18;
    pub(crate) const LdaContextSlot: u8 = 0x19; // implicit store to r0

    /* Property loads (LoadIC) operations */
    pub(crate) const GetNamedProperty: u8 = 0x1c;

    /* - [Register Loads ] */
    pub(crate) const Mov: u8 = 0x1d;
    pub(crate) const Push: u8 = 0x1e;
    pub(crate) const Pop: u8 = 0x1f;

    /* - [Test Operations ] */
    pub(crate) const TestReferenceEqual: u8 = 0x20;
    pub(crate) const TestNull: u8 = 0x21;
    pub(crate) const TestUndefined: u8 = 0x22;

    /* Binary Operators */
    pub(crate) const Add: u8 = 0x23;
    pub(crate) const Sub: u8 = 0x24;
    pub(crate) const Mul: u8 = 0x25;
    pub(crate) const Div: u8 = 0x26;
    pub(crate) const Mod: u8 = 0x27;
    pub(crate) const BitwiseOr: u8 = 0x28;
    pub(crate) const BitwiseXor: u8 = 0x29;
    pub(crate) const BitwiseAnd: u8 = 0x2a;
    pub(crate) const ShiftLeft: u8 = 0x2b;
    pub(crate) const ShiftRight: u8 = 0x2c;
    pub(crate) const ShiftRightLogical: u8 = 0x2d;
    /* Binary operators with immediate operands */
    pub(crate) const AddSmi: u8 = 0x2e;
    pub(crate) const SubSmi: u8 = 0x2f;
    pub(crate) const MulSmi: u8 = 0x30;
    pub(crate) const DivSmi: u8 = 0x31;
    pub(crate) const ModSmi: u8 = 0x32;
    pub(crate) const ExpSmi: u8 = 0x33;
    pub(crate) const BitwiseOrSmi: u8 = 0x34;
    pub(crate) const BitwiseXorSmi: u8 = 0x35;
    pub(crate) const BitwiseAndSmi: u8 = 0x36;
    pub(crate) const ShiftLeftSmi: u8 = 0x37;
    pub(crate) const ShiftRightSmi: u8 = 0x38;
    pub(crate) const ShiftRightLogicalSmi: u8 = 0x39;

    /* Unary Operators */
    pub(crate) const Inc: u8 = 0x3a;
    pub(crate) const Dec: u8 = 0x3b;
    pub(crate) const Negate: u8 = 0x3c;
    pub(crate) const BitwiseNot: u8 = 0x3d;
    pub(crate) const TypeOf: u8 = 0x3e;

    /* Effectful Test Operators */
    pub(crate) const TestEqual: u8 = 0x3f;
    pub(crate) const TestEqualStrict: u8 = 0x40;
    pub(crate) const TestLessThan: u8 = 0x41;
    pub(crate) const TestGreaterThan: u8 = 0x42;
    pub(crate) const TestLessThanOrEqual: u8 = 0x43;
    pub(crate) const TestGreaterThanOrEqual: u8 = 0x44;
    pub(crate) const TestInstanceOf: u8 = 0x45;
    pub(crate) const TestIn: u8 = 0x46;

    /* - [Unconditional jumps] */
    pub(crate) const JumpLoop: u8 = 0x47;
    /* - [Forward jumps] */
    pub(crate) const Jump: u8 = 0x48;
    /* - [Conditional jumps] */
    /* - [Conditional constant jumps] */
    pub(crate) const JumpIfNullConstant: u8 = 0x49;
    pub(crate) const JumpIfNotNullConstant: u8 = 0x4a;
    pub(crate) const JumpIfUndefinedConstant: u8 = 0x4b;
    pub(crate) const JumpIfNotUndefinedConstant: u8 = 0x4c;
    pub(crate) const JumpIfUndefinedOrNullConstant: u8 = 0x4d;
    pub(crate) const JumpIfTrueConstant: u8 = 0x4e;
    pub(crate) const JumpIfFalseConstant: u8 = 0x4f;
    pub(crate) const JumpIfTrue: u8 = 0x50;
    pub(crate) const JumpIfFalse: u8 = 0x51;
    pub(crate) const JumpIfNull: u8 = 0x52;
    pub(crate) const JumpIfNotNull: u8 = 0x53;
    pub(crate) const JumpIfUndefined: u8 = 0x54;
    pub(crate) const JumpIfNotUndefined: u8 = 0x55;
    pub(crate) const JumpIfUndefinedOrNull: u8 = 0x56;

    /* Call operations */
    pub(crate) const CallAnyReceiver: u8 = 0x57;
    pub(crate) const CallProperty: u8 = 0x58;

    pub(crate) const Return: u8 = 0x59;
    pub(crate) const Hlt: u8 = 0x5a;

    pub(crate) const Construct: u8 = 0x5b; // implicit store to r0
    pub(crate) const StaContextSlot: u8 = 0x5c; // implicit load r0 and store to context slot
}

#[allow(non_snake_case)]
pub(crate) mod RName {
    pub(crate) const R0: u8 = 0x00;
    pub(crate) const R1: u8 = 0x01;
    pub(crate) const R2: u8 = 0x02;
    pub(crate) const R3: u8 = 0x03;
    pub(crate) const R4: u8 = 0x04;
    pub(crate) const R5: u8 = 0x05;
    pub(crate) const R6: u8 = 0x06;
    pub(crate) const R7: u8 = 0x07;
}
