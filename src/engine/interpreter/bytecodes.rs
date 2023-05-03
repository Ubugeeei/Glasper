#[allow(non_snake_case)]
pub(crate) mod Bytecodes {
    /* - [Loading the accumulator] */
    pub(crate) const Ldar: u8 = 0x01;
    pub(crate) const LdaZero: u8 = 0x02;
    pub(crate) const LdaSmi: u8 = 0x03;
    pub(crate) const LdaUndefined: u8 = 0x04;
    pub(crate) const LdaNull: u8 = 0x05;
    pub(crate) const LdaTheHole: u8 = 0x06;
    pub(crate) const LdaTrue: u8 = 0x07;
    pub(crate) const LdaFalse: u8 = 0x08;
    pub(crate) const LdaConstant: u8 = 0x09;
    pub(crate) const LdaContextSlot: u8 = 0x0a;
    pub(crate) const LdaImmutableContextSlot: u8 = 0x0b;
    pub(crate) const LdaCurrentContextSlot: u8 = 0x0c;
    pub(crate) const LdaImmutableCurrentContextSlot: u8 = 0x0d;

    /* - [Register Loads ] */
    pub(crate) const Mov: u8 = 0x0e;
    pub(crate) const Push: u8 = 0x0f;
    pub(crate) const Pop: u8 = 0x10;

    /* - [Test Operations ] */
    pub(crate) const TestReferenceEqual: u8 = 0x11;
    pub(crate) const TestNull: u8 = 0x12;
    pub(crate) const TestUndefined: u8 = 0x13;

    /* Binary Operators */
    pub(crate) const Add: u8 = 0x14;
    pub(crate) const Sub: u8 = 0x15;
    pub(crate) const Mul: u8 = 0x16;
    pub(crate) const Div: u8 = 0x17;
    pub(crate) const Mod: u8 = 0x18;
    pub(crate) const BitwiseOr: u8 = 0x19;
    pub(crate) const BitwiseXor: u8 = 0x1a;
    pub(crate) const BitwiseAnd: u8 = 0x1b;
    pub(crate) const ShiftLeft: u8 = 0x1c;
    pub(crate) const ShiftRight: u8 = 0x1d;
    pub(crate) const ShiftRightLogical: u8 = 0x1e;

    /* Binary operators with immediate operands */
    pub(crate) const AddSmi: u8 = 0x1f;
    pub(crate) const SubSmi: u8 = 0x20;
    pub(crate) const MulSmi: u8 = 0x21;
    pub(crate) const DivSmi: u8 = 0x22;
    pub(crate) const ModSmi: u8 = 0x23;
    pub(crate) const ExpSmi: u8 = 0x24;
    pub(crate) const BitwiseOrSmi: u8 = 0x25;
    pub(crate) const BitwiseXorSmi: u8 = 0x26;
    pub(crate) const BitwiseAndSmi: u8 = 0x27;
    pub(crate) const ShiftLeftSmi: u8 = 0x28;
    pub(crate) const ShiftRightSmi: u8 = 0x29;
    pub(crate) const ShiftRightLogicalSmi: u8 = 0x2a;

    /* Unary Operators */
    pub(crate) const Inc: u8 = 0x2b;
    pub(crate) const Dec: u8 = 0x2c;
    pub(crate) const Negate: u8 = 0x2d;
    pub(crate) const BitwiseNot: u8 = 0x2e;
    pub(crate) const TypeOf: u8 = 0x2f;

    /* Effectful Test Operators */
    pub(crate) const TestEqual: u8 = 0x30;
    pub(crate) const TestEqualStrict: u8 = 0x31;
    pub(crate) const TestLessThan: u8 = 0x32;
    pub(crate) const TestGreaterThan: u8 = 0x33;
    pub(crate) const TestLessThanOrEqual: u8 = 0x34;
    pub(crate) const TestGreaterThanOrEqual: u8 = 0x35;
    pub(crate) const TestInstanceOf: u8 = 0x36;
    pub(crate) const TestIn: u8 = 0x37;

    /* - [Unconditional jumps] */
    pub(crate) const JumpLoop: u8 = 0x38;
    /* - [Forward jumps] */
    pub(crate) const Jump: u8 = 0x39;
    /* - [Conditional jumps] */
    /* - [Conditional constant jumps] */
    pub(crate) const JumpIfNullConstant: u8 = 0x3a;
    pub(crate) const JumpIfNotNullConstant: u8 = 0x3b;
    pub(crate) const JumpIfUndefinedConstant: u8 = 0x3c;
    pub(crate) const JumpIfNotUndefinedConstant: u8 = 0x3d;
    pub(crate) const JumpIfUndefinedOrNullConstant: u8 = 0x3e;
    pub(crate) const JumpIfTrueConstant: u8 = 0x3f;
    pub(crate) const JumpIfFalseConstant: u8 = 0x40;
    pub(crate) const JumpIfTrue: u8 = 0x41;
    pub(crate) const JumpIfFalse: u8 = 0x42;
    pub(crate) const JumpIfNull: u8 = 0x43;
    pub(crate) const JumpIfNotNull: u8 = 0x44;
    pub(crate) const JumpIfUndefined: u8 = 0x45;
    pub(crate) const JumpIfNotUndefined: u8 = 0x46;
    pub(crate) const JumpIfUndefinedOrNull: u8 = 0x47;

    /* Call operations */
    pub(crate) const CallAnyReceiver: u8 = 0x48;
    pub(crate) const CallProperty: u8 = 0x49;

    pub(crate) const Return: u8 = 0x4a;
}
