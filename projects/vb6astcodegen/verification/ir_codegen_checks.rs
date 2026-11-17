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
 let base=fixture();let mut query=Backend::new(base.clone(),NoHost);query.rust_enabled=false;let slot=query.call(0x0faab81f,&[0x20000500],0).unwrap() as u16;
 let mut bindings=Bindings::default();bindings.slots.insert(1,slot);
 let mut count=0;
 for ty in [3,5,6,8,10,11,12,13,15,16]{
  for kind in [6,7,8,9,10,11,12,13,0x16,0x17,0x18,0x19,0x1a,0x1d,0x1e,0x1f,0x20,0x21,0x22,0x23,0x24,0x25,0x26,0x27,0x28,0x29,0x2a,0x2b,0x36]{
   if (10..=13).contains(&ty)&&matches!(kind,6|0x1d..=0x23){continue}
   if kind==0x24&&ty!=15&&ty!=16{continue}
   if matches!(kind,12|13)&&ty!=15&&ty!=16{continue}
   for mode in [1,2,3,5,6]{
    for flags in [0,0x8000]{
     let mut v=base.clone();let mut ast=Ast::default();let l=ast.push(bound_leaf(&mut v,0x20000200,ty,7));let r=ast.push(bound_leaf(&mut v,0x20000300,ty,2));
     let result=if (0x25..=0x2b).contains(&kind)||kind==0x36{3}else{ty};
     let unary=kind<=13;let mut root=Node::operator(kind,result,l,if unary{None}else{Some(r)});root.flags=flags;root.symbol=Some(1);let id=ast.push(root);
     v.memory.write_u32(0x20000400,(result as u32)<<16|kind as u32).unwrap();v.memory.write_u32(0x20000404,flags).unwrap();v.memory.write_u32(0x20000410,0x20000200).unwrap();if !unary{v.memory.write_u32(0x20000414,0x20000300).unwrap();}v.memory.write_u32(0x20000418,0x20000500).unwrap();
     let mut oracle=Backend::new(v,NoHost);oracle.rust_enabled=false;let expected=oracle.expression(0x20000000,0x20000400,mode).unwrap_or_else(|e|panic!("reference kind={kind:X} ty={ty} mode={mode}: {e:?}"));let ret=oracle.vm.get_register("EAX").unwrap() as u32;
     let actual=emit_ast(&ast,id,mode as i32,&bindings).unwrap();assert_eq!(actual.bytes,expected,"kind={kind:X} ty={ty} mode={mode} flags={flags:X}");assert_eq!(actual.native_return,ret,"return kind={kind:X} ty={ty} mode={mode} flags={flags:X}");count+=1;
    }
   }
  }
 }
 println!("{count} expression byte-stream and native-return equivalence checks passed against the original lifted IR; not Microsoft runtime validation.");
}
