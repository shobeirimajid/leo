/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping foo: (u32, u32) => u32;
    
    mapping floo: baz => u8;
    
    mapping bar: foo => baz;
    
}
