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

fn main(){let base=fixture();let mut total=0;
for kind in [0x51u16,0x52]{for category in 0..6u32{for mode in [1i32,5,6]{
let mut v=base.clone();let mut ast=Ast::default();let mut next=0x20000200u32;let mut addresses=Vec::new();
 let mut add=|ast:&mut Ast,v:&mut Vm,n:Node|{let p=next;next+=0x80;v.memory.write_u32(p,(n.type_id as u32)<<16|n.kind as u32).unwrap();v.memory.write_u32(p+4,n.flags).unwrap();if let Some(l)=n.left{v.memory.write_u32(p+16,addresses[l]).unwrap();}if let Some(r)=n.right{v.memory.write_u32(p+20,addresses[r]).unwrap();}if let Literal::Integer(i)=n.literal{v.memory.write_u32(p+16,i as u32).unwrap();}if let Some(h)=n.reference{v.memory.write_u32(p+16,h).unwrap();}let id=ast.push(n);addresses.push(p);id};

let a=add(&mut ast,&mut v,Node::integer(6,7));let b=add(&mut ast,&mut v,Node::integer(6,2));let args=add(&mut ast,&mut v,Node{kind:0x37,left:Some(a),right:Some(b),..Node::default()});
let refleaf=add(&mut ast,&mut v,Node{kind:0x6f,reference:Some(0x77),..Node::default()});let list=if category==2||category==3{add(&mut ast,&mut v,Node{kind:0x37,left:Some(refleaf),right:Some(args),..Node::default()})}else{args};let root=add(&mut ast,&mut v,Node{kind,flags:category<<8,right:Some(list),..Node::default()});drop(add);
let mut bindings=Bindings::default();bindings.references.insert(0x77,0x2468);let out=emit_ast(&ast,root,mode,&bindings).unwrap();
v.set_register("ECX",0x20000000).unwrap();v.push_u32(mode as u32).unwrap();v.push_u32(addresses[root]).unwrap();v.push_u32(0xffffff00).unwrap();v.pc=0x0fabf2d2;let mut host=NoHost;for _ in 0..3000{if v.pc==0xffffff00{break;}if v.pc==0x0fac2e29{v.return_stdcall(0x2468,4).unwrap();}else{v.step(&mut host).unwrap();}}assert_eq!(v.pc,0xffffff00);let len=v.memory.read(0x2000010c,2).unwrap() as usize;let expected:Vec<u8>=(0..len).map(|i|v.memory.read(0x20010000+i as u32,1).unwrap() as u8).collect();assert_eq!(out.bytes,expected,"kind={kind:X} category={category} mode={mode}");assert_eq!(out.native_return,v.get_register("EAX").unwrap() as u32);total+=1;
}}}println!("{total} Get/Put stream cases pass against original lifted IR, with explicit reference-pool helper result fixture.");}
