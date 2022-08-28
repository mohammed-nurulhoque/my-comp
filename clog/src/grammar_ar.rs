// auto-generated: "lalrpop 0.19.6"
// sha3: c33c41de143d6d3f9a99eece92bb5a9dee91798575acffbb8a5915881375b
use crate::{
    ast::*,
    types::{ProtoType, Literal, BinOpcode, UnOpcode}
};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::{
    ast::*,
    types::{ProtoType, Literal, BinOpcode, UnOpcode}
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(__lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>),
        Variant2(core::option::Option<&'input str>),
        Variant3((alloc::vec::Vec<Pattern<'input>>, Expr<'input>)),
        Variant4(alloc::vec::Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>),
        Variant5(Expr<'input>),
        Variant6(alloc::vec::Vec<Expr<'input>>),
        Variant7(alloc::vec::Vec<&'input str>),
        Variant8(Pattern<'input>),
        Variant9(alloc::vec::Vec<Pattern<'input>>),
        Variant10(ProtoType<'input>),
        Variant11(alloc::vec::Vec<ProtoType<'input>>),
        Variant12(usize),
        Variant13(core::option::Option<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>),
        Variant14(Vec<Expr<'input>>),
        Variant15(Vec<Pattern<'input>>),
        Variant16(Vec<ProtoType<'input>>),
        Variant17(Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>),
        Variant18(Vec<&'input str>),
        Variant19(Binding<'input>),
        Variant20(Literal<'input>),
        Variant21(BinOpcode),
        Variant22(UnOpcode),
        Variant23(Vec<Binding<'input>>),
        Variant24(alloc::vec::Vec<Binding<'input>>),
        Variant25((&'input str, ProtoType<'input>)),
        Variant26(alloc::vec::Vec<(&'input str, ProtoType<'input>)>),
        Variant27(core::option::Option<Vec<&'input str>>),
    }
    const __ACTION: &[i16] = &[
        // State 0
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 1
        -120, 9, 39, -120, -120, -120, -120, -120, -120, -120, 0, -120, -120, -120, -120, -120, -120, -120, 0, 10, 0, -120, -120, -120, -120, -120, -120, 0, 41, 0, 0, 42, 11, 0, 0, 0, 0, -120, 44, 45, 46, 47,
        // State 2
        0, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58, -58, 0, 0, 0, -58, -58, 49, -58, -58, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, 0,
        // State 3
        0, 0, 0, -59, 0, 0, 0, 0, 0, 51, 0, 0, 0, 52, 53, 54, -59, -59, 0, 0, 0, -59, -59, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, 0,
        // State 4
        0, 0, 0, -60, 0, 0, 0, 0, 0, -60, 0, 55, 56, -60, -60, -60, -60, -60, 0, 0, 0, -60, -60, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0,
        // State 5
        0, 0, 0, -61, 0, 57, 58, 59, 0, -61, 0, -61, -61, -61, -61, -61, -61, -61, 0, 0, 0, -61, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0,
        // State 6
        60, 0, 0, -62, 61, -62, -62, -62, 62, -62, 0, -62, -62, -62, -62, -62, -62, -62, 0, 0, 0, -62, -62, -62, -62, -62, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, 0,
        // State 7
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 8
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 9
        0, 22, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 0, 0, 0, 0, 0, 0, 44, 45, 46, 72,
        // State 10
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 11
        -121, 9, 39, -121, -121, -121, -121, -121, -121, -121, 0, -121, -121, -121, -121, -121, -121, -121, 0, 10, 0, -121, -121, -121, -121, -121, -121, 0, 41, 0, 0, 42, 11, 0, 0, 0, 0, -121, 44, 45, 46, 47,
        // State 12
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 13
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 14
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 15
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 16
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 17
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 18
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 19
        0, 22, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, -52, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 0, 0, 0, 0, 0, 0, 44, 45, 46, 72,
        // State 20
        0, 22, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 0, 0, 0, 0, 0, 0, 44, 45, 46, 72,
        // State 21
        0, 22, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 0, 0, 0, 0, 0, 0, 44, 45, 46, 72,
        // State 22
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 23
        0, 22, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 0, 0, 0, 0, 0, 0, 44, 45, 46, 72,
        // State 24
        0, 22, 39, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, -91, 0, 0, 0, 0, 0, 41, 0, 0, 42, 0, 0, 0, 0, 0, 0, 44, 45, 46, 72,
        // State 25
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 26
        0, 9, 39, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 42, 11, 43, 0, 0, 0, 0, 44, 45, 46, 47,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, -57, 0, 0, 0, -57, -57, 0, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0,
        // State 29
        0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, 0, 0, 0, -35, -35, -35, -35, -35, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, 0,
        // State 30
        0, 0, 0, -37, 0, 0, 0, 0, 0, -37, 0, 0, 0, -37, -37, -37, -37, -37, 0, 0, 0, -37, -37, -37, -37, -37, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0,
        // State 31
        0, 0, 0, -39, 0, 0, 0, 0, 0, -39, 0, -39, -39, -39, -39, -39, -39, -39, 0, 0, 0, -39, -39, -39, -39, -39, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0,
        // State 32
        0, 0, 0, -41, 0, -41, -41, -41, 0, -41, 0, -41, -41, -41, -41, -41, -41, -41, 0, 0, 0, -41, -41, -41, -41, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0,
        // State 33
        -43, 0, 0, -43, -43, -43, -43, -43, -43, -43, 0, -43, -43, -43, -43, -43, -43, -43, 0, 0, 0, -43, -43, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0,
        // State 34
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, 0, -26, 0, -26, -26, -26, -26, -26, -26, 0, -26, 0, 0, -26, -26, 0, 0, 0, 0, -26, -26, -26, -26, -26,
        // State 35
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, -25, -25, -25, -25, -25, -25, 0, -25, 0, -25, -25, -25, -25, -25, -25, 0, -25, 0, 0, -25, -25, 0, 0, 0, 0, -25, -25, -25, -25, -25,
        // State 36
        -132, 0, 0, -132, -132, -132, -132, -132, -132, -132, 0, -132, -132, -132, -132, -132, -132, -132, 0, 0, 0, -132, -132, -132, -132, -132, -132, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -132, 0, 0, 0, 0,
        // State 37
        -63, 0, 0, -63, -63, -63, -63, -63, -63, -63, 0, -63, -63, -63, -63, -63, -63, -63, 0, 0, 0, -63, -63, -63, -63, -63, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, 0, 0, 0, 0,
        // State 38
        -72, -72, -72, -72, -72, -72, -72, -72, -72, -72, 0, -72, -72, -72, -72, -72, -72, -72, -72, -72, 0, -72, -72, -72, -72, -72, -72, 0, -72, 0, 0, -72, -72, 0, 0, 0, 0, -72, -72, -72, -72, -72,
        // State 39
        0, -88, -88, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, -88, 0, 0, -88, -88, -88, 0, 0, 0, 0, -88, -88, -88, -88,
        // State 40
        -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, 0, -71, -71, -71, -71, -71, -71, -71, -71, -71, 0, -71, -71, -71, -71, -71, -71, 0, -71, 0, 0, -71, -71, 0, 0, 0, 0, -71, -71, -71, -71, -71,
        // State 41
        -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, 0, -70, -70, -70, -70, -70, -70, -70, -70, -70, 0, -70, -70, -70, -70, -70, -70, 0, -70, 0, 0, -70, -70, 0, 0, 0, 0, -70, -70, -70, -70, -70,
        // State 42
        0, -87, -87, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0, -87, -87, -87, 0, 0, 0, 0, -87, -87, -87, -87,
        // State 43
        -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, 0, -68, -68, -68, -68, -68, -68, -68, -68, -68, 0, -68, -68, -68, -68, -68, -68, 0, -68, 0, 0, -68, -68, 0, 0, 0, 0, -68, -68, -68, -68, -68,
        // State 44
        -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, 0, -69, -69, -69, -69, -69, -69, -69, -69, -69, 0, -69, -69, -69, -69, -69, -69, 0, -69, 0, 0, -69, -69, 0, 0, 0, 0, -69, -69, -69, -69, -69,
        // State 45
        -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, 0, -65, -65, -65, -65, -65, -65, -65, -65, -65, 0, -65, -65, -65, -65, -65, -65, 0, -65, 0, 0, -65, -65, 0, 0, 0, 0, -65, -65, -65, -65, -65,
        // State 46
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, 0, -29, -29, -29, -29, -29, -29, -29, 0, -29, 0, -29, -29, -29, -29, -29, -29, 0, -29, 0, 0, -29, -29, 0, 0, 0, 0, -29, -29, -29, -29, -29,
        // State 47
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, 0, -32, 0, -32, -32, -32, -32, -32, -32, 0, -32, 0, 0, -32, -32, 0, 0, 0, 0, -32, -32, -32, -32, -32,
        // State 48
        0, -74, -74, 0, 0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, -74, -74, -74, 0, 0, 0, 0, -74, -74, -74, -74,
        // State 49
        0, -73, -73, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, -73, -73, -73, 0, 0, 0, 0, -73, -73, -73, -73,
        // State 50
        0, -75, -75, 0, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, -75, -75, -75, 0, 0, 0, 0, -75, -75, -75, -75,
        // State 51
        0, -77, -77, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, -77, -77, -77, 0, 0, 0, 0, -77, -77, -77, -77,
        // State 52
        0, -78, -78, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, -78, 0, 0, -78, -78, -78, 0, 0, 0, 0, -78, -78, -78, -78,
        // State 53
        0, -76, -76, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, -76, -76, -76, 0, 0, 0, 0, -76, -76, -76, -76,
        // State 54
        0, -79, -79, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, -79, 0, 0, -79, -79, -79, 0, 0, 0, 0, -79, -79, -79, -79,
        // State 55
        0, -80, -80, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, -80, -80, -80, 0, 0, 0, 0, -80, -80, -80, -80,
        // State 56
        0, -81, -81, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, -81, -81, -81, 0, 0, 0, 0, -81, -81, -81, -81,
        // State 57
        0, -83, -83, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, -83, -83, -83, 0, 0, 0, 0, -83, -83, -83, -83,
        // State 58
        0, -82, -82, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, -82, -82, -82, 0, 0, 0, 0, -82, -82, -82, -82,
        // State 59
        0, -86, -86, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, -86, -86, -86, 0, 0, 0, 0, -86, -86, -86, -86,
        // State 60
        0, -84, -84, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, -84, -84, -84, 0, 0, 0, 0, -84, -84, -84, -84,
        // State 61
        0, -85, -85, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, -85, -85, -85, 0, 0, 0, 0, -85, -85, -85, -85,
        // State 62
        -131, 0, 0, -131, -131, -131, -131, -131, -131, -131, 0, -131, -131, -131, -131, -131, -131, -131, 0, 0, 0, -131, -131, -131, -131, -131, -131, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -131, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, -91, -91, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0, 0, -91, 0, 0, 0, -91, 0, 0, 0, 0, 0, -91, 0, 0, -91, 0, 0, 0, 0, 0, 0, -91, -91, -91, -91,
        // State 68
        0, -89, -89, -89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -89, 0, 0, 0, -89, 0, 0, 0, -89, 0, 0, 0, 0, 0, -89, 0, 0, -89, 0, 0, 0, 0, 0, 0, -89, -89, -89, -89,
        // State 69
        0, -95, -95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -95, 0, 0, 0, -95, 0, 0, 0, 0, 0, 0, 0, 0, 0, -95, 0, 0, -95, 0, 0, 0, 0, 0, 0, -95, -95, -95, -95,
        // State 70
        0, -90, -90, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, 0, -90, 0, 0, 0, -90, 0, 0, 0, 0, 0, -90, 0, 0, -90, 0, 0, 0, 0, 0, 0, -90, -90, -90, -90,
        // State 71
        0, -94, -94, -94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -94, 0, 0, 0, -94, 0, 0, 0, -94, 0, 0, 0, 0, 0, -94, 0, 0, -94, 0, 0, 0, 0, 0, 0, -94, -94, -94, -94,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, 0, -33, 0, -33, -33, -33, -33, -33, -33, 0, -33, 0, 0, -33, -33, 0, 0, 0, 0, -33, -33, -33, -33, -33,
        // State 74
        0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, 0, 0, 0, -34, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0, 0, 0, 0,
        // State 75
        0, 0, 0, -36, 0, 0, 0, 0, 0, -36, 0, 0, 0, -36, -36, -36, -36, -36, 0, 0, 0, -36, -36, -36, -36, -36, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0,
        // State 76
        0, 0, 0, -38, 0, 0, 0, 0, 0, -38, 0, -38, -38, -38, -38, -38, -38, -38, 0, 0, 0, -38, -38, -38, -38, -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0,
        // State 77
        0, 0, 0, -40, 0, -40, -40, -40, 0, -40, 0, -40, -40, -40, -40, -40, -40, -40, 0, 0, 0, -40, -40, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0,
        // State 78
        -42, 0, 0, -42, -42, -42, -42, -42, -42, -42, 0, -42, -42, -42, -42, -42, -42, -42, 0, 0, 0, -42, -42, -42, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, -27, -27, -27, -27, -27, -27, 0, -27, 0, -27, -27, -27, -27, -27, -27, 0, -27, 0, 0, -27, -27, 0, 0, 0, 0, -27, -27, -27, -27, -27,
        // State 82
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, 0, -28, -28, -28, -28, -28, -28, -28, 0, -28, 0, -28, -28, -28, -28, -28, -28, 0, -28, 0, 0, -28, -28, 0, 0, 0, 0, -28, -28, -28, -28, -28,
        // State 83
        0, -7, -7, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, -7, -7, -7, 0, 0, 0, 0, -7, -7, -7, -7,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, -4, 0, 0, 0, 0, 0, 0, -4, 0, 0, -4, 0, 0, 0, 0, 0, 0, -4, -4, -4, -4,
        // State 86
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, -24, -24, -24, -24, -24, -24, 0, -24, 0, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, -24, -24, 0, 0, 0, 0, -24, -24, -24, -24, -24,
        // State 87
        0, -96, -96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -96, 0, 0, 0, -96, 0, 0, 0, 0, 0, 0, 0, 0, 0, -96, 0, 0, -96, 0, 0, 0, 0, 0, 0, -96, -96, -96, -96,
        // State 88
        0, 0, 0, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, 0, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 0, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 91
        0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, -56, 0, 0, 0, -56, -56, 0, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0,
        // State 92
        0, -8, -8, -44, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, -8, -8, -8, 0, 0, 0, 0, -8, -8, -8, -8,
        // State 93
        0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, -5, -5, -5,
        // State 94
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, -20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 95
        0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 96
        0, -92, -92, -92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -92, 0, 0, 0, -92, 0, 0, 0, -92, 0, 0, 0, 0, 0, -92, 0, 0, -92, 0, 0, 0, 0, 0, 0, -92, -92, -92, -92,
        // State 97
        0, 0, 0, -97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 98
        0, -93, -93, -93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -93, 0, 0, 0, -93, 0, 0, 0, -93, 0, 0, 0, 0, 0, -93, 0, 0, -93, 0, 0, 0, 0, 0, 0, -93, -93, -93, -93,
        // State 99
        0, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, -13, 0, 0, 0, 0, 0, 0, -13, -13, -13, -13,
        // State 100
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0,
        // State 101
        0, -14, -14, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, -14, 0, 0, 0, 0, 0, 0, -14, -14, -14, -14,
        // State 102
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 103
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, 0, -23, 0, -23, -23, -23, -23, -23, -23, 0, -23, 0, 0, -23, -23, 0, 0, 0, 0, -23, -23, -23, -23, -23,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 42 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        0,
        // State 1
        -120,
        // State 2
        -58,
        // State 3
        -59,
        // State 4
        -60,
        // State 5
        -61,
        // State 6
        -62,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        -121,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        -134,
        // State 28
        -57,
        // State 29
        -35,
        // State 30
        -37,
        // State 31
        -39,
        // State 32
        -41,
        // State 33
        -43,
        // State 34
        -26,
        // State 35
        -25,
        // State 36
        -132,
        // State 37
        -63,
        // State 38
        -72,
        // State 39
        0,
        // State 40
        -71,
        // State 41
        -70,
        // State 42
        0,
        // State 43
        -68,
        // State 44
        -69,
        // State 45
        -65,
        // State 46
        -29,
        // State 47
        -32,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        -131,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        -33,
        // State 74
        -34,
        // State 75
        -36,
        // State 76
        -38,
        // State 77
        -40,
        // State 78
        -42,
        // State 79
        0,
        // State 80
        0,
        // State 81
        -27,
        // State 82
        -28,
        // State 83
        0,
        // State 84
        0,
        // State 85
        0,
        // State 86
        -24,
        // State 87
        0,
        // State 88
        0,
        // State 89
        0,
        // State 90
        0,
        // State 91
        -56,
        // State 92
        0,
        // State 93
        0,
        // State 94
        0,
        // State 95
        0,
        // State 96
        0,
        // State 97
        0,
        // State 98
        0,
        // State 99
        0,
        // State 100
        0,
        // State 101
        0,
        // State 102
        0,
        // State 103
        -23,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            2 => 19,
            4 => 18,
            8 => 23,
            13 => match state {
                19 => 84,
                _ => 65,
            },
            15 => match state {
                1 => 47,
                11 => 73,
                _ => 1,
            },
            17 => 11,
            18 => 2,
            19 => 3,
            20 => 4,
            21 => 5,
            22 => 6,
            23 => 63,
            24 => 88,
            26 => 66,
            28 => match state {
                8 => 64,
                10 => 72,
                17 => 79,
                18 => 80,
                22 => 94,
                25 => 100,
                26 => 102,
                _ => 27,
            },
            29 => 28,
            30 => match state {
                12 => 74,
                _ => 29,
            },
            31 => match state {
                13 => 75,
                _ => 30,
            },
            32 => match state {
                14 => 76,
                _ => 31,
            },
            33 => match state {
                15 => 77,
                _ => 32,
            },
            34 => match state {
                16 => 78,
                _ => 33,
            },
            36 => match state {
                21 | 23 => 24,
                9 | 19..=20 | 24 => 67,
                _ => 34,
            },
            38 => match state {
                9 | 19..=21 | 23..=24 => 68,
                _ => 35,
            },
            39 => 12,
            40 => 13,
            41 => 14,
            42 => 15,
            43 => 16,
            44 => 7,
            45 => match state {
                20 => 87,
                21 | 23 => 89,
                24 => 97,
                _ => 69,
            },
            46 => 20,
            47 => match state {
                23 => 95,
                _ => 90,
            },
            55 => 36,
            60 => match state {
                7 => 62,
                _ => 37,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i16) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""%""###,
            r###""(""###,
            r###""()""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""++""###,
            r###""-""###,
            r###""/""###,
            r###""<""###,
            r###""<-""###,
            r###""=""###,
            r###""=!""###,
            r###""=<""###,
            r###""=>""###,
            r###"">""###,
            r###""[""###,
            r###""]""###,
            r###""_""###,
            r###""{""###,
            r###""|""###,
            r###""}""###,
            r###""،""###,
            r###""أو""###,
            r###""إذن""###,
            r###""تم""###,
            r###""ثم""###,
            r###""حق""###,
            r###""خطأ""###,
            r###""رد""###,
            r###""صح""###,
            r###""صواب""###,
            r###""لو""###,
            r###""ليس""###,
            r###""ليكن""###,
            r###""نص""###,
            r###""نمط""###,
            r###""وإلا""###,
            r###"r#"\"(\\\\.|[^\"\\\\])*\""#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"\\pL\\w*"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input, 'err>
    where 
    {
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'err ())>,
    }
    impl<'input, 'err> __state_machine::ParserDefinition for __StateMachine<'input, 'err>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Expr<'input>;
        type StateIndex = i16;
        type Action = i16;
        type ReduceIndex = i16;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 42 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i16) -> i16 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i16, nt: usize) -> i16 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i16) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            true
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            __Symbol::Variant1(recovery)
        }

        fn reduce(
            &mut self,
            action: i16,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i16>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.errors,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'err,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(3, _) if true => Some(0),
            Token(4, _) if true => Some(1),
            Token(5, _) if true => Some(2),
            Token(6, _) if true => Some(3),
            Token(7, _) if true => Some(4),
            Token(8, _) if true => Some(5),
            Token(9, _) if true => Some(6),
            Token(10, _) if true => Some(7),
            Token(11, _) if true => Some(8),
            Token(12, _) if true => Some(9),
            Token(13, _) if true => Some(10),
            Token(14, _) if true => Some(11),
            Token(15, _) if true => Some(12),
            Token(16, _) if true => Some(13),
            Token(17, _) if true => Some(14),
            Token(18, _) if true => Some(15),
            Token(19, _) if true => Some(16),
            Token(20, _) if true => Some(17),
            Token(21, _) if true => Some(18),
            Token(22, _) if true => Some(19),
            Token(23, _) if true => Some(20),
            Token(24, _) if true => Some(21),
            Token(25, _) if true => Some(22),
            Token(26, _) if true => Some(23),
            Token(27, _) if true => Some(24),
            Token(28, _) if true => Some(25),
            Token(29, _) if true => Some(26),
            Token(30, _) if true => Some(27),
            Token(31, _) if true => Some(28),
            Token(32, _) if true => Some(29),
            Token(33, _) if true => Some(30),
            Token(34, _) if true => Some(31),
            Token(35, _) if true => Some(32),
            Token(36, _) if true => Some(33),
            Token(37, _) if true => Some(34),
            Token(38, _) if true => Some(35),
            Token(39, _) if true => Some(36),
            Token(40, _) if true => Some(37),
            Token(0, _) if true => Some(38),
            Token(1, _) if true => Some(39),
            Token(2, _) if true => Some(40),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'err,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 => match __token {
                Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) | Token(25, __tok0) | Token(26, __tok0) | Token(27, __tok0) | Token(28, __tok0) | Token(29, __tok0) | Token(30, __tok0) | Token(31, __tok0) | Token(32, __tok0) | Token(33, __tok0) | Token(34, __tok0) | Token(35, __tok0) | Token(36, __tok0) | Token(37, __tok0) | Token(38, __tok0) | Token(39, __tok0) | Token(40, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'err,
    >(
        __reduce_index: i16,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'err>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 10,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 11,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 12,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 14,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 15,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 16,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 17,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 18,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 19,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 21,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 22,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 23,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 23,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 24,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 24,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 25,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 25,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 26,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 27,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 28,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 35,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 37,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 45,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 45,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            94 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 46,
                }
            }
            95 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 46,
                }
            }
            96 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 47,
                }
            }
            97 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            98 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 48,
                }
            }
            99 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            100 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            101 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            102 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            104 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 49,
                }
            }
            105 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            106 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 49,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 49,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 51,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 52,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 53,
                }
            }
            117 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 54,
                }
            }
            118 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 54,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 55,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 56,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 57,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 57,
                }
            }
            125 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            126 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 58,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 59,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 59,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 60,
                }
            }
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            132 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 61,
                }
            }
            133 => __state_machine::SimulatedReduce::Accept,
            134 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ExprParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ExprParser {
        pub fn new() -> ExprParser {
            let __builder = super::__intern_token::new_builder();
            ExprParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'err,
        >(
            &self,
            errors: &'err mut Vec<usize>,
            input: &'input str,
        ) -> Result<Expr<'input>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    errors,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __error_state: i16,
        __states: & [i16],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.push(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i16>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<Result<Expr<'input>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            50 => {
                __reduce50(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            51 => {
                __reduce51(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            52 => {
                __reduce52(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            53 => {
                __reduce53(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            54 => {
                __reduce54(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            55 => {
                __reduce55(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            56 => {
                __reduce56(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            57 => {
                __reduce57(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            58 => {
                __reduce58(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            59 => {
                __reduce59(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            60 => {
                __reduce60(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            61 => {
                __reduce61(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            62 => {
                __reduce62(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            63 => {
                __reduce63(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            64 => {
                __reduce64(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            65 => {
                __reduce65(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            66 => {
                __reduce66(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            67 => {
                __reduce67(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            68 => {
                __reduce68(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            69 => {
                __reduce69(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            70 => {
                __reduce70(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            71 => {
                __reduce71(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            72 => {
                __reduce72(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            73 => {
                __reduce73(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            74 => {
                __reduce74(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            75 => {
                __reduce75(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            76 => {
                __reduce76(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            77 => {
                __reduce77(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            78 => {
                __reduce78(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            79 => {
                __reduce79(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            80 => {
                __reduce80(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            81 => {
                __reduce81(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            82 => {
                __reduce82(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            83 => {
                __reduce83(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            84 => {
                __reduce84(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            85 => {
                __reduce85(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            86 => {
                __reduce86(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            87 => {
                __reduce87(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            88 => {
                __reduce88(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            89 => {
                __reduce89(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            90 => {
                __reduce90(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            91 => {
                __reduce91(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            92 => {
                __reduce92(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            93 => {
                __reduce93(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            94 => {
                __reduce94(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            95 => {
                __reduce95(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            96 => {
                __reduce96(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            97 => {
                __reduce97(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            98 => {
                __reduce98(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            99 => {
                __reduce99(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            100 => {
                __reduce100(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            101 => {
                __reduce101(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            102 => {
                __reduce102(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            103 => {
                __reduce103(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            104 => {
                __reduce104(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            105 => {
                __reduce105(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            106 => {
                __reduce106(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            107 => {
                __reduce107(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            108 => {
                __reduce108(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            109 => {
                __reduce109(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            110 => {
                __reduce110(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            111 => {
                __reduce111(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            112 => {
                __reduce112(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            113 => {
                __reduce113(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            114 => {
                __reduce114(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            115 => {
                __reduce115(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            116 => {
                __reduce116(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            117 => {
                __reduce117(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            118 => {
                __reduce118(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            119 => {
                __reduce119(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            120 => {
                __reduce120(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            121 => {
                __reduce121(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            122 => {
                __reduce122(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            123 => {
                __reduce123(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            124 => {
                __reduce124(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            125 => {
                __reduce125(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            126 => {
                __reduce126(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            127 => {
                __reduce127(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            128 => {
                __reduce128(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            129 => {
                __reduce129(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            130 => {
                __reduce130(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            131 => {
                __reduce131(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            132 => {
                __reduce132(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            133 => {
                // __Expr = Expr => ActionFn(1);
                let __sym0 = __pop_Variant5(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            134 => {
                __reduce134(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (alloc::vec::Vec<Pattern<'input>>, Expr<'input>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (&'input str, ProtoType<'input>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant25(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpcode, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Binding<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Pattern<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ProtoType<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UnOpcode, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Binding<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expr<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Pattern<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ProtoType<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<(&'input str, ProtoType<'input>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant26(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Binding<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant24(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Expr<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Pattern<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<ProtoType<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<&'input str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Vec<&'input str>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant27(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<&'input str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // "،"? = "،" => ActionFn(122);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action122::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // "،"? =  => ActionFn(123);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action123::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Arm> "،") = Arm, "،" => ActionFn(113);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action113::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Arm> "،")+ = Arm, "،" => ActionFn(133);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action133::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Arm> "،")+ = (<Arm> "،")+, Arm, "،" => ActionFn(134);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action134::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> "،") = Expr, "،" => ActionFn(126);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action126::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> "،")+ = Expr, "،" => ActionFn(135);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action135::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce7<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> "،")+ = (<Expr> "،")+, Expr, "،" => ActionFn(136);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action136::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<ID> "،") = ID, "،" => ActionFn(108);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action108::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<ID> "،")+ = ID, "،" => ActionFn(137);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action137::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce10<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<ID> "،")+ = (<ID> "،")+, ID, "،" => ActionFn(138);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action138::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce11<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<PatternH> "،") = PatternH, "،" => ActionFn(116);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action116::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce12<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<PatternH> "،")+ = PatternH, "،" => ActionFn(139);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action139::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce13<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<PatternH> "،")+ = (<PatternH> "،")+, PatternH, "،" => ActionFn(140);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action140::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce14<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Type> "،") = Type, "،" => ActionFn(119);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action119::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce15<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Type> "،")+ = Type, "،" => ActionFn(141);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action141::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce16<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Type> "،")+ = (<Type> "،")+, Type, "،" => ActionFn(142);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action142::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce17<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(88);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action88::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce18<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(87);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action87::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce19<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Arm = Pattern+, "=>", Expr => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 13)
    }
    pub(crate) fn __reduce20<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Arm? = Arm => ActionFn(109);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action109::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce21<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Arm? =  => ActionFn(110);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action110::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce22<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = "لو", Expr, "إذن", Expr, "وإلا", Expr, "تم" => ActionFn(42);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant5(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action42::<>(errors, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (7, 15)
    }
    pub(crate) fn __reduce23<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = "{", Comma<Arm>, "}" => ActionFn(43);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action43::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce24<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = Literal => ActionFn(44);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce25<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = ID => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce26<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = "(", Comma2<Expr>, ")" => ActionFn(46);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant14(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action46::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce27<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = "(", Expr, ")" => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action47::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce28<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = error => ActionFn(146);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action146::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce29<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base* =  => ActionFn(72);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action72::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 16)
    }
    pub(crate) fn __reduce30<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base* = Base+ => ActionFn(73);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce31<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base+ = Base => ActionFn(120);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action120::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce32<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base+ = Base+, Base => ActionFn(121);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action121::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce33<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op0, Expr1> = Class<Op0, Expr1>, Op0, Expr1 => ActionFn(84);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action84::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 18)
    }
    pub(crate) fn __reduce34<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op0, Expr1> = Expr1 => ActionFn(85);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action85::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce35<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op1, Expr2> = Class<Op1, Expr2>, Op1, Expr2 => ActionFn(82);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce36<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op1, Expr2> = Expr2 => ActionFn(83);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action83::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce37<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op2, Expr3> = Class<Op2, Expr3>, Op2, Expr3 => ActionFn(80);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action80::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce38<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op2, Expr3> = Expr3 => ActionFn(81);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action81::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce39<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op3, Expr4> = Class<Op3, Expr4>, Op3, Expr4 => ActionFn(78);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action78::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 21)
    }
    pub(crate) fn __reduce40<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op3, Expr4> = Expr4 => ActionFn(79);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce41<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op4, Expr5> = Class<Op4, Expr5>, Op4, Expr5 => ActionFn(76);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action76::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 22)
    }
    pub(crate) fn __reduce42<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op4, Expr5> = Expr5 => ActionFn(77);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce43<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma2<Expr> = (<Expr> "،")+, Expr, "،" => ActionFn(127);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action127::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce44<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma2<Expr> = (<Expr> "،")+, Expr => ActionFn(128);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action128::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 23)
    }
    pub(crate) fn __reduce45<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma2<PatternH> = (<PatternH> "،")+, PatternH, "،" => ActionFn(129);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action129::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 24)
    }
    pub(crate) fn __reduce46<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma2<PatternH> = (<PatternH> "،")+, PatternH => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action130::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 24)
    }
    pub(crate) fn __reduce47<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma2<Type> = (<Type> "،")+, Type, "،" => ActionFn(131);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action131::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (3, 25)
    }
    pub(crate) fn __reduce48<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma2<Type> = (<Type> "،")+, Type => ActionFn(132);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action132::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 25)
    }
    pub(crate) fn __reduce49<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<Arm> = Arm => ActionFn(92);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action92::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce50<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<Arm> = (<Arm> "،")+, Arm => ActionFn(149);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action149::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 26)
    }
    pub(crate) fn __reduce51<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<Arm> = (<Arm> "،")+ => ActionFn(150);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action150::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce52<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<ID> = ID => ActionFn(94);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action94::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce53<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<ID> = (<ID> "،")+, ID => ActionFn(153);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action153::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 27)
    }
    pub(crate) fn __reduce54<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<ID> = (<ID> "،")+ => ActionFn(154);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action154::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce55<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr, "[", Expr, "]" => ActionFn(33);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action33::<>(errors, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (4, 28)
    }
    pub(crate) fn __reduce56<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr0 => ActionFn(34);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce57<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = Class<Op0, Expr1> => ActionFn(35);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce58<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr1 = Class<Op1, Expr2> => ActionFn(36);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce59<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr2 = Class<Op2, Expr3> => ActionFn(37);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce60<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr3 = Class<Op3, Expr4> => ActionFn(38);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce61<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr4 = Class<Op4, Expr5> => ActionFn(39);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce62<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr5 = UnaryClass<Op5, Term> => ActionFn(40);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce63<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FnBinding = "رد", ID, "=", "{", Comma<Arm>, "}" => ActionFn(12);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action12::<>(errors, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (6, 35)
    }
    pub(crate) fn __reduce64<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ID = r#"\\pL\\w*"# => ActionFn(70);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce65<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ID? = ID => ActionFn(104);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action104::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce66<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ID? =  => ActionFn(105);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action105::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 37)
    }
    pub(crate) fn __reduce67<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Literal = r#"\"(\\\\.|[^\"\\\\])*\""# => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce68<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Literal = r#"[0-9]+"# => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce69<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Literal = "صواب" => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce70<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Literal = "خطأ" => ActionFn(52);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce71<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Literal = "()" => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce72<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op0 = "ثم" => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce73<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op0 = "أو" => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce74<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op1 = "<" => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce75<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op1 = ">" => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce76<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op1 = "=<" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce77<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op1 = "=>" => ActionFn(59);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action59::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce78<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op2 = "=" => ActionFn(60);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce79<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op2 = "=!" => ActionFn(61);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce80<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op3 = "+" => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action62::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce81<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op3 = "-" => ActionFn(63);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action63::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce82<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op3 = "++" => ActionFn(64);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce83<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "*" => ActionFn(65);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce84<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "/" => ActionFn(66);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce85<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "%" => ActionFn(67);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce86<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op5 = "ليس" => ActionFn(68);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce87<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op5 = "-" => ActionFn(69);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce88<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern = Literal => ActionFn(14);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce89<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern = "_" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce90<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern = ID => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce91<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern = "(", Comma2<PatternH>, ")" => ActionFn(17);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 45)
    }
    pub(crate) fn __reduce92<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern = "(", PatternH, ")" => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 45)
    }
    pub(crate) fn __reduce93<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern = error => ActionFn(147);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action147::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce94<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern+ = Pattern => ActionFn(90);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action90::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce95<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern+ = Pattern+, Pattern => ActionFn(91);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action91::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 46)
    }
    pub(crate) fn __reduce96<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // PatternH = ID, Pattern => ActionFn(20);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 47)
    }
    pub(crate) fn __reduce97<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // PatternH = Pattern => ActionFn(21);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce98<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(155);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action155::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (0, 48)
    }
    pub(crate) fn __reduce99<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Program = Statement+ => ActionFn(156);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action156::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce100<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = "()" => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce101<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = "صح" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce102<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = "حق" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce103<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = "نص" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce104<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = "(", Type, ")" => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 49)
    }
    pub(crate) fn __reduce105<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = ID => ActionFn(29);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce106<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = ID, SimpleType => ActionFn(30);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action30::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 49)
    }
    pub(crate) fn __reduce107<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = "(", Comma2<Type>, ")" => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant16(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 49)
    }
    pub(crate) fn __reduce108<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = error => ActionFn(148);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action148::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce109<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement = TypeDecl => ActionFn(3);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce110<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement = ValBinding => ActionFn(4);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce111<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement = FnBinding => ActionFn(5);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce112<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(100);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action100::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (0, 51)
    }
    pub(crate) fn __reduce113<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(101);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action101::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce114<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(102);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce115<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(103);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action103::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (2, 52)
    }
    pub(crate) fn __reduce116<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SumVarDecl = "|", ID, Type => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (3, 53)
    }
    pub(crate) fn __reduce117<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SumVarDecl+ = SumVarDecl => ActionFn(96);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce118<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SumVarDecl+ = SumVarDecl+, SumVarDecl => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant26(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce119<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = Base => ActionFn(151);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action151::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce120<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = Base, Base+ => ActionFn(152);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action152::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 55)
    }
    pub(crate) fn __reduce121<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = SimpleType => ActionFn(22);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce122<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = SimpleType, "<-", Type => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 56)
    }
    pub(crate) fn __reduce123<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeDecl = "نمط", ID, TypeVars, "=", SumVarDecl+ => ActionFn(157);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant26(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action157::<>(errors, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (5, 57)
    }
    pub(crate) fn __reduce124<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeDecl = "نمط", ID, "=", SumVarDecl+ => ActionFn(158);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant26(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action158::<>(errors, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (4, 57)
    }
    pub(crate) fn __reduce125<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeVars = "()" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce126<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeVars = ID => ActionFn(8);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce127<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeVars = "(", Comma<ID>, ")" => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (3, 58)
    }
    pub(crate) fn __reduce128<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeVars? = TypeVars => ActionFn(98);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce129<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeVars? =  => ActionFn(99);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action99::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (0, 59)
    }
    pub(crate) fn __reduce130<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // UnaryClass<Op5, Term> = Op5, UnaryClass<Op5, Term> => ActionFn(74);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action74::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 60)
    }
    pub(crate) fn __reduce131<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // UnaryClass<Op5, Term> = Term => ActionFn(75);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce132<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ValBinding = "ليكن", Pattern, "=", Expr => ActionFn(11);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action11::<>(errors, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (4, 61)
    }
    pub(crate) fn __reduce134<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 63)
    }
}
pub use self::__parse__Expr::ExprParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::{
    ast::*,
    types::{ProtoType, Literal, BinOpcode, UnOpcode}
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(__lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>),
        Variant2(core::option::Option<&'input str>),
        Variant3((alloc::vec::Vec<Pattern<'input>>, Expr<'input>)),
        Variant4(alloc::vec::Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>),
        Variant5(Expr<'input>),
        Variant6(alloc::vec::Vec<Expr<'input>>),
        Variant7(alloc::vec::Vec<&'input str>),
        Variant8(Pattern<'input>),
        Variant9(alloc::vec::Vec<Pattern<'input>>),
        Variant10(ProtoType<'input>),
        Variant11(alloc::vec::Vec<ProtoType<'input>>),
        Variant12(usize),
        Variant13(core::option::Option<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>),
        Variant14(Vec<Expr<'input>>),
        Variant15(Vec<Pattern<'input>>),
        Variant16(Vec<ProtoType<'input>>),
        Variant17(Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>),
        Variant18(Vec<&'input str>),
        Variant19(Binding<'input>),
        Variant20(Literal<'input>),
        Variant21(BinOpcode),
        Variant22(UnOpcode),
        Variant23(Vec<Binding<'input>>),
        Variant24(alloc::vec::Vec<Binding<'input>>),
        Variant25((&'input str, ProtoType<'input>)),
        Variant26(alloc::vec::Vec<(&'input str, ProtoType<'input>)>),
        Variant27(core::option::Option<Vec<&'input str>>),
    }
    const __ACTION: &[i16] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 5, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 5, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0,
        // State 3
        0, 6, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 0, 0, 0, 0, 0, 0, 62, 63, 54, 64,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0,
        // State 5
        0, 6, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 0, 0, 0, 0, 0, 0, 62, 63, 54, 64,
        // State 6
        0, 11, 71, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0,
        // State 7
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 8
        0, 6, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 0, 0, 0, 0, 0, 0, 62, 63, 54, 64,
        // State 9
        0, 6, 58, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, -91, 0, 0, 0, 0, 0, 60, 0, 0, 61, 0, 0, 0, 0, 0, 0, 62, 63, 54, 64,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 6, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 0, 0, 0, 0, 0, 0, 62, 63, 54, 64,
        // State 13
        -120, 21, 58, -120, -120, -120, -120, -120, -120, -120, 0, -120, -120, -120, -120, -120, -120, -120, 0, 22, 0, -120, -120, -120, -120, -120, -120, 0, 60, -120, 0, 61, 23, 0, -120, 0, -120, -120, 62, 63, 54, 85,
        // State 14
        0, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58, -58, 0, 0, 0, -58, -58, 98, -58, -58, 99, 0, 0, -58, 0, 0, 0, 0, -58, 0, -58, -58, 0, 0, 0, 0,
        // State 15
        0, 0, 0, -59, 0, 0, 0, 0, 0, 100, 0, 0, 0, 101, 102, 103, -59, -59, 0, 0, 0, -59, -59, -59, -59, -59, -59, 0, 0, -59, 0, 0, 0, 0, -59, 0, -59, -59, 0, 0, 0, 0,
        // State 16
        0, 0, 0, -60, 0, 0, 0, 0, 0, -60, 0, 104, 105, -60, -60, -60, -60, -60, 0, 0, 0, -60, -60, -60, -60, -60, -60, 0, 0, -60, 0, 0, 0, 0, -60, 0, -60, -60, 0, 0, 0, 0,
        // State 17
        0, 0, 0, -61, 0, 106, 107, 108, 0, -61, 0, -61, -61, -61, -61, -61, -61, -61, 0, 0, 0, -61, -61, -61, -61, -61, -61, 0, 0, -61, 0, 0, 0, 0, -61, 0, -61, -61, 0, 0, 0, 0,
        // State 18
        109, 0, 0, -62, 110, -62, -62, -62, 111, -62, 0, -62, -62, -62, -62, -62, -62, -62, 0, 0, 0, -62, -62, -62, -62, -62, -62, 0, 0, -62, 0, 0, 0, 0, -62, 0, -62, -62, 0, 0, 0, 0,
        // State 19
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 20
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 21
        0, 6, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 0, 0, 0, 0, 0, 0, 62, 63, 54, 64,
        // State 22
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, -125, 0, 0, 0, 0, -125, 0, -125, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0,
        // State 27
        0, 6, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, -52, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 0, 0, 0, 0, 0, 0, 62, 63, 54, 64,
        // State 28
        0, 6, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 0, 0, 0, 0, 0, 0, 62, 63, 54, 64,
        // State 29
        -121, 21, 58, -121, -121, -121, -121, -121, -121, -121, 0, -121, -121, -121, -121, -121, -121, -121, 0, 22, 0, -121, -121, -121, -121, -121, -121, 0, 60, -121, 0, 61, 23, 0, -121, 0, -121, -121, 62, 63, 54, 85,
        // State 30
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 31
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 32
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 33
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 34
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 35
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 36
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, -124, 0, 0, 0, 0, -124, 0, -124, 0, 0, 0, 0, 0,
        // State 38
        0, 43, 141, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 142, 0, 0, 143, 0, 0, 0, 0, 144, 0, 0, 0, 0, 54, 145,
        // State 39
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 40
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 41
        0, 43, 141, -106, 0, 0, 0, 0, 0, 0, -106, 0, 0, 0, 0, 0, 0, 0, 0, 0, -106, 0, -106, 0, 0, 0, 0, 142, 0, -106, 143, 0, 0, 0, -106, 144, -106, 0, 0, 0, 54, 145,
        // State 42
        0, 43, 141, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 142, 0, 0, 143, 0, 0, 0, 0, 144, 0, 0, 0, 0, 54, 145,
        // State 43
        0, 43, 141, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 142, 0, 0, 143, 0, 0, 0, 0, 144, 0, 0, 0, 0, 54, 145,
        // State 44
        0, 43, 141, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 142, 0, 0, 143, 0, 0, 0, 0, 144, 0, 0, 0, 0, 54, 145,
        // State 45
        0, 21, 58, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 61, 23, 84, 0, 0, 0, 0, 62, 63, 54, 85,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -112, 0, 0, 0, 0, -112, 0, -112, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -115, 0, 0, 0, 0, -115, 0, -115, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -110, 0, 0, 0, 0, -110, 0, -110, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -111, 0, 0, 0, 0, -111, 0, -111, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -116, 0, 0, 0, 0, -116, 0, -116, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, 0, -65, -65, -65, -65, -65, -65, -65, -65,
        // State 54
        0, -91, -91, -91, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0, -91, 0, 0, 0, -91, 0, 0, 0, -91, 0, 0, 0, 0, 0, -91, 0, 0, -91, 0, 0, 0, 0, 0, 0, -91, -91, -91, -91,
        // State 55
        0, -89, -89, -89, 0, 0, 0, 0, 0, 0, 0, -89, 0, 0, -89, 0, 0, 0, -89, 0, 0, 0, -89, 0, 0, 0, 0, 0, -89, 0, 0, -89, 0, 0, 0, 0, 0, 0, -89, -89, -89, -89,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        -72, -72, -72, -72, -72, -72, -72, -72, -72, -72, 0, -72, -72, -72, -72, -72, -72, -72, -72, -72, 0, -72, -72, -72, -72, -72, -72, 0, -72, -72, 0, -72, -72, 0, -72, 0, -72, -72, -72, -72, -72, -72,
        // State 58
        0, -90, -90, -90, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, -90, 0, 0, 0, -90, 0, 0, 0, -90, 0, 0, 0, 0, 0, -90, 0, 0, -90, 0, 0, 0, 0, 0, 0, -90, -90, -90, -90,
        // State 59
        -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, 0, -71, -71, -71, -71, -71, -71, -71, -71, -71, 0, -71, -71, -71, -71, -71, -71, 0, -71, -71, 0, -71, -71, 0, -71, 0, -71, -71, -71, -71, -71, -71,
        // State 60
        -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, 0, -70, -70, -70, -70, -70, -70, -70, -70, -70, 0, -70, -70, -70, -70, -70, -70, 0, -70, -70, 0, -70, -70, 0, -70, 0, -70, -70, -70, -70, -70, -70,
        // State 61
        -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, 0, -68, -68, -68, -68, -68, -68, -68, -68, -68, 0, -68, -68, -68, -68, -68, -68, 0, -68, -68, 0, -68, -68, 0, -68, 0, -68, -68, -68, -68, -68, -68,
        // State 62
        -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, 0, -69, -69, -69, -69, -69, -69, -69, -69, -69, 0, -69, -69, -69, -69, -69, -69, 0, -69, -69, 0, -69, -69, 0, -69, 0, -69, -69, -69, -69, -69, -69,
        // State 63
        0, -94, -94, -94, 0, 0, 0, 0, 0, 0, 0, -94, 0, 0, -94, 0, 0, 0, -94, 0, 0, 0, -94, 0, 0, 0, 0, 0, -94, 0, 0, -94, 0, 0, 0, 0, 0, 0, -94, -94, -94, -94,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -133, 0, 0, 0, 0, -133, 0, -133, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, -57, 0, 0, 0, -57, -57, 0, -57, -57, 0, 0, 0, -57, 0, 0, 0, 0, -57, 0, -57, -57, 0, 0, 0, 0,
        // State 73
        0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, 0, 0, 0, -35, -35, -35, -35, -35, -35, 0, 0, -35, 0, 0, 0, 0, -35, 0, -35, -35, 0, 0, 0, 0,
        // State 74
        0, 0, 0, -37, 0, 0, 0, 0, 0, -37, 0, 0, 0, -37, -37, -37, -37, -37, 0, 0, 0, -37, -37, -37, -37, -37, -37, 0, 0, -37, 0, 0, 0, 0, -37, 0, -37, -37, 0, 0, 0, 0,
        // State 75
        0, 0, 0, -39, 0, 0, 0, 0, 0, -39, 0, -39, -39, -39, -39, -39, -39, -39, 0, 0, 0, -39, -39, -39, -39, -39, -39, 0, 0, -39, 0, 0, 0, 0, -39, 0, -39, -39, 0, 0, 0, 0,
        // State 76
        0, 0, 0, -41, 0, -41, -41, -41, 0, -41, 0, -41, -41, -41, -41, -41, -41, -41, 0, 0, 0, -41, -41, -41, -41, -41, -41, 0, 0, -41, 0, 0, 0, 0, -41, 0, -41, -41, 0, 0, 0, 0,
        // State 77
        -43, 0, 0, -43, -43, -43, -43, -43, -43, -43, 0, -43, -43, -43, -43, -43, -43, -43, 0, 0, 0, -43, -43, -43, -43, -43, -43, 0, 0, -43, 0, 0, 0, 0, -43, 0, -43, -43, 0, 0, 0, 0,
        // State 78
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, 0, -26, 0, -26, -26, -26, -26, -26, -26, 0, -26, -26, 0, -26, -26, 0, -26, 0, -26, -26, -26, -26, -26, -26,
        // State 79
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, -25, -25, -25, -25, -25, -25, 0, -25, 0, -25, -25, -25, -25, -25, -25, 0, -25, -25, 0, -25, -25, 0, -25, 0, -25, -25, -25, -25, -25, -25,
        // State 80
        -132, 0, 0, -132, -132, -132, -132, -132, -132, -132, 0, -132, -132, -132, -132, -132, -132, -132, 0, 0, 0, -132, -132, -132, -132, -132, -132, 0, 0, -132, 0, 0, 0, 0, -132, 0, -132, -132, 0, 0, 0, 0,
        // State 81
        -63, 0, 0, -63, -63, -63, -63, -63, -63, -63, 0, -63, -63, -63, -63, -63, -63, -63, 0, 0, 0, -63, -63, -63, -63, -63, -63, 0, 0, -63, 0, 0, 0, 0, -63, 0, -63, -63, 0, 0, 0, 0,
        // State 82
        0, -88, -88, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, -88, 0, 0, -88, -88, -88, 0, 0, 0, 0, -88, -88, -88, -88,
        // State 83
        0, -87, -87, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0, -87, -87, -87, 0, 0, 0, 0, -87, -87, -87, -87,
        // State 84
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, 0, -29, -29, -29, -29, -29, -29, -29, 0, -29, 0, -29, -29, -29, -29, -29, -29, 0, -29, -29, 0, -29, -29, 0, -29, 0, -29, -29, -29, -29, -29, -29,
        // State 85
        0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 117, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, -92, -92, -92, 0, 0, 0, 0, 0, 0, 0, -92, 0, 0, -92, 0, 0, 0, -92, 0, 0, 0, -92, 0, 0, 0, 0, 0, -92, 0, 0, -92, 0, 0, 0, 0, 0, 0, -92, -92, -92, -92,
        // State 87
        0, 0, 0, -97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        0, -93, -93, -93, 0, 0, 0, 0, 0, 0, 0, -93, 0, 0, -93, 0, 0, 0, -93, 0, 0, 0, -93, 0, 0, 0, 0, 0, -93, 0, 0, -93, 0, 0, 0, 0, 0, 0, -93, -93, -93, -93,
        // State 89
        0, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, -13, 0, 0, 0, 0, 0, 0, -13, -13, -13, -13,
        // State 90
        0, 0, 0, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 91
        0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 92
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -118, 0, 0, 0, 0, 0, 0, 0, 0, -118, 0, 0, 0, 0, -118, 0, -118, 0, 0, 0, 0, 0,
        // State 93
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 123, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 94
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 95
        0, -95, -95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -95, 0, 0, 0, -95, 0, 0, 0, 0, 0, 0, 0, 0, 0, -95, 0, 0, -95, 0, 0, 0, 0, 0, 0, -95, -95, -95, -95,
        // State 96
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, 0, -32, 0, -32, -32, -32, -32, -32, -32, 0, -32, -32, 0, -32, -32, 0, -32, 0, -32, -32, -32, -32, -32, -32,
        // State 97
        0, -74, -74, 0, 0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, -74, -74, -74, 0, 0, 0, 0, -74, -74, -74, -74,
        // State 98
        0, -73, -73, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, -73, -73, -73, 0, 0, 0, 0, -73, -73, -73, -73,
        // State 99
        0, -75, -75, 0, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, -75, -75, -75, 0, 0, 0, 0, -75, -75, -75, -75,
        // State 100
        0, -77, -77, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, -77, -77, -77, 0, 0, 0, 0, -77, -77, -77, -77,
        // State 101
        0, -78, -78, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, -78, 0, 0, -78, -78, -78, 0, 0, 0, 0, -78, -78, -78, -78,
        // State 102
        0, -76, -76, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, -76, -76, -76, 0, 0, 0, 0, -76, -76, -76, -76,
        // State 103
        0, -79, -79, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, -79, 0, 0, -79, -79, -79, 0, 0, 0, 0, -79, -79, -79, -79,
        // State 104
        0, -80, -80, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, -80, -80, -80, 0, 0, 0, 0, -80, -80, -80, -80,
        // State 105
        0, -81, -81, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, -81, -81, -81, 0, 0, 0, 0, -81, -81, -81, -81,
        // State 106
        0, -83, -83, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, -83, -83, -83, 0, 0, 0, 0, -83, -83, -83, -83,
        // State 107
        0, -82, -82, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, -82, -82, -82, 0, 0, 0, 0, -82, -82, -82, -82,
        // State 108
        0, -86, -86, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, -86, -86, -86, 0, 0, 0, 0, -86, -86, -86, -86,
        // State 109
        0, -84, -84, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, -84, -84, -84, 0, 0, 0, 0, -84, -84, -84, -84,
        // State 110
        0, -85, -85, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, -85, -85, -85, 0, 0, 0, 0, -85, -85, -85, -85,
        // State 111
        -131, 0, 0, -131, -131, -131, -131, -131, -131, -131, 0, -131, -131, -131, -131, -131, -131, -131, 0, 0, 0, -131, -131, -131, -131, -131, -131, 0, 0, -131, 0, 0, 0, 0, -131, 0, -131, -131, 0, 0, 0, 0,
        // State 112
        0, 0, 0, 134, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 113
        0, 0, 0, 135, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 114
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 115
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 116
        0, -14, -14, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, -14, 0, 0, 0, 0, 0, 0, -14, -14, -14, -14,
        // State 117
        0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 118
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 119
        0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0,
        // State 120
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -119, 0, 0, 0, 0, 0, 0, 0, 0, -119, 0, 0, 0, 0, -119, 0, -119, 0, 0, 0, 0, 0,
        // State 121
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 146, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 122
        0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, -4, 0, 0, 0, 0, 0, 0, -4, 0, 0, -4, 0, 0, 0, 0, 0, 0, -4, -4, -4, -4,
        // State 123
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0, -64, 0, -64, 0, 0, 0, 0, 0,
        // State 124
        0, -96, -96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -96, 0, 0, 0, -96, 0, 0, 0, 0, 0, 0, 0, 0, 0, -96, 0, 0, -96, 0, 0, 0, 0, 0, 0, -96, -96, -96, -96,
        // State 125
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, 0, -33, 0, -33, -33, -33, -33, -33, -33, 0, -33, -33, 0, -33, -33, 0, -33, 0, -33, -33, -33, -33, -33, -33,
        // State 126
        0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, 0, 0, 0, -34, -34, -34, -34, -34, -34, 0, 0, -34, 0, 0, 0, 0, -34, 0, -34, -34, 0, 0, 0, 0,
        // State 127
        0, 0, 0, -36, 0, 0, 0, 0, 0, -36, 0, 0, 0, -36, -36, -36, -36, -36, 0, 0, 0, -36, -36, -36, -36, -36, -36, 0, 0, -36, 0, 0, 0, 0, -36, 0, -36, -36, 0, 0, 0, 0,
        // State 128
        0, 0, 0, -38, 0, 0, 0, 0, 0, -38, 0, -38, -38, -38, -38, -38, -38, -38, 0, 0, 0, -38, -38, -38, -38, -38, -38, 0, 0, -38, 0, 0, 0, 0, -38, 0, -38, -38, 0, 0, 0, 0,
        // State 129
        0, 0, 0, -40, 0, -40, -40, -40, 0, -40, 0, -40, -40, -40, -40, -40, -40, -40, 0, 0, 0, -40, -40, -40, -40, -40, -40, 0, 0, -40, 0, 0, 0, 0, -40, 0, -40, -40, 0, 0, 0, 0,
        // State 130
        -42, 0, 0, -42, -42, -42, -42, -42, -42, -42, 0, -42, -42, -42, -42, -42, -42, -42, 0, 0, 0, -42, -42, -42, -42, -42, -42, 0, 0, -42, 0, 0, 0, 0, -42, 0, -42, -42, 0, 0, 0, 0,
        // State 131
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 148, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 132
        0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 149, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 133
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, -27, -27, -27, -27, -27, -27, 0, -27, 0, -27, -27, -27, -27, -27, -27, 0, -27, -27, 0, -27, -27, 0, -27, 0, -27, -27, -27, -27, -27, -27,
        // State 134
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, 0, -28, -28, -28, -28, -28, -28, -28, 0, -28, 0, -28, -28, -28, -28, -28, -28, 0, -28, -28, 0, -28, -28, 0, -28, 0, -28, -28, -28, -28, -28, -28,
        // State 135
        0, -7, -7, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, -7, -7, -7, 0, 0, 0, 0, -7, -7, -7, -7,
        // State 136
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, -24, -24, -24, -24, -24, -24, 0, -24, 0, -24, -24, -24, -24, -24, -24, 0, -24, -24, 0, -24, -24, 0, -24, 0, -24, -24, -24, -24, -24, -24,
        // State 137
        0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0,
        // State 138
        0, 0, 0, -122, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, -122, 0, -122, 0, 0, 0, 0, 0, 0, -122, 0, 0, 0, 0, -122, 0, -122, 0, 0, 0, 0, 0,
        // State 139
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -117, 0, 0, 0, 0, 0, 0, 0, 0, -117, 0, 0, 0, 0, -117, 0, -117, 0, 0, 0, 0, 0,
        // State 140
        0, 0, 0, -101, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, 0, 0, 0, 0, -101, 0, -101, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, -101, 0, -101, 0, 0, 0, 0, 0,
        // State 141
        0, 0, 0, -103, 0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, 0, 0, 0, 0, 0, -103, 0, -103, 0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, -103, 0, -103, 0, 0, 0, 0, 0,
        // State 142
        0, 0, 0, -102, 0, 0, 0, 0, 0, 0, -102, 0, 0, 0, 0, 0, 0, 0, 0, 0, -102, 0, -102, 0, 0, 0, 0, 0, 0, -102, 0, 0, 0, 0, -102, 0, -102, 0, 0, 0, 0, 0,
        // State 143
        0, 0, 0, -104, 0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, 0, 0, 0, 0, 0, -104, 0, -104, 0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, -104, 0, -104, 0, 0, 0, 0, 0,
        // State 144
        0, 0, 0, -109, 0, 0, 0, 0, 0, 0, -109, 0, 0, 0, 0, 0, 0, 0, 0, 0, -109, 0, -109, 0, 0, 0, 0, 0, 0, -109, 0, 0, 0, 0, -109, 0, -109, 0, 0, 0, 0, 0,
        // State 145
        0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, -5, -5, -5,
        // State 146
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, -20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 147
        0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, -56, 0, 0, 0, -56, -56, 0, -56, -56, 0, 0, 0, -56, 0, 0, 0, 0, -56, 0, -56, -56, 0, 0, 0, 0,
        // State 148
        0, -8, -8, -44, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, -8, -8, -8, 0, 0, 0, 0, -8, -8, -8, -8,
        // State 149
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 0,
        // State 150
        0, 0, 0, -107, 0, 0, 0, 0, 0, 0, -107, 0, 0, 0, 0, 0, 0, 0, 0, 0, -107, 0, -107, 0, 0, 0, 0, 0, 0, -107, 0, 0, 0, 0, -107, 0, -107, 0, 0, 0, 0, 0,
        // State 151
        0, 0, 0, 156, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 152
        0, 0, 0, 157, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 153
        0, 0, 0, -123, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -123, 0, -123, 0, 0, 0, 0, 0, 0, -123, 0, 0, 0, 0, -123, 0, -123, 0, 0, 0, 0, 0,
        // State 154
        0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 155
        0, 0, 0, -108, 0, 0, 0, 0, 0, 0, -108, 0, 0, 0, 0, 0, 0, 0, 0, 0, -108, 0, -108, 0, 0, 0, 0, 0, 0, -108, 0, 0, 0, 0, -108, 0, -108, 0, 0, 0, 0, 0,
        // State 156
        0, 0, 0, -105, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, 0, 0, 0, 0, 0, -105, 0, -105, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, -105, 0, -105, 0, 0, 0, 0, 0,
        // State 157
        0, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, -16, 0, 0, 0, 0, -16, 0, 0, 0, 0, -16, -16,
        // State 158
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 161, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 159
        0, -17, -17, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, -17, 0, 0, 0, 0, -17, 0, 0, 0, 0, -17, -17,
        // State 160
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, 0, -23, 0, -23, -23, -23, -23, -23, -23, 0, -23, -23, 0, -23, -23, 0, -23, 0, -23, -23, -23, -23, -23, -23,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 42 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        -99,
        // State 1
        -100,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        -120,
        // State 14
        -58,
        // State 15
        -59,
        // State 16
        -60,
        // State 17
        -61,
        // State 18
        -62,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        -125,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        -121,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        -124,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        -106,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        -112,
        // State 47
        -135,
        // State 48
        -115,
        // State 49
        -110,
        // State 50
        -111,
        // State 51
        -116,
        // State 52
        0,
        // State 53
        -65,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        -72,
        // State 58
        0,
        // State 59
        -71,
        // State 60
        -70,
        // State 61
        -68,
        // State 62
        -69,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        -133,
        // State 72
        -57,
        // State 73
        -35,
        // State 74
        -37,
        // State 75
        -39,
        // State 76
        -41,
        // State 77
        -43,
        // State 78
        -26,
        // State 79
        -25,
        // State 80
        -132,
        // State 81
        -63,
        // State 82
        0,
        // State 83
        0,
        // State 84
        -29,
        // State 85
        0,
        // State 86
        0,
        // State 87
        0,
        // State 88
        0,
        // State 89
        0,
        // State 90
        0,
        // State 91
        0,
        // State 92
        -118,
        // State 93
        0,
        // State 94
        0,
        // State 95
        0,
        // State 96
        -32,
        // State 97
        0,
        // State 98
        0,
        // State 99
        0,
        // State 100
        0,
        // State 101
        0,
        // State 102
        0,
        // State 103
        0,
        // State 104
        0,
        // State 105
        0,
        // State 106
        0,
        // State 107
        0,
        // State 108
        0,
        // State 109
        0,
        // State 110
        0,
        // State 111
        -131,
        // State 112
        0,
        // State 113
        0,
        // State 114
        0,
        // State 115
        0,
        // State 116
        0,
        // State 117
        0,
        // State 118
        0,
        // State 119
        0,
        // State 120
        -119,
        // State 121
        0,
        // State 122
        0,
        // State 123
        -64,
        // State 124
        0,
        // State 125
        -33,
        // State 126
        -34,
        // State 127
        -36,
        // State 128
        -38,
        // State 129
        -40,
        // State 130
        -42,
        // State 131
        0,
        // State 132
        0,
        // State 133
        -27,
        // State 134
        -28,
        // State 135
        0,
        // State 136
        -24,
        // State 137
        0,
        // State 138
        -122,
        // State 139
        -117,
        // State 140
        -101,
        // State 141
        -103,
        // State 142
        -102,
        // State 143
        -104,
        // State 144
        -109,
        // State 145
        0,
        // State 146
        0,
        // State 147
        -56,
        // State 148
        0,
        // State 149
        0,
        // State 150
        -107,
        // State 151
        0,
        // State 152
        0,
        // State 153
        -123,
        // State 154
        0,
        // State 155
        -108,
        // State 156
        -105,
        // State 157
        0,
        // State 158
        0,
        // State 159
        0,
        // State 160
        -23,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            2 => 27,
            4 => 36,
            6 => 24,
            8 => 8,
            10 => 44,
            13 => match state {
                27 => 121,
                _ => 93,
            },
            15 => match state {
                13 => 96,
                29 => 125,
                _ => 13,
            },
            17 => 29,
            18 => 14,
            19 => 15,
            20 => 16,
            21 => 17,
            22 => 18,
            23 => 112,
            24 => 65,
            25 => 151,
            26 => match state {
                21 => 114,
                _ => 94,
            },
            27 => 90,
            28 => match state {
                20 => 113,
                22 => 115,
                35 => 131,
                36 => 132,
                39 => 146,
                40 => 149,
                45 => 158,
                _ => 71,
            },
            29 => 72,
            30 => match state {
                30 => 126,
                _ => 73,
            },
            31 => match state {
                31 => 127,
                _ => 74,
            },
            32 => match state {
                32 => 128,
                _ => 75,
            },
            33 => match state {
                33 => 129,
                _ => 76,
            },
            34 => match state {
                34 => 130,
                _ => 77,
            },
            35 => 46,
            36 => match state {
                4 => 6,
                5 | 8 => 9,
                26 => 38,
                38 | 41..=44 => 41,
                2 => 52,
                3 | 9 | 12 | 21 | 27..=28 => 54,
                6 => 68,
                10 => 91,
                24 => 117,
                _ => 78,
            },
            38 => match state {
                3 | 5 | 8..=9 | 12 | 21 | 27..=28 => 55,
                _ => 79,
            },
            39 => 30,
            40 => 31,
            41 => 32,
            42 => 33,
            43 => 34,
            44 => 19,
            45 => match state {
                3 => 56,
                5 | 8 => 66,
                9 => 87,
                28 => 124,
                _ => 95,
            },
            46 => 28,
            47 => match state {
                8 => 85,
                _ => 67,
            },
            48 => 47,
            49 => match state {
                41 => 150,
                _ => 138,
            },
            50 => match state {
                1 => 51,
                _ => 48,
            },
            52 => 1,
            53 => match state {
                25 | 37 => 120,
                _ => 92,
            },
            54 => match state {
                23 => 37,
                _ => 25,
            },
            55 => 80,
            56 => match state {
                42 => 152,
                43 => 153,
                44 => 154,
                _ => 139,
            },
            57 => 49,
            58 => 69,
            60 => match state {
                19 => 111,
                _ => 81,
            },
            61 => 50,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i16) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""%""###,
            r###""(""###,
            r###""()""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""++""###,
            r###""-""###,
            r###""/""###,
            r###""<""###,
            r###""<-""###,
            r###""=""###,
            r###""=!""###,
            r###""=<""###,
            r###""=>""###,
            r###"">""###,
            r###""[""###,
            r###""]""###,
            r###""_""###,
            r###""{""###,
            r###""|""###,
            r###""}""###,
            r###""،""###,
            r###""أو""###,
            r###""إذن""###,
            r###""تم""###,
            r###""ثم""###,
            r###""حق""###,
            r###""خطأ""###,
            r###""رد""###,
            r###""صح""###,
            r###""صواب""###,
            r###""لو""###,
            r###""ليس""###,
            r###""ليكن""###,
            r###""نص""###,
            r###""نمط""###,
            r###""وإلا""###,
            r###"r#"\"(\\\\.|[^\"\\\\])*\""#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"\\pL\\w*"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input, 'err>
    where 
    {
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'err ())>,
    }
    impl<'input, 'err> __state_machine::ParserDefinition for __StateMachine<'input, 'err>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Vec<Binding<'input>>;
        type StateIndex = i16;
        type Action = i16;
        type ReduceIndex = i16;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 42 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i16) -> i16 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i16, nt: usize) -> i16 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i16) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            true
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            __Symbol::Variant1(recovery)
        }

        fn reduce(
            &mut self,
            action: i16,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i16>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.errors,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'err,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(3, _) if true => Some(0),
            Token(4, _) if true => Some(1),
            Token(5, _) if true => Some(2),
            Token(6, _) if true => Some(3),
            Token(7, _) if true => Some(4),
            Token(8, _) if true => Some(5),
            Token(9, _) if true => Some(6),
            Token(10, _) if true => Some(7),
            Token(11, _) if true => Some(8),
            Token(12, _) if true => Some(9),
            Token(13, _) if true => Some(10),
            Token(14, _) if true => Some(11),
            Token(15, _) if true => Some(12),
            Token(16, _) if true => Some(13),
            Token(17, _) if true => Some(14),
            Token(18, _) if true => Some(15),
            Token(19, _) if true => Some(16),
            Token(20, _) if true => Some(17),
            Token(21, _) if true => Some(18),
            Token(22, _) if true => Some(19),
            Token(23, _) if true => Some(20),
            Token(24, _) if true => Some(21),
            Token(25, _) if true => Some(22),
            Token(26, _) if true => Some(23),
            Token(27, _) if true => Some(24),
            Token(28, _) if true => Some(25),
            Token(29, _) if true => Some(26),
            Token(30, _) if true => Some(27),
            Token(31, _) if true => Some(28),
            Token(32, _) if true => Some(29),
            Token(33, _) if true => Some(30),
            Token(34, _) if true => Some(31),
            Token(35, _) if true => Some(32),
            Token(36, _) if true => Some(33),
            Token(37, _) if true => Some(34),
            Token(38, _) if true => Some(35),
            Token(39, _) if true => Some(36),
            Token(40, _) if true => Some(37),
            Token(0, _) if true => Some(38),
            Token(1, _) if true => Some(39),
            Token(2, _) if true => Some(40),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'err,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 => match __token {
                Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) | Token(25, __tok0) | Token(26, __tok0) | Token(27, __tok0) | Token(28, __tok0) | Token(29, __tok0) | Token(30, __tok0) | Token(31, __tok0) | Token(32, __tok0) | Token(33, __tok0) | Token(34, __tok0) | Token(35, __tok0) | Token(36, __tok0) | Token(37, __tok0) | Token(38, __tok0) | Token(39, __tok0) | Token(40, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'err,
    >(
        __reduce_index: i16,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'err>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 10,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 11,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 12,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 14,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 15,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 16,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 17,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 18,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 19,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 21,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 22,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 23,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 23,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 24,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 24,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 25,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 25,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 26,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 27,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 28,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 35,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 37,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 45,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 45,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            94 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 46,
                }
            }
            95 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 46,
                }
            }
            96 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 47,
                }
            }
            97 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            98 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 48,
                }
            }
            99 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            100 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            101 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            102 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            104 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 49,
                }
            }
            105 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            106 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 49,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 49,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 51,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 52,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 53,
                }
            }
            117 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 54,
                }
            }
            118 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 54,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 55,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 56,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 57,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 57,
                }
            }
            125 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            126 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 58,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 59,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 59,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 60,
                }
            }
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            132 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 61,
                }
            }
            133 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            134 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ProgramParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            let __builder = super::__intern_token::new_builder();
            ProgramParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'err,
        >(
            &self,
            errors: &'err mut Vec<usize>,
            input: &'input str,
        ) -> Result<Vec<Binding<'input>>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    errors,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __error_state: i16,
        __states: & [i16],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.push(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i16>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<Result<Vec<Binding<'input>>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            50 => {
                __reduce50(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            51 => {
                __reduce51(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            52 => {
                __reduce52(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            53 => {
                __reduce53(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            54 => {
                __reduce54(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            55 => {
                __reduce55(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            56 => {
                __reduce56(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            57 => {
                __reduce57(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            58 => {
                __reduce58(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            59 => {
                __reduce59(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            60 => {
                __reduce60(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            61 => {
                __reduce61(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            62 => {
                __reduce62(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            63 => {
                __reduce63(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            64 => {
                __reduce64(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            65 => {
                __reduce65(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            66 => {
                __reduce66(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            67 => {
                __reduce67(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            68 => {
                __reduce68(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            69 => {
                __reduce69(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            70 => {
                __reduce70(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            71 => {
                __reduce71(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            72 => {
                __reduce72(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            73 => {
                __reduce73(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            74 => {
                __reduce74(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            75 => {
                __reduce75(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            76 => {
                __reduce76(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            77 => {
                __reduce77(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            78 => {
                __reduce78(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            79 => {
                __reduce79(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            80 => {
                __reduce80(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            81 => {
                __reduce81(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            82 => {
                __reduce82(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            83 => {
                __reduce83(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            84 => {
                __reduce84(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            85 => {
                __reduce85(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            86 => {
                __reduce86(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            87 => {
                __reduce87(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            88 => {
                __reduce88(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            89 => {
                __reduce89(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            90 => {
                __reduce90(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            91 => {
                __reduce91(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            92 => {
                __reduce92(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            93 => {
                __reduce93(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            94 => {
                __reduce94(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            95 => {
                __reduce95(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            96 => {
                __reduce96(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            97 => {
                __reduce97(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            98 => {
                __reduce98(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            99 => {
                __reduce99(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            100 => {
                __reduce100(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            101 => {
                __reduce101(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            102 => {
                __reduce102(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            103 => {
                __reduce103(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            104 => {
                __reduce104(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            105 => {
                __reduce105(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            106 => {
                __reduce106(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            107 => {
                __reduce107(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            108 => {
                __reduce108(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            109 => {
                __reduce109(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            110 => {
                __reduce110(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            111 => {
                __reduce111(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            112 => {
                __reduce112(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            113 => {
                __reduce113(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            114 => {
                __reduce114(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            115 => {
                __reduce115(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            116 => {
                __reduce116(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            117 => {
                __reduce117(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            118 => {
                __reduce118(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            119 => {
                __reduce119(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            120 => {
                __reduce120(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            121 => {
                __reduce121(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            122 => {
                __reduce122(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            123 => {
                __reduce123(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            124 => {
                __reduce124(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            125 => {
                __reduce125(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            126 => {
                __reduce126(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            127 => {
                __reduce127(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            128 => {
                __reduce128(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            129 => {
                __reduce129(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            130 => {
                __reduce130(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            131 => {
                __reduce131(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            132 => {
                __reduce132(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            133 => {
                __reduce133(errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            134 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (alloc::vec::Vec<Pattern<'input>>, Expr<'input>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (&'input str, ProtoType<'input>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant25(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpcode, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Binding<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Pattern<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ProtoType<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UnOpcode, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Binding<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expr<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Pattern<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ProtoType<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<(&'input str, ProtoType<'input>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant26(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Binding<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant24(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Expr<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Pattern<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<ProtoType<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<&'input str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Vec<&'input str>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant27(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<&'input str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // "،"? = "،" => ActionFn(122);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action122::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // "،"? =  => ActionFn(123);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action123::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Arm> "،") = Arm, "،" => ActionFn(113);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action113::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Arm> "،")+ = Arm, "،" => ActionFn(133);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action133::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Arm> "،")+ = (<Arm> "،")+, Arm, "،" => ActionFn(134);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action134::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> "،") = Expr, "،" => ActionFn(126);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action126::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> "،")+ = Expr, "،" => ActionFn(135);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action135::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce7<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> "،")+ = (<Expr> "،")+, Expr, "،" => ActionFn(136);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action136::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<ID> "،") = ID, "،" => ActionFn(108);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action108::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<ID> "،")+ = ID, "،" => ActionFn(137);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action137::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce10<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<ID> "،")+ = (<ID> "،")+, ID, "،" => ActionFn(138);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action138::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce11<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<PatternH> "،") = PatternH, "،" => ActionFn(116);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action116::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce12<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<PatternH> "،")+ = PatternH, "،" => ActionFn(139);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action139::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce13<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<PatternH> "،")+ = (<PatternH> "،")+, PatternH, "،" => ActionFn(140);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action140::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce14<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Type> "،") = Type, "،" => ActionFn(119);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action119::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce15<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Type> "،")+ = Type, "،" => ActionFn(141);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action141::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce16<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Type> "،")+ = (<Type> "،")+, Type, "،" => ActionFn(142);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action142::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce17<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(88);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action88::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce18<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(87);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action87::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce19<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Arm = Pattern+, "=>", Expr => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 13)
    }
    pub(crate) fn __reduce20<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Arm? = Arm => ActionFn(109);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action109::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce21<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Arm? =  => ActionFn(110);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action110::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce22<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = "لو", Expr, "إذن", Expr, "وإلا", Expr, "تم" => ActionFn(42);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant5(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action42::<>(errors, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (7, 15)
    }
    pub(crate) fn __reduce23<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = "{", Comma<Arm>, "}" => ActionFn(43);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action43::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce24<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = Literal => ActionFn(44);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce25<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = ID => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce26<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = "(", Comma2<Expr>, ")" => ActionFn(46);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant14(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action46::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce27<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = "(", Expr, ")" => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action47::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce28<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base = error => ActionFn(146);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action146::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce29<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base* =  => ActionFn(72);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action72::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 16)
    }
    pub(crate) fn __reduce30<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base* = Base+ => ActionFn(73);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce31<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base+ = Base => ActionFn(120);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action120::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce32<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Base+ = Base+, Base => ActionFn(121);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action121::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce33<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op0, Expr1> = Class<Op0, Expr1>, Op0, Expr1 => ActionFn(84);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action84::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 18)
    }
    pub(crate) fn __reduce34<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op0, Expr1> = Expr1 => ActionFn(85);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action85::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce35<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op1, Expr2> = Class<Op1, Expr2>, Op1, Expr2 => ActionFn(82);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce36<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op1, Expr2> = Expr2 => ActionFn(83);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action83::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce37<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op2, Expr3> = Class<Op2, Expr3>, Op2, Expr3 => ActionFn(80);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action80::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce38<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op2, Expr3> = Expr3 => ActionFn(81);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action81::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce39<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op3, Expr4> = Class<Op3, Expr4>, Op3, Expr4 => ActionFn(78);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action78::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 21)
    }
    pub(crate) fn __reduce40<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op3, Expr4> = Expr4 => ActionFn(79);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce41<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op4, Expr5> = Class<Op4, Expr5>, Op4, Expr5 => ActionFn(76);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action76::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 22)
    }
    pub(crate) fn __reduce42<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Class<Op4, Expr5> = Expr5 => ActionFn(77);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce43<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma2<Expr> = (<Expr> "،")+, Expr, "،" => ActionFn(127);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action127::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce44<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma2<Expr> = (<Expr> "،")+, Expr => ActionFn(128);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action128::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 23)
    }
    pub(crate) fn __reduce45<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma2<PatternH> = (<PatternH> "،")+, PatternH, "،" => ActionFn(129);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action129::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 24)
    }
    pub(crate) fn __reduce46<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma2<PatternH> = (<PatternH> "،")+, PatternH => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action130::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 24)
    }
    pub(crate) fn __reduce47<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma2<Type> = (<Type> "،")+, Type, "،" => ActionFn(131);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action131::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (3, 25)
    }
    pub(crate) fn __reduce48<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma2<Type> = (<Type> "،")+, Type => ActionFn(132);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action132::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 25)
    }
    pub(crate) fn __reduce49<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<Arm> = Arm => ActionFn(92);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action92::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce50<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<Arm> = (<Arm> "،")+, Arm => ActionFn(149);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action149::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 26)
    }
    pub(crate) fn __reduce51<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<Arm> = (<Arm> "،")+ => ActionFn(150);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action150::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce52<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<ID> = ID => ActionFn(94);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action94::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce53<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<ID> = (<ID> "،")+, ID => ActionFn(153);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action153::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 27)
    }
    pub(crate) fn __reduce54<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<ID> = (<ID> "،")+ => ActionFn(154);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action154::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce55<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr, "[", Expr, "]" => ActionFn(33);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action33::<>(errors, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (4, 28)
    }
    pub(crate) fn __reduce56<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr0 => ActionFn(34);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce57<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = Class<Op0, Expr1> => ActionFn(35);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce58<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr1 = Class<Op1, Expr2> => ActionFn(36);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce59<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr2 = Class<Op2, Expr3> => ActionFn(37);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce60<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr3 = Class<Op3, Expr4> => ActionFn(38);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce61<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr4 = Class<Op4, Expr5> => ActionFn(39);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce62<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr5 = UnaryClass<Op5, Term> => ActionFn(40);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce63<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FnBinding = "رد", ID, "=", "{", Comma<Arm>, "}" => ActionFn(12);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action12::<>(errors, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (6, 35)
    }
    pub(crate) fn __reduce64<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ID = r#"\\pL\\w*"# => ActionFn(70);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce65<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ID? = ID => ActionFn(104);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action104::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce66<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ID? =  => ActionFn(105);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action105::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 37)
    }
    pub(crate) fn __reduce67<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Literal = r#"\"(\\\\.|[^\"\\\\])*\""# => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce68<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Literal = r#"[0-9]+"# => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce69<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Literal = "صواب" => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce70<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Literal = "خطأ" => ActionFn(52);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce71<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Literal = "()" => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce72<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op0 = "ثم" => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce73<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op0 = "أو" => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce74<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op1 = "<" => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce75<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op1 = ">" => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce76<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op1 = "=<" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce77<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op1 = "=>" => ActionFn(59);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action59::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce78<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op2 = "=" => ActionFn(60);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce79<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op2 = "=!" => ActionFn(61);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce80<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op3 = "+" => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action62::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce81<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op3 = "-" => ActionFn(63);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action63::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce82<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op3 = "++" => ActionFn(64);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce83<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "*" => ActionFn(65);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce84<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "/" => ActionFn(66);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce85<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "%" => ActionFn(67);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce86<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op5 = "ليس" => ActionFn(68);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce87<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op5 = "-" => ActionFn(69);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce88<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern = Literal => ActionFn(14);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce89<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern = "_" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce90<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern = ID => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce91<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern = "(", Comma2<PatternH>, ")" => ActionFn(17);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 45)
    }
    pub(crate) fn __reduce92<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern = "(", PatternH, ")" => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 45)
    }
    pub(crate) fn __reduce93<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern = error => ActionFn(147);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action147::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce94<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern+ = Pattern => ActionFn(90);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action90::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce95<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Pattern+ = Pattern+, Pattern => ActionFn(91);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action91::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 46)
    }
    pub(crate) fn __reduce96<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // PatternH = ID, Pattern => ActionFn(20);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 47)
    }
    pub(crate) fn __reduce97<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // PatternH = Pattern => ActionFn(21);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce98<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(155);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action155::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (0, 48)
    }
    pub(crate) fn __reduce99<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Program = Statement+ => ActionFn(156);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action156::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce100<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = "()" => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce101<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = "صح" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce102<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = "حق" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce103<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = "نص" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce104<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = "(", Type, ")" => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 49)
    }
    pub(crate) fn __reduce105<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = ID => ActionFn(29);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce106<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = ID, SimpleType => ActionFn(30);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action30::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 49)
    }
    pub(crate) fn __reduce107<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = "(", Comma2<Type>, ")" => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant16(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 49)
    }
    pub(crate) fn __reduce108<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SimpleType = error => ActionFn(148);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action148::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce109<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement = TypeDecl => ActionFn(3);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce110<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement = ValBinding => ActionFn(4);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce111<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement = FnBinding => ActionFn(5);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce112<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(100);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action100::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (0, 51)
    }
    pub(crate) fn __reduce113<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(101);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action101::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce114<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(102);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce115<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(103);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action103::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (2, 52)
    }
    pub(crate) fn __reduce116<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SumVarDecl = "|", ID, Type => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (3, 53)
    }
    pub(crate) fn __reduce117<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SumVarDecl+ = SumVarDecl => ActionFn(96);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce118<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // SumVarDecl+ = SumVarDecl+, SumVarDecl => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant26(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce119<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = Base => ActionFn(151);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action151::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce120<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = Base, Base+ => ActionFn(152);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action152::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 55)
    }
    pub(crate) fn __reduce121<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = SimpleType => ActionFn(22);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce122<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = SimpleType, "<-", Type => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 56)
    }
    pub(crate) fn __reduce123<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeDecl = "نمط", ID, TypeVars, "=", SumVarDecl+ => ActionFn(157);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant26(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action157::<>(errors, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (5, 57)
    }
    pub(crate) fn __reduce124<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeDecl = "نمط", ID, "=", SumVarDecl+ => ActionFn(158);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant26(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action158::<>(errors, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (4, 57)
    }
    pub(crate) fn __reduce125<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeVars = "()" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce126<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeVars = ID => ActionFn(8);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce127<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeVars = "(", Comma<ID>, ")" => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (3, 58)
    }
    pub(crate) fn __reduce128<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeVars? = TypeVars => ActionFn(98);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce129<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // TypeVars? =  => ActionFn(99);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action99::<>(errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (0, 59)
    }
    pub(crate) fn __reduce130<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // UnaryClass<Op5, Term> = Op5, UnaryClass<Op5, Term> => ActionFn(74);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action74::<>(errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 60)
    }
    pub(crate) fn __reduce131<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // UnaryClass<Op5, Term> = Term => ActionFn(75);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce132<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ValBinding = "ليكن", Pattern, "=", Expr => ActionFn(11);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action11::<>(errors, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (4, 61)
    }
    pub(crate) fn __reduce133<
        'input,
        'err,
    >(
        errors: &'err mut Vec<usize>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(1);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 62)
    }
}
pub use self::__parse__Program::ProgramParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::{
    ast::*,
    types::{ProtoType, Literal, BinOpcode, UnOpcode}
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^(\"(\\\\[\u{0}-\t\u{b}-\u{10ffff}]|[\u{0}-!\\#-\\[\\]-\u{10ffff}])*\")", false),
            ("^([0-9]+)", false),
            ("^([A-Za-zªµºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬˮͰ-ʹͶ-ͷͺ-ͽͿΆΈ-ΊΌΎ-ΡΣ-ϵϷ-ҁҊ-ԯԱ-Ֆՙՠ-ֈא-תׯ-ײؠ-يٮ-ٯٱ-ۓەۥ-ۦۮ-ۯۺ-ۼۿܐܒ-ܯݍ-ޥޱߊ-ߪߴ-ߵߺࠀ-ࠕࠚࠤࠨࡀ-ࡘࡠ-ࡪࢠ-ࢴࢶ-ࣇऄ-हऽॐक़-ॡॱ-ঀঅ-ঌএ-ঐও-নপ-রলশ-হঽৎড়-ঢ়য়-ৡৰ-ৱৼਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹਖ਼-ੜਫ਼ੲ-ੴઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હઽૐૠ-ૡૹଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହଽଡ଼-ଢ଼ୟ-ୡୱஃஅ-ஊஎ-ஐஒ-கங-சஜஞ-டண-தந-பம-ஹௐఅ-ఌఎ-ఐఒ-నప-హఽౘ-ౚౠ-ౡಀಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹಽೞೠ-ೡೱ-ೲഄ-ഌഎ-ഐഒ-ഺഽൎൔ-ൖൟ-ൡൺ-ൿඅ-ඖක-නඳ-රලව-ෆก-ะา-ำเ-ๆກ-ຂຄຆ-ຊຌ-ຣລວ-ະາ-ຳຽເ-ໄໆໜ-ໟༀཀ-ཇཉ-ཬྈ-ྌက-ဪဿၐ-ၕၚ-ၝၡၥ-ၦၮ-ၰၵ-ႁႎႠ-ჅჇჍა-ჺჼ-ቈቊ-ቍቐ-ቖቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛱ-ᛸᜀ-ᜌᜎ-ᜑᜠ-ᜱᝀ-ᝑᝠ-ᝬᝮ-ᝰក-ឳៗៜᠠ-ᡸᢀ-ᢄᢇ-ᢨᢪᢰ-ᣵᤀ-ᤞᥐ-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉᨀ-ᨖᨠ-ᩔᪧᬅ-ᬳᭅ-ᭋᮃ-ᮠᮮ-ᮯᮺ-ᯥᰀ-ᰣᱍ-ᱏᱚ-ᱽᲀ-ᲈᲐ-ᲺᲽ-Ჿᳩ-ᳬᳮ-ᳳᳵ-ᳶᳺᴀ-ᶿḀ-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙὛὝὟ-ώᾀ-ᾴᾶ-ᾼιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼⁱⁿₐ-ₜℂℇℊ-ℓℕℙ-ℝℤΩℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎↃ-ↄⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳮⳲ-ⳳⴀ-ⴥⴧⴭⴰ-ⵧⵯⶀ-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⸯ々-〆〱-〵〻-〼ぁ-ゖゝ-ゟァ-ヺー-ヿㄅ-ㄯㄱ-ㆎㆠ-ㆿㇰ-ㇿ㐀-䶿一-鿼ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘟꘪ-ꘫꙀ-ꙮꙿ-ꚝꚠ-ꛥꜗ-ꜟꜢ-ꞈꞋ-ꞿꟂ-ꟊꟵ-ꠁꠃ-ꠅꠇ-ꠊꠌ-ꠢꡀ-ꡳꢂ-ꢳꣲ-ꣷꣻꣽ-ꣾꤊ-ꤥꤰ-ꥆꥠ-ꥼꦄ-ꦲꧏꧠ-ꧤꧦ-ꧯꧺ-ꧾꨀ-ꨨꩀ-ꩂꩄ-ꩋꩠ-ꩶꩺꩾ-ꪯꪱꪵ-ꪶꪹ-ꪽꫀꫂꫛ-ꫝꫠ-ꫪꫲ-ꫴꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭩꭰ-ꯢ가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִײַ-ﬨשׁ-זּטּ-לּמּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻﹰ-ﹴﹶ-ﻼＡ-Ｚａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐊀-𐊜𐊠-𐋐𐌀-𐌟𐌭-𐍀𐍂-𐍉𐍐-𐍵𐎀-𐎝𐎠-𐏃𐏈-𐏏𐐀-𐒝𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈𐠊-𐠵𐠷-𐠸𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀𐨐-𐨓𐨕-𐨗𐨙-𐨵𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫤𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𐴀-𐴣𐺀-𐺩𐺰-𐺱𐼀-𐼜𐼧𐼰-𐽅𐾰-𐿄𐿠-𐿶𑀃-𑀷𑂃-𑂯𑃐-𑃨𑄃-𑄦𑅄𑅇𑅐-𑅲𑅶𑆃-𑆲𑇁-𑇄𑇚𑇜𑈀-𑈑𑈓-𑈫𑊀-𑊆𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋞𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌽𑍐𑍝-𑍡𑐀-𑐴𑑇-𑑊𑑟-𑑡𑒀-𑒯𑓄-𑓅𑓇𑖀-𑖮𑗘-𑗛𑘀-𑘯𑙄𑚀-𑚪𑚸𑜀-𑜚𑠀-𑠫𑢠-𑣟𑣿-𑤆𑤉𑤌-𑤓𑤕-𑤖𑤘-𑤯𑤿𑥁𑦠-𑦧𑦪-𑧐𑧡𑧣𑨀𑨋-𑨲𑨺𑩐𑩜-𑪉𑪝𑫀-𑫸𑰀-𑰈𑰊-𑰮𑱀𑱲-𑲏𑴀-𑴆𑴈-𑴉𑴋-𑴰𑵆𑵠-𑵥𑵧-𑵨𑵪-𑶉𑶘𑻠-𑻲𑾰𒀀-𒎙𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖫐-𖫭𖬀-𖬯𖭀-𖭃𖭣-𖭷𖭽-𖮏𖹀-𖹿𖼀-𖽊𖽐𖾓-𖾟𖿠-𖿡𖿣𗀀-𘟷𘠀-𘳕𘴀-𘴈𛀀-𛄞𛅐-𛅒𛅤-𛅧𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𞄀-𞄬𞄷-𞄽𞅎𞋀-𞋫𞠀-𞣄𞤀-𞥃𞥋𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤𞸧𞸩-𞸲𞸴-𞸷𞸹𞸻𞹂𞹇𞹉𞹋𞹍-𞹏𞹑-𞹒𞹔𞹗𞹙𞹛𞹝𞹟𞹡-𞹢𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻𠀀-𪛝𪜀-𫜴𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀𰀀-𱍊][0-9A-Z_a-zªµºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬˮ\u{300}-ʹͶ-ͷͺ-ͽͿΆΈ-ΊΌΎ-ΡΣ-ϵϷ-ҁ\u{483}-ԯԱ-Ֆՙՠ-ֈ\u{591}-\u{5bd}\u{5bf}\u{5c1}-\u{5c2}\u{5c4}-\u{5c5}\u{5c7}א-תׯ-ײ\u{610}-\u{61a}ؠ-٩ٮ-ۓە-\u{6dc}\u{6df}-\u{6e8}\u{6ea}-ۼۿܐ-\u{74a}ݍ-ޱ߀-ߵߺ\u{7fd}ࠀ-\u{82d}ࡀ-\u{85b}ࡠ-ࡪࢠ-ࢴࢶ-ࣇ\u{8d3}-\u{8e1}\u{8e3}-\u{963}०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রলশ-হ\u{9bc}-\u{9c4}ে-ৈো-ৎ\u{9d7}ড়-ঢ়য়-\u{9e3}০-ৱৼ\u{9fe}\u{a01}-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ\u{a3c}ਾ-\u{a42}\u{a47}-\u{a48}\u{a4b}-\u{a4d}\u{a51}ਖ਼-ੜਫ਼੦-\u{a75}\u{a81}-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ\u{abc}-\u{ac5}\u{ac7}-ૉો-\u{acd}ૐૠ-\u{ae3}૦-૯ૹ-\u{aff}\u{b01}-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ\u{b3c}-\u{b44}େ-ୈୋ-\u{b4d}\u{b55}-\u{b57}ଡ଼-ଢ଼ୟ-\u{b63}୦-୯ୱ\u{b82}-ஃஅ-ஊஎ-ஐஒ-கங-சஜஞ-டண-தந-பம-ஹ\u{bbe}-ூெ-ைொ-\u{bcd}ௐ\u{bd7}௦-௯\u{c00}-ఌఎ-ఐఒ-నప-హఽ-ౄ\u{c46}-\u{c48}\u{c4a}-\u{c4d}\u{c55}-\u{c56}ౘ-ౚౠ-\u{c63}౦-౯ಀ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ\u{cbc}-ೄ\u{cc6}-ೈೊ-\u{ccd}\u{cd5}-\u{cd6}ೞೠ-\u{ce3}೦-೯ೱ-ೲ\u{d00}-ഌഎ-ഐഒ-\u{d44}െ-ൈൊ-ൎൔ-\u{d57}ൟ-\u{d63}൦-൯ൺ-ൿ\u{d81}-ඃඅ-ඖක-නඳ-රලව-ෆ\u{dca}\u{dcf}-\u{dd4}\u{dd6}ෘ-\u{ddf}෦-෯ෲ-ෳก-\u{e3a}เ-\u{e4e}๐-๙ກ-ຂຄຆ-ຊຌ-ຣລວ-ຽເ-ໄໆ\u{ec8}-\u{ecd}໐-໙ໜ-ໟༀ\u{f18}-\u{f19}༠-༩\u{f35}\u{f37}\u{f39}༾-ཇཉ-ཬ\u{f71}-\u{f84}\u{f86}-\u{f97}\u{f99}-\u{fbc}\u{fc6}က-၉ၐ-\u{109d}Ⴀ-ჅჇჍა-ჺჼ-ቈቊ-ቍቐ-ቖቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ\u{135d}-\u{135f}ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-\u{1714}ᜠ-\u{1734}ᝀ-\u{1753}ᝠ-ᝬᝮ-ᝰ\u{1772}-\u{1773}ក-\u{17d3}ៗៜ-\u{17dd}០-៩\u{180b}-\u{180d}᠐-᠙ᠠ-ᡸᢀ-ᢪᢰ-ᣵᤀ-ᤞ\u{1920}-ᤫᤰ-\u{193b}᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-\u{1a1b}ᨠ-\u{1a5e}\u{1a60}-\u{1a7c}\u{1a7f}-᪉᪐-᪙ᪧ\u{1ab0}-\u{1ac0}\u{1b00}-ᭋ᭐-᭙\u{1b6b}-\u{1b73}\u{1b80}-᯳ᰀ-\u{1c37}᱀-᱉ᱍ-ᱽᲀ-ᲈᲐ-ᲺᲽ-Ჿ\u{1cd0}-\u{1cd2}\u{1cd4}-ᳺᴀ-\u{1df9}\u{1dfb}-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙὛὝὟ-ώᾀ-ᾴᾶ-ᾼιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔ⁱⁿₐ-ₜ\u{20d0}-\u{20f0}ℂℇℊ-ℓℕℙ-ℝℤΩℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧⴭⴰ-ⵧⵯ\u{2d7f}-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞ\u{2de0}-\u{2dff}ⸯ々-〇〡-\u{302f}〱-〵〸-〼ぁ-ゖ\u{3099}-\u{309a}ゝ-ゟァ-ヺー-ヿㄅ-ㄯㄱ-ㆎㆠ-ㆿㇰ-ㇿ㐀-䶿一-鿼ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-\u{a672}\u{a674}-\u{a67d}ꙿ-\u{a6f1}ꜗ-ꜟꜢ-ꞈꞋ-ꞿꟂ-ꟊꟵ-ꠧ\u{a82c}ꡀ-ꡳꢀ-\u{a8c5}꣐-꣙\u{a8e0}-ꣷꣻꣽ-\u{a92d}ꤰ-꥓ꥠ-ꥼ\u{a980}-꧀ꧏ-꧙ꧠ-ꧾꨀ-\u{aa36}ꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-\u{aaf6}ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭩꭰ-ꯪ꯬-\u{abed}꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ\u{fe00}-\u{fe0f}\u{fe20}-\u{fe2f}︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴\u{101fd}𐊀-𐊜𐊠-𐋐\u{102e0}𐌀-𐌟𐌭-𐍊𐍐-\u{1037a}𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈𐠊-𐠵𐠷-𐠸𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-\u{10a03}\u{10a05}-\u{10a06}\u{10a0c}-𐨓𐨕-𐨗𐨙-𐨵\u{10a38}-\u{10a3a}\u{10a3f}𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-\u{10ae6}𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𐴀-\u{10d27}𐴰-𐴹𐺀-𐺩\u{10eab}-\u{10eac}𐺰-𐺱𐼀-𐼜𐼧𐼰-\u{10f50}𐾰-𐿄𐿠-𐿶𑀀-\u{11046}𑁦-𑁯\u{1107f}-\u{110ba}𑃐-𑃨𑃰-𑃹\u{11100}-\u{11134}𑄶-𑄿𑅄-𑅇𑅐-\u{11173}𑅶\u{11180}-𑇄\u{111c9}-\u{111cc}𑇎-𑇚𑇜𑈀-𑈑𑈓-\u{11237}\u{1123e}𑊀-𑊆𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-\u{112ea}𑋰-𑋹\u{11300}-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹\u{1133b}-𑍄𑍇-𑍈𑍋-𑍍𑍐\u{11357}𑍝-𑍣\u{11366}-\u{1136c}\u{11370}-\u{11374}𑐀-𑑊𑑐-𑑙\u{1145e}-𑑡𑒀-𑓅𑓇𑓐-𑓙𑖀-\u{115b5}𑖸-\u{115c0}𑗘-\u{115dd}𑘀-\u{11640}𑙄𑙐-𑙙𑚀-𑚸𑛀-𑛉𑜀-𑜚\u{1171d}-\u{1172b}𑜰-𑜹𑠀-\u{1183a}𑢠-𑣩𑣿-𑤆𑤉𑤌-𑤓𑤕-𑤖𑤘-𑤵𑤷-𑤸\u{1193b}-\u{11943}𑥐-𑥙𑦠-𑦧𑦪-\u{119d7}\u{119da}-𑧡𑧣-𑧤𑨀-\u{11a3e}\u{11a47}𑩐-\u{11a99}𑪝𑫀-𑫸𑰀-𑰈𑰊-\u{11c36}\u{11c38}-𑱀𑱐-𑱙𑱲-𑲏\u{11c92}-\u{11ca7}𑲩-\u{11cb6}𑴀-𑴆𑴈-𑴉𑴋-\u{11d36}\u{11d3a}\u{11d3c}-\u{11d3d}\u{11d3f}-\u{11d47}𑵐-𑵙𑵠-𑵥𑵧-𑵨𑵪-𑶎\u{11d90}-\u{11d91}𑶓-𑶘𑶠-𑶩𑻠-𑻶𑾰𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭\u{16af0}-\u{16af4}𖬀-\u{16b36}𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖹀-𖹿𖼀-𖽊\u{16f4f}-𖾇\u{16f8f}-𖾟𖿠-𖿡𖿣-\u{16fe4}𖿰-𖿱𗀀-𘟷𘠀-𘳕𘴀-𘴈𛀀-𛄞𛅐-𛅒𛅤-𛅧𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙\u{1bc9d}-\u{1bc9e}\u{1d165}-\u{1d169}𝅭-\u{1d172}\u{1d17b}-\u{1d182}\u{1d185}-\u{1d18b}\u{1d1aa}-\u{1d1ad}\u{1d242}-\u{1d244}𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿\u{1da00}-\u{1da36}\u{1da3b}-\u{1da6c}\u{1da75}\u{1da84}\u{1da9b}-\u{1da9f}\u{1daa1}-\u{1daaf}\u{1e000}-\u{1e006}\u{1e008}-\u{1e018}\u{1e01b}-\u{1e021}\u{1e023}-\u{1e024}\u{1e026}-\u{1e02a}𞄀-𞄬\u{1e130}-𞄽𞅀-𞅉𞅎𞋀-𞋹𞠀-𞣄\u{1e8d0}-\u{1e8d6}𞤀-𞥋𞥐-𞥙𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤𞸧𞸩-𞸲𞸴-𞸷𞸹𞸻𞹂𞹇𞹉𞹋𞹍-𞹏𞹑-𞹒𞹔𞹗𞹙𞹛𞹝𞹟𞹡-𞹢𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉🯰-🯹𠀀-𪛝𪜀-𫜴𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀𰀀-𱍊\u{e0100}-\u{e01ef}]*)", false),
            ("^(%)", false),
            ("^(\\()", false),
            ("^(\\(\\))", false),
            ("^(\\))", false),
            ("^(\\*)", false),
            ("^(\\+)", false),
            ("^(\\+\\+)", false),
            ("^(\\-)", false),
            ("^(/)", false),
            ("^(<)", false),
            ("^(<\\-)", false),
            ("^(=)", false),
            ("^(=!)", false),
            ("^(=<)", false),
            ("^(=>)", false),
            ("^(>)", false),
            ("^(\\[)", false),
            ("^(\\])", false),
            ("^(_)", false),
            ("^(\\{)", false),
            ("^(\\|)", false),
            ("^(\\})", false),
            ("^(،)", false),
            ("^(أو)", false),
            ("^(إذن)", false),
            ("^(تم)", false),
            ("^(ثم)", false),
            ("^(حق)", false),
            ("^(خطأ)", false),
            ("^(رد)", false),
            ("^(صح)", false),
            ("^(صواب)", false),
            ("^(لو)", false),
            ("^(ليس)", false),
            ("^(ليكن)", false),
            ("^(نص)", false),
            ("^(نمط)", false),
            ("^(وإلا)", false),
            (r"^(\s*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Vec<Binding<'input>>, usize),
) -> Vec<Binding<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, alloc::vec::Vec<Binding<'input>>, usize),
) -> Vec<Binding<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action3<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Binding<'input>, usize),
) -> Binding<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action4<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Binding<'input>, usize),
) -> Binding<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action5<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Binding<'input>, usize),
) -> Binding<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action6<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, &'input str, usize),
    (_, vars, _): (usize, core::option::Option<Vec<&'input str>>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, variants, _): (usize, alloc::vec::Vec<(&'input str, ProtoType<'input>)>, usize),
) -> Binding<'input>
{
    {
        Binding::Type { name, vars: vars.unwrap_or(vec![]), variants }
    }
}

#[allow(unused_variables)]
fn __action7<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Vec<&'input str>
{
    vec![]
}

#[allow(unused_variables)]
fn __action8<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Vec<&'input str>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action9<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<&'input str>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<&'input str>
{
    __0
}

#[allow(unused_variables)]
fn __action10<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, ProtoType<'input>, usize),
) -> (&'input str, ProtoType<'input>)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action11<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Pattern<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Expr<'input>, usize),
) -> Binding<'input>
{
    Binding::Value(__0, __1, false)
}

#[allow(unused_variables)]
fn __action12<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Binding<'input>
{
    Binding::Value(Pattern::Bind(name), Expr::Closure(v), true)
}

#[allow(unused_variables)]
fn __action13<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, alloc::vec::Vec<Pattern<'input>>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Expr<'input>, usize),
) -> (alloc::vec::Vec<Pattern<'input>>, Expr<'input>)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action14<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Literal<'input>, usize),
) -> Pattern<'input>
{
    Pattern::Literal(__0)
}

#[allow(unused_variables)]
fn __action15<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Pattern<'input>
{
    Pattern::Wild
}

#[allow(unused_variables)]
fn __action16<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Pattern<'input>
{
    Pattern::Bind(__0)
}

#[allow(unused_variables)]
fn __action17<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, Vec<Pattern<'input>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Pattern<'input>
{
    Pattern::Tuple(v)
}

#[allow(unused_variables)]
fn __action18<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Pattern<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Pattern<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action19<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, _, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize),
    (_, end, _): (usize, usize, usize),
) -> Pattern<'input>
{
    { errors.push(start); Pattern::Error(start, end) }
}

#[allow(unused_variables)]
fn __action20<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, id, _): (usize, &'input str, usize),
    (_, field, _): (usize, Pattern<'input>, usize),
) -> Pattern<'input>
{
    Pattern::SumVar(id, Box::new(field))
}

#[allow(unused_variables)]
fn __action21<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Pattern<'input>, usize),
) -> Pattern<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action22<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, ProtoType<'input>, usize),
) -> ProtoType<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action23<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, from, _): (usize, ProtoType<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, to, _): (usize, ProtoType<'input>, usize),
) -> ProtoType<'input>
{
    ProtoType::Function(Box::new(from), Box::new(to))
}

#[allow(unused_variables)]
fn __action24<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ProtoType<'input>
{
    ProtoType::Unit
}

#[allow(unused_variables)]
fn __action25<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ProtoType<'input>
{
    ProtoType::Int
}

#[allow(unused_variables)]
fn __action26<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ProtoType<'input>
{
    ProtoType::Bool
}

#[allow(unused_variables)]
fn __action27<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ProtoType<'input>
{
    ProtoType::String
}

#[allow(unused_variables)]
fn __action28<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, ProtoType<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ProtoType<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action29<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ProtoType<'input>
{
    ProtoType::Generic(__0)
}

#[allow(unused_variables)]
fn __action30<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, name, _): (usize, &'input str, usize),
    (_, tp, _): (usize, ProtoType<'input>, usize),
) -> ProtoType<'input>
{
    ProtoType::Sum(name, Box::new(tp))
}

#[allow(unused_variables)]
fn __action31<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<ProtoType<'input>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ProtoType<'input>
{
    ProtoType::Tuple(__0)
}

#[allow(unused_variables)]
fn __action32<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, _, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize),
    (_, end, _): (usize, usize, usize),
) -> ProtoType<'input>
{
    { errors.push(start); ProtoType::Error(start, end) }
}

#[allow(unused_variables)]
fn __action33<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, e1, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e2, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    Expr::BinOp(Box::new(e1), BinOpcode::Index, Box::new(e2))
}

#[allow(unused_variables)]
fn __action34<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action35<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action36<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action37<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action38<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action39<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action40<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action41<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, hd, _): (usize, Expr<'input>, usize),
    (_, v, _): (usize, alloc::vec::Vec<Expr<'input>>, usize),
) -> Expr<'input>
{
    {
        let mut expr = hd;
        for e in v {
            expr = Expr::Application(Box::new(expr), Box::new(e));
        }
        expr
    }
}

#[allow(unused_variables)]
fn __action42<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, cond, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, a, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, b, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    {
        Expr::Conditional(Box::new(cond), Box::new(a), Box::new(b))
    }
}

#[allow(unused_variables)]
fn __action43<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    Expr::Closure(v)
}

#[allow(unused_variables)]
fn __action44<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Literal<'input>, usize),
) -> Expr<'input>
{
    Expr::Literal(__0)
}

#[allow(unused_variables)]
fn __action45<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, bound, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    Expr::Bound(bound)
}

#[allow(unused_variables)]
fn __action46<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Expr<'input>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    Expr::Tuple(__0)
}

#[allow(unused_variables)]
fn __action47<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action48<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, _, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize),
    (_, end, _): (usize, usize, usize),
) -> Expr<'input>
{
    { errors.push(start); Expr::Error(start, end) }
}

#[allow(unused_variables)]
fn __action49<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Literal<'input>
{
    Literal::String(&__0[1..(__0.len()-1)])
}

#[allow(unused_variables)]
fn __action50<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Literal<'input>
{
    Literal::Int(isize::from_str_radix(__0, 10).unwrap())
}

#[allow(unused_variables)]
fn __action51<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Literal<'input>
{
    Literal::Bool(true)
}

#[allow(unused_variables)]
fn __action52<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Literal<'input>
{
    Literal::Bool(false)
}

#[allow(unused_variables)]
fn __action53<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Literal<'input>
{
    Literal::Unit
}

#[allow(unused_variables)]
fn __action54<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::And
}

#[allow(unused_variables)]
fn __action55<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::Or
}

#[allow(unused_variables)]
fn __action56<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::Greater
}

#[allow(unused_variables)]
fn __action57<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::Less
}

#[allow(unused_variables)]
fn __action58<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::GreaterEq
}

#[allow(unused_variables)]
fn __action59<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::LessEq
}

#[allow(unused_variables)]
fn __action60<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::Equal
}

#[allow(unused_variables)]
fn __action61<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::NotEq
}

#[allow(unused_variables)]
fn __action62<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::Add
}

#[allow(unused_variables)]
fn __action63<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::Sub
}

#[allow(unused_variables)]
fn __action64<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::Concat
}

#[allow(unused_variables)]
fn __action65<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::Mul
}

#[allow(unused_variables)]
fn __action66<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::Div
}

#[allow(unused_variables)]
fn __action67<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpcode
{
    BinOpcode::Mod
}

#[allow(unused_variables)]
fn __action68<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> UnOpcode
{
    UnOpcode::Not
}

#[allow(unused_variables)]
fn __action69<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> UnOpcode
{
    UnOpcode::Minus
}

#[allow(unused_variables)]
fn __action70<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action71<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Expr<'input>>, usize),
    (_, last, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, core::option::Option<&'input str>, usize),
) -> Vec<Expr<'input>>
{
    {
        let mut v = v;
        v.push(last);
        v
    }
}

#[allow(unused_variables)]
fn __action72<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Expr<'input>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action73<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Expr<'input>>, usize),
) -> alloc::vec::Vec<Expr<'input>>
{
    v
}

#[allow(unused_variables)]
fn __action74<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, op, _): (usize, UnOpcode, usize),
    (_, e, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    Expr::UnOp(op, Box::new(e))
}

#[allow(unused_variables)]
fn __action75<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action76<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, op, _): (usize, BinOpcode, usize),
    (_, r, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    Expr::BinOp(Box::new(l), op, Box::new(r))
}

#[allow(unused_variables)]
fn __action77<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action78<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, op, _): (usize, BinOpcode, usize),
    (_, r, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    Expr::BinOp(Box::new(l), op, Box::new(r))
}

#[allow(unused_variables)]
fn __action79<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action80<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, op, _): (usize, BinOpcode, usize),
    (_, r, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    Expr::BinOp(Box::new(l), op, Box::new(r))
}

#[allow(unused_variables)]
fn __action81<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action82<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, op, _): (usize, BinOpcode, usize),
    (_, r, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    Expr::BinOp(Box::new(l), op, Box::new(r))
}

#[allow(unused_variables)]
fn __action83<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action84<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, op, _): (usize, BinOpcode, usize),
    (_, r, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    Expr::BinOp(Box::new(l), op, Box::new(r))
}

#[allow(unused_variables)]
fn __action85<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action86<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<ProtoType<'input>>, usize),
    (_, last, _): (usize, ProtoType<'input>, usize),
    (_, _, _): (usize, core::option::Option<&'input str>, usize),
) -> Vec<ProtoType<'input>>
{
    {
        let mut v = v;
        v.push(last);
        v
    }
}

#[allow(unused_variables)]
fn __action87<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action88<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action89<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Pattern<'input>>, usize),
    (_, last, _): (usize, Pattern<'input>, usize),
    (_, _, _): (usize, core::option::Option<&'input str>, usize),
) -> Vec<Pattern<'input>>
{
    {
        let mut v = v;
        v.push(last);
        v
    }
}

#[allow(unused_variables)]
fn __action90<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Pattern<'input>, usize),
) -> alloc::vec::Vec<Pattern<'input>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action91<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Pattern<'input>>, usize),
    (_, e, _): (usize, Pattern<'input>, usize),
) -> alloc::vec::Vec<Pattern<'input>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action92<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, (alloc::vec::Vec<Pattern<'input>>, Expr<'input>), usize),
) -> Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action93<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize),
    (_, last, _): (usize, core::option::Option<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize),
) -> Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>
{
    {
        let mut v = v;
        if let Some(e) = last {
            v.push(e);
        }
        v
    }
}

#[allow(unused_variables)]
fn __action94<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Vec<&'input str>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action95<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<&'input str>, usize),
    (_, last, _): (usize, core::option::Option<&'input str>, usize),
) -> Vec<&'input str>
{
    {
        let mut v = v;
        if let Some(e) = last {
            v.push(e);
        }
        v
    }
}

#[allow(unused_variables)]
fn __action96<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, (&'input str, ProtoType<'input>), usize),
) -> alloc::vec::Vec<(&'input str, ProtoType<'input>)>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action97<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<(&'input str, ProtoType<'input>)>, usize),
    (_, e, _): (usize, (&'input str, ProtoType<'input>), usize),
) -> alloc::vec::Vec<(&'input str, ProtoType<'input>)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action98<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Vec<&'input str>, usize),
) -> core::option::Option<Vec<&'input str>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action99<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Vec<&'input str>>
{
    None
}

#[allow(unused_variables)]
fn __action100<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Binding<'input>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action101<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Binding<'input>>, usize),
) -> alloc::vec::Vec<Binding<'input>>
{
    v
}

#[allow(unused_variables)]
fn __action102<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Binding<'input>, usize),
) -> alloc::vec::Vec<Binding<'input>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action103<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Binding<'input>>, usize),
    (_, e, _): (usize, Binding<'input>, usize),
) -> alloc::vec::Vec<Binding<'input>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action104<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> core::option::Option<&'input str>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action105<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<&'input str>
{
    None
}

#[allow(unused_variables)]
fn __action106<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> alloc::vec::Vec<&'input str>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action107<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<&'input str>, usize),
    (_, e, _): (usize, &'input str, usize),
) -> alloc::vec::Vec<&'input str>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action108<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action109<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, (alloc::vec::Vec<Pattern<'input>>, Expr<'input>), usize),
) -> core::option::Option<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action110<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>
{
    None
}

#[allow(unused_variables)]
fn __action111<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, (alloc::vec::Vec<Pattern<'input>>, Expr<'input>), usize),
) -> alloc::vec::Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action112<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize),
    (_, e, _): (usize, (alloc::vec::Vec<Pattern<'input>>, Expr<'input>), usize),
) -> alloc::vec::Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action113<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, (alloc::vec::Vec<Pattern<'input>>, Expr<'input>), usize),
    (_, _, _): (usize, &'input str, usize),
) -> (alloc::vec::Vec<Pattern<'input>>, Expr<'input>)
{
    __0
}

#[allow(unused_variables)]
fn __action114<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Pattern<'input>, usize),
) -> alloc::vec::Vec<Pattern<'input>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action115<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Pattern<'input>>, usize),
    (_, e, _): (usize, Pattern<'input>, usize),
) -> alloc::vec::Vec<Pattern<'input>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action116<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Pattern<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Pattern<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action117<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, ProtoType<'input>, usize),
) -> alloc::vec::Vec<ProtoType<'input>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action118<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<ProtoType<'input>>, usize),
    (_, e, _): (usize, ProtoType<'input>, usize),
) -> alloc::vec::Vec<ProtoType<'input>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action119<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, ProtoType<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ProtoType<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action120<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> alloc::vec::Vec<Expr<'input>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action121<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Expr<'input>>, usize),
    (_, e, _): (usize, Expr<'input>, usize),
) -> alloc::vec::Vec<Expr<'input>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action122<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> core::option::Option<&'input str>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action123<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<&'input str>
{
    None
}

#[allow(unused_variables)]
fn __action124<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> alloc::vec::Vec<Expr<'input>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action125<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Expr<'input>>, usize),
    (_, e, _): (usize, Expr<'input>, usize),
) -> alloc::vec::Vec<Expr<'input>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action126<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action127<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Expr<'input>>, usize),
    __1: (usize, Expr<'input>, usize),
    __2: (usize, &'input str, usize),
) -> Vec<Expr<'input>>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action122(
        errors,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        errors,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action128<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Expr<'input>>, usize),
    __1: (usize, Expr<'input>, usize),
) -> Vec<Expr<'input>>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action123(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        errors,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action129<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Pattern<'input>>, usize),
    __1: (usize, Pattern<'input>, usize),
    __2: (usize, &'input str, usize),
) -> Vec<Pattern<'input>>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action122(
        errors,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action89(
        errors,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action130<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Pattern<'input>>, usize),
    __1: (usize, Pattern<'input>, usize),
) -> Vec<Pattern<'input>>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action123(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action89(
        errors,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action131<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<ProtoType<'input>>, usize),
    __1: (usize, ProtoType<'input>, usize),
    __2: (usize, &'input str, usize),
) -> Vec<ProtoType<'input>>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action122(
        errors,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        errors,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action132<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<ProtoType<'input>>, usize),
    __1: (usize, ProtoType<'input>, usize),
) -> Vec<ProtoType<'input>>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action123(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        errors,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action133<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, (alloc::vec::Vec<Pattern<'input>>, Expr<'input>), usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action113(
        errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action111(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action134<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize),
    __1: (usize, (alloc::vec::Vec<Pattern<'input>>, Expr<'input>), usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action113(
        errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action112(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action135<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, Expr<'input>, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Expr<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action126(
        errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action124(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action136<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Expr<'input>>, usize),
    __1: (usize, Expr<'input>, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Expr<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action126(
        errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action125(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action137<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<&'input str>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action108(
        errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action106(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action138<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<&'input str>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<&'input str>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action108(
        errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action107(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action139<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, Pattern<'input>, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Pattern<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action116(
        errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action114(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action140<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Pattern<'input>>, usize),
    __1: (usize, Pattern<'input>, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Pattern<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action116(
        errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action115(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action141<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, ProtoType<'input>, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<ProtoType<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action119(
        errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action117(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action142<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<ProtoType<'input>>, usize),
    __1: (usize, ProtoType<'input>, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<ProtoType<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action119(
        errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action118(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action143<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize),
    __1: (usize, usize, usize),
) -> Expr<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        errors,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action144<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize),
    __1: (usize, usize, usize),
) -> Pattern<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        errors,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action145<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize),
    __1: (usize, usize, usize),
) -> ProtoType<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        errors,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action146<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize),
) -> Expr<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action87(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action143(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action147<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize),
) -> Pattern<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action87(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action144(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action148<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize),
) -> ProtoType<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action87(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action145(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action149<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize),
    __1: (usize, (alloc::vec::Vec<Pattern<'input>>, Expr<'input>), usize),
) -> Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action109(
        errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action93(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action150<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>, usize),
) -> Vec<(alloc::vec::Vec<Pattern<'input>>, Expr<'input>)>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action110(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action93(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action151<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, Expr<'input>, usize),
) -> Expr<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action72(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action152<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, Expr<'input>, usize),
    __1: (usize, alloc::vec::Vec<Expr<'input>>, usize),
) -> Expr<'input>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action73(
        errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action153<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<&'input str>, usize),
    __1: (usize, &'input str, usize),
) -> Vec<&'input str>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action104(
        errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action154<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<&'input str>, usize),
) -> Vec<&'input str>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action105(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action155<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Binding<'input>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action100(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action156<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Binding<'input>>, usize),
) -> Vec<Binding<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action101(
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action157<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<&'input str>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, alloc::vec::Vec<(&'input str, ProtoType<'input>)>, usize),
) -> Binding<'input>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action98(
        errors,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        errors,
        input,
        __0,
        __1,
        __temp0,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action158<
    'input,
    'err,
>(
    errors: &'err mut Vec<usize>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, alloc::vec::Vec<(&'input str, ProtoType<'input>)>, usize),
) -> Binding<'input>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action99(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        errors,
        input,
        __0,
        __1,
        __temp0,
        __2,
        __3,
    )
}

pub trait __ToTriple<'input, 'err, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, 'err, > __ToTriple<'input, 'err, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, 'err, > __ToTriple<'input, 'err, > for Result<(usize, Token<'input>, usize), &'static str> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
