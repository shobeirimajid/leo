/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    struct Foo {
        a: u128,
        b: u128,
    }
    
    transition main(
        i8_value: i8,
        i16_value: i16,
        i32_value: i32,
        i64_value: i64,
        u8_value: u8,
        u16_value: u16,
        u32_value: u32,
        u64_value: u64,
    ) -> field {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        // let group_value: group = (0, 1)group;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // BHP1024 commit tests
        let a: field = BHP1024::commit(addr_value, 1scalar);
        let b: field = BHP1024::commit(bool_value, 1scalar);
        let c: field = BHP1024::commit(field_value, 1scalar);
        // let d: field = BHP1024::commit(group_value, 1scalar);
        let e: field = BHP1024::commit(i8_value, 1scalar);
        let f: field = BHP1024::commit(i16_value, 1scalar);
        let g: field = BHP1024::commit(i32_value, 1scalar);
        let h: field = BHP1024::commit(i64_value, 1scalar);
        let i: field = BHP1024::commit(i128_value, 1scalar);
        let j: field = BHP1024::commit(u8_value, 1scalar);
        let k: field = BHP1024::commit(u16_value, 1scalar);
        let l: field = BHP1024::commit(u32_value, 1scalar);
        let m: field = BHP1024::commit(u64_value, 1scalar);
        let n: field = BHP1024::commit(u128_value, 1scalar);
        let o: field = BHP1024::commit(scalar_value, 1scalar);
        // let p: field = BHP1024::commit(string_value, 1scalar);
        let q: field = BHP1024::commit(Foo { a: 1u128, b: 2u128 }, 1scalar);
    
        return a + o;
    }}
