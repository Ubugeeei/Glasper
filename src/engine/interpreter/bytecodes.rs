#[derive(Clone, Copy)]
pub(crate) enum Bytecodes {
    /* - [Loading the accumulator] */
    Ldar,
    LdaZero,
    LdaSmi,
    LdaUndefined,
    LdaNull,
    LdaTheHole,
    LdaTrue,
    LdaFalse,
    LdaConstant,
    LdaContextSlot,
    LdaImmutableContextSlot,
    LdaCurrentContextSlot,
    LdaImmutableCurrentContextSlot,

    /* - [Register Loads ] */
    Mov,
    PushContext,
    PopContext,

    /* - [Test Operations ] */
    TestReferenceEqual,
    TestNull,
    TestUndefined,

    /* Binary Operators */
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    ShiftLeft,
    ShiftRight,
    ShiftRightLogical,

    /* Binary operators with immediate operands */
    AddSmi,
    SubSmi,
    MulSmi,
    DivSmi,
    ModSmi,
    ExpSmi,
    BitwiseOrSmi,
    BitwiseXorSmi,
    BitwiseAndSmi,
    ShiftLeftSmi,
    ShiftRightSmi,
    ShiftRightLogicalSmi,

    /* Unary Operators */
    Inc,
    Dec,
    Negate,
    BitwiseNot,
    TypeOf,

    /* Effectful Test Operators */
    TestEqual,
    TestEqualStrict,
    TestLessThan,
    TestGreaterThan,
    TestLessThanOrEqual,
    TestGreaterThanOrEqual,
    TestInstanceOf,
    TestIn,

    /* - [Unconditional jumps] */
    JumpLoop,
    /* - [Forward jumps] */
    Jump,
    /* - [Conditional jumps] */
    /* - [Conditional constant jumps] */
    JumpIfNullConstant,
    JumpIfNotNullConstant,
    JumpIfUndefinedConstant,
    JumpIfNotUndefinedConstant,
    JumpIfUndefinedOrNullConstant,
    JumpIfTrueConstant,
    JumpIfFalseConstant,
    JumpIfTrue,
    JumpIfFalse,
    JumpIfNull,
    JumpIfNotNull,
    JumpIfUndefined,
    JumpIfNotUndefined,
    JumpIfUndefinedOrNull,

    /* Call operations */
    CallAnyReceiver,
    CallProperty,

    Return,
}

impl Bytecodes {
    pub fn as_byte(&self) -> u8 {
        self.clone() as u8
    }
}
