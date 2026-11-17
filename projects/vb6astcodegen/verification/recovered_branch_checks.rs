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

fn run(mut v:Vm,root:u32,mode:i32,checks:(bool,bool))->(Vec<u8>,u32,u32){
 v.set_register("ECX",0x20000000).unwrap();v.push_u32(mode as u32).unwrap();v.push_u32(root).unwrap();v.push_u32(0xffffff00).unwrap();v.pc=0x0fabf2d2;let mut h=NoHost;
 for _ in 0..10000{if v.pc==0xffffff00{let len=v.memory.read(0x2000010c,2).unwrap() as usize;let bytes=(0..len).map(|i|v.memory.read(0x20010000+i as u32,1).unwrap() as u8).collect();return(bytes,v.get_register("EAX").unwrap() as u32,v.memory.read(root,4).unwrap() as u32);}
 match v.pc{
 0x0faab81f=>v.return_stdcall(0x84,4).unwrap(),
 0x0fab5494=>v.return_stdcall(checks.0 as u32,4).unwrap(),
 0x0fab54ae=>v.return_stdcall(checks.1 as u32,4).unwrap(),
 0x0fac2e29=>v.return_stdcall(0x2468,4).unwrap(),
 0x0fab8628=>v.return_stdcall(0x4444,4).unwrap(),
 0x0faab2c2=>{let p=v.stack_argument(0).unwrap();let old=v.memory.read(p,4).unwrap() as u32;v.memory.write_u32(p,(old&0xffff)|0x20000).unwrap();v.return_stdcall(0,4).unwrap();},
 0x0fac0f35|0x0fac0fa8=>{let which=v.pc;let p=v.stack_argument(0).unwrap();let out=v.stack_argument(if which==0x0fac0f35{1}else{2}).unwrap();let flags=v.memory.read(p+4,4).unwrap() as u32;v.memory.write_u32(p+4,flags&!0x1000).unwrap();for i in 0..4{v.memory.write_u32(out+i*4,0).unwrap();}v.memory.write_u32(out,1).unwrap();v.memory.write(out+10,2,0x84).unwrap();v.return_stdcall(0,if which==0x0fac0f35{8}else{12}).unwrap();},
 _=>v.step(&mut h).unwrap_or_else(|e|panic!("root={root:X} PC={:X}: {e:?}",v.pc))
 }}panic!("step limit");
}
fn add(v:&mut Vm,ast:&mut Ast,addresses:&mut Vec<u32>,n:Node)->usize{let p=0x20000200+addresses.len() as u32*0x80;v.memory.write_u32(p,(n.type_id as u32)<<16|n.kind as u32).unwrap();v.memory.write_u32(p+4,n.flags).unwrap();if let Some(c)=n.left{v.memory.write_u32(p+16,addresses[c]).unwrap();}if let Some(c)=n.right{v.memory.write_u32(p+20,addresses[c]).unwrap();}if n.symbol.is_some(){v.memory.write_u32(p+24,0x20000500).unwrap();}if n.reference.is_some(){v.memory.write_u32(p+20,0x20000500).unwrap();}if let Some(_)=n.member{v.memory.write_u32(p+28,0x20000500).unwrap();}v.memory.write(p+32,2,n.count as u128).unwrap();let id=ast.push(n);addresses.push(p);id}
fn leaf(v:&mut Vm,ast:&mut Ast,addresses:&mut Vec<u32>,ty:u16,value:i32)->usize{let p=0x20000200+addresses.len() as u32*0x80;let n=bound_leaf(v,p,ty,value);let id=ast.push(n);addresses.push(p);id}
fn main(){let base=fixture();let mut total=0;let mut b=Bindings::default();b.slots.insert(1,0x84);b.references.insert(1,0x2468);
 for ty in [2u16,6,10,15,22]{for child_ty in [6u16,10,15,22]{for mode in [1i32,2,3,5,6]{for checks in [(false,false),(true,false),(false,true),(true,true)]{
 let mut v=base.clone();let mut ast=Ast::default();let mut addresses=vec![];let child=leaf(&mut v,&mut ast,&mut addresses,child_ty,7);let root=add(&mut v,&mut ast,&mut addresses,Node{kind:0x68,type_id:ty,left:Some(child),reference:Some(1),..Node::default()});b.property_checks.insert(root,checks);let out=emit_ast(&ast,root,mode,&b).unwrap();let (expected,ret,_)=run(v,addresses[root],mode,checks);assert_eq!(out.bytes,expected,"property ty={ty} child={child_ty} mode={mode} checks={checks:?}");assert_eq!(out.native_return,ret);total+=1;
 }}}}println!("{total} property stream cases pass with explicit helper-result fixtures.");
 let mut indexed=0;for flags in [0x2000u32,0x3000,0x4000,0x5000,0x6000,0x7000]{for category in 0..4u32{for indirect in [false,true]{for ty in [6u16,15,16]{
 let mut v=base.clone();let mut ast=Ast::default();let mut addresses=vec![];let basearg=leaf(&mut v,&mut ast,&mut addresses,6,7);let subscript=leaf(&mut v,&mut ast,&mut addresses,6,2);let ix=add(&mut v,&mut ast,&mut addresses,Node{kind:0x6a,type_id:ty,flags:category<<8|if indirect{0x8000}else{0},left:Some(basearg),right:Some(subscript),symbol:Some(1),member:Some(vba6_ast_codegen::Member::Reference(1)),count:1,..Node::default()});let lhs=add(&mut v,&mut ast,&mut addresses,Node{kind:0x11,type_id:ty,left:Some(ix),..Node::default()});let rhs=leaf(&mut v,&mut ast,&mut addresses,6,9);let root=add(&mut v,&mut ast,&mut addresses,Node{kind:0x2c,type_id:2,flags,left:Some(lhs),right:Some(rhs),..Node::default()});let out=emit_ast(&ast,root,1,&b).unwrap();let(expected,ret,_)=run(v,addresses[root],1,(false,false));assert_eq!(out.bytes,expected,"indexed flags={flags:X} category={category} indirect={indirect} ty={ty}");assert_eq!(out.native_return,ret);indexed+=1;
 }}}}println!("{indexed} indexed-assignment stream cases pass with explicit slot fixtures.");
 let mut retag=0;for ty in [6u16,10,15,16]{for flags in [0u32,0x1000,0x2000,0x4000,0x8000]{if ty!=15&&ty!=16&&flags&0x6000!=0{continue;}for mode in [1i32,5,6]{
 let mut v=base.clone();let mut ast=Ast::default();let mut addresses=vec![];let lhs=add(&mut v,&mut ast,&mut addresses,Node{kind:0x60,type_id:ty,reference:Some(1),..Node::default()});let rhs=leaf(&mut v,&mut ast,&mut addresses,6,9);let root=add(&mut v,&mut ast,&mut addresses,Node{kind:0x2d,type_id:ty,flags,left:Some(lhs),right:Some(rhs),..Node::default()});b.prepared_addresses.insert(lhs,vba6_ast_codegen::addressing::AddressDescriptor{kind:1,word_at_10:0x84,flags_at_4:0,word_at_6:0,word_at_8:0,word_at_12:0});let out=emit_ast(&ast,root,mode,&b).unwrap();let(expected,ret,mutated)=run(v,addresses[root],mode,(false,false));assert_eq!(out.bytes,expected,"retag flags={flags:X} ty={ty} mode={mode}");assert_eq!(out.native_return,ret);if flags!=0||ty==15{assert_eq!(mutated,(ty as u32)<<16|0x2c);}assert_eq!(ast.nodes[root].kind,0x2d);retag+=1;
 }}}println!("{retag} assignment-retag stream cases pass with explicit address-preparation/slot/void-type fixtures.");
 let mut aliases=0;for ty in [6u16,10,15,16]{for flags in [0u32,0x1000,0x8000]{for mode in [1i32,5,6]{
 let mut v=base.clone();let mut ast=Ast::default();let mut addresses=vec![];let lhs=add(&mut v,&mut ast,&mut addresses,Node{kind:0x60,type_id:ty,reference:Some(1),..Node::default()});let rhs=leaf(&mut v,&mut ast,&mut addresses,10,9);let assign=add(&mut v,&mut ast,&mut addresses,Node{kind:0x2d,type_id:ty,flags,left:Some(lhs),right:Some(rhs),..Node::default()});let root=add(&mut v,&mut ast,&mut addresses,Node{kind:0x2f,left:Some(assign),right:Some(assign),..Node::default()});b.prepared_addresses.insert(lhs,vba6_ast_codegen::addressing::AddressDescriptor{kind:1,word_at_10:0x84,flags_at_4:0,word_at_6:0,word_at_8:0,word_at_12:0});let out=emit_ast(&ast,root,mode,&b).unwrap();let(expected,ret,_)=run(v,addresses[root],mode,(false,false));assert_eq!(out.bytes,expected,"shared assignment flags={flags:X} ty={ty} mode={mode}");assert_eq!(out.native_return,ret);assert_eq!(ast.nodes[assign].kind,0x2d);aliases+=1;
 }}}println!("{aliases} shared-assignment stream cases pass, including persistent native tag mutation and floating RHS.");

}
