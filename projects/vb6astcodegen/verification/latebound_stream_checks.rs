#[path="../../research/sources/backend/vb_ast_backend.rs"]
mod original;
use original::{Backend,residual::{Vm,NoHost}};
use vba6_ast_codegen::{Ast,Bindings,Node,Literal,emit_ast};
fn fixture()->Vm{
 let mut v=Vm::new().unwrap();v.memory.map(0x20000000,vec![0;131072]).unwrap();v.memory.map(0x30000000,vec![0;65536]).unwrap();v.set_register("ESP",0x3000f000).unwrap();
 v.memory.write_u32(0x20000000,0x20000100).unwrap();v.memory.write_u32(0x20000100,0x20010000).unwrap();v.memory.write(0x2000010e,2,65535).unwrap();
 v.memory.write_u32(0x20000500,4).unwrap();v.memory.write_u32(0x20000508,0x20000540).unwrap();v.memory.write_u32(0x20000540,0x20000560).unwrap();v.memory.write(0x20000568,2,0x84).unwrap();v
}
fn bound_leaf(v:&mut Vm,p:u32,ty:u16,value:i32)->Node{
 let mut n=Node::integer(ty,value);
 if (10..=12).contains(&ty){n.kind=3;n.literal=Literal::Raw64((value as f64).to_le_bytes());}
 if ty==13{n.kind=2;n.literal=Literal::Raw64((value as i64*10000).to_le_bytes());}
 if ty==15{n.kind=5;n.symbol=Some(1);let mut bytes=[0;8];bytes[..4].copy_from_slice(&value.to_le_bytes());n.literal=Literal::Variant{tag:3,bytes,string_reference:None};v.memory.write_u32(p+16,p+64).unwrap();v.memory.write(p+64,2,3).unwrap();v.memory.write(p+72,4,value as u32 as u128).unwrap();v.memory.write_u32(p+24,0x20000500).unwrap();}
 else{match n.literal{Literal::Integer(i)=>v.memory.write_u32(p+16,i as u32).unwrap(),Literal::Raw64(b)=>v.memory.write(p+16,8,u64::from_le_bytes(b) as u128).unwrap(),_=>unreachable!()}}
 v.memory.write_u32(p,(ty as u32)<<16|n.kind as u32).unwrap();n
}
fn main(){
 let mut base=fixture();base.memory.map(0x60000000,vec![0;4096]).unwrap();base.memory.write_u32(0x60000000,u32::MAX).unwrap();base.set_register("FS_OFFSET",0x60000000).unwrap();
 base.memory.write_u32(0x20000004,0x20000800).unwrap();base.memory.write_u32(0x20000a54,0x200009ec).unwrap();base.memory.write_u32(0x20000a00,0x20000b00).unwrap();base.memory.write_u32(0x20000a08,0x20000c00).unwrap();base.memory.write_u32(0x20000a0c,32).unwrap();base.memory.write_u32(0x20000c18,8).unwrap();base.memory.write(0x20000b2a,2,32).unwrap();base.memory.write_u32(0x20000b30,0x20000e00).unwrap();
 let mut query=Backend::new(base.clone(),NoHost);query.rust_enabled=false;let slot=query.call(0x0faab81f,&[0x20000500],0).unwrap() as u16;let index=query.call(0x0fac2e29,&[!8],0x20000000).unwrap() as u16;
 let mut bindings=Bindings::default();bindings.slots.insert(1,slot);bindings.references.insert(!8,index);let mut cases=0;
 for kind in 0x6b..=0x6e{for comb in 0u32..64 {let flags=comb<<10;for variant in [false,true]{for count in [0u16,1]{for mode in [1u32,2,3]{
  let mut v=base.clone();let mut ast=Ast::default();let target=ast.push(bound_leaf(&mut v,0x20000200,if variant{15}else{8},7));let arg=ast.push(bound_leaf(&mut v,0x20000300,8,3));
  let mut n=Node::operator(kind,8,target,Some(arg));n.flags=flags;n.symbol=Some(1);n.count=count;n.layout_count=1;n.member=Some(if flags&0x8000!=0{vba6_ast_codegen::Member::Direct(0xdeadbeef)}else{vba6_ast_codegen::Member::Reference(!8)});let root=ast.push(n);
  v.memory.write_u32(0x20000400,kind as u32|(8<<16)).unwrap();v.memory.write_u32(0x20000404,flags).unwrap();v.memory.write_u32(0x20000410,0x20000200).unwrap();v.memory.write_u32(0x20000414,0x20000300).unwrap();v.memory.write_u32(0x20000418,0x20000500).unwrap();v.memory.write_u32(0x2000041c,if flags&0x8000!=0{0xdeadbeef}else{!8}).unwrap();v.memory.write(0x20000420,2,count as u128).unwrap();v.memory.write(0x20000422,2,1).unwrap();
  let mut reference=Backend::new(v,NoHost);reference.rust_enabled=false;let expected=reference.expression(0x20000000,0x20000400,mode).unwrap();
  let out=emit_ast(&ast,root,mode as i32,&bindings).unwrap();assert_eq!(out.bytes,expected,"kind={kind:X} flags={flags:X} variant={variant} count={count} mode={mode}");assert_eq!(out.native_return,reference.vm.get_register("EAX").unwrap() as u32);cases+=1;
 }}}}}
 println!("{cases} late-bound complete byte-stream equivalence cases pass against original lifted IR; no Microsoft runtime validation.");
}
