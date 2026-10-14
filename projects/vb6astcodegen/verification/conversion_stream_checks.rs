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
 let base=fixture();let mut q=Backend::new(base.clone(),NoHost);q.rust_enabled=false;let slot=q.call(0x0faab81f,&[0x20000500],0).unwrap() as u16;let mut b=Bindings::default();b.slots.insert(1,slot);let mut count=0;let mut unresolved=0;
 for dest in [3,5,6,8,10,11,12,13,15,16,22]{for source in [3,5,6,8,10,11,12,13,15,16]{for flags in [0,0x100,0x200,0x300,0x400,0x500,0x700,0x4000,0x8000]{for mode in [1u32,2,3,5,6]{
  let mut v=base.clone();let mut ast=Ast::default();let child=ast.push(bound_leaf(&mut v,0x20000200,source,7));let root=ast.push(Node{kind:0xe,type_id:dest,flags,left:Some(child),symbol:Some(1),..Node::default()});
  v.memory.write_u32(0x20000400,(dest as u32)<<16|0xe).unwrap();v.memory.write_u32(0x20000404,flags).unwrap();v.memory.write_u32(0x20000410,0x20000200).unwrap();v.memory.write_u32(0x20000418,0x20000500).unwrap();
  let out=match emit_ast(&ast,root,mode as i32,&b){Ok(out)=>out,Err(_)=>{unresolved+=1;continue;}};
  let mut original=Backend::new(v,NoHost);original.rust_enabled=false;let expected=original.expression(0x20000000,0x20000400,mode).unwrap_or_else(|e|panic!("reference dest={dest} source={source} flags={flags:X} mode={mode}: {e:?}"));
  assert_eq!(out.bytes,expected,"dest={dest} source={source} flags={flags:X} mode={mode}");assert_eq!(out.native_return,original.vm.get_register("EAX").unwrap() as u32,"native return dest={dest} source={source} flags={flags:X} mode={mode}");count+=1;
 }}}}
 println!("{count} conversion expression stream and return checks passed; {unresolved} explicit errors in synthetic input grid. Original lifted IR, not Microsoft runtime.");
}
