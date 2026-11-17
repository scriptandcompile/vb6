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
fn run(vm:&mut Vm,kind:u32,class:u32,flags:u32,df:u8,foreign:bool)->(Vec<u8>,u32){
 vm.set_register("ECX",0x20000000).unwrap();vm.push_u32(1).unwrap();vm.push_u32(0x20000400).unwrap();vm.push_u32(0xffffff00).unwrap();vm.pc=0x0fabf2d2;
 let mut host=NoHost;
 for _ in 0..10000 {
  if vm.pc==0xffffff00 {let len=vm.memory.read(0x2000010c,2).unwrap() as usize;let b=(0..len).map(|i|vm.memory.read(0x20010000+i as u32,1).unwrap() as u8).collect();return(b,vm.get_register("EAX").unwrap() as u32);}
  match vm.pc {
   0x0fab7f18=>vm.return_stdcall(kind,4).unwrap(),
   0x0fab7f4d=>vm.return_stdcall(class,4).unwrap(),
   0x0faab81f=>vm.return_stdcall(0x84,4).unwrap(),
   0x0fac3945=>{let p=vm.stack_argument(1).unwrap();vm.memory.write(p,2,0x3344).unwrap();vm.return_stdcall(0,8).unwrap();},
   0x0fac2e29=>vm.return_stdcall(0x5678,4).unwrap(),
   0x0fab4e58=>vm.return_stdcall(0x20005000,12).unwrap(),
   0x0faeba30=>vm.return_stdcall(0xabcd1234,8).unwrap(),
   0x0fab5d51=>{let p=vm.stack_argument(1).unwrap();vm.memory.write_u32(p,if foreign{0x20004200}else{0x20004000}).unwrap();vm.return_stdcall(0,8).unwrap();},
   0x50008000=>vm.return_stdcall(1,4).unwrap(),
   _=>vm.step(&mut host).unwrap_or_else(|e|panic!("PC={:X} kind={kind} class={class} flags={flags:X} df={df:X}: {e:?}",vm.pc)),
  }
 }
 panic!("step limit");
}
fn main(){
 let mut base=fixture();base.memory.write_u32(0x20000004,0x20000800).unwrap();base.memory.write_u32(0x20000824,0x20004000).unwrap();for p in [0x20004000,0x20004200]{base.memory.write_u32(p,0x20004100).unwrap();}base.memory.write_u32(0x20004108,0x50008000).unwrap();let mut total=0;
 for kind in [1u32,4]{for class in [0u32,1,2]{for flags in [0u32,0x4000,0x8000,0x0800,0x2000,0x2800,0xa000,0x1000,0x0200]{for df in [0u8,8,16,24]{for ty in [2u16,6,10,15,18]{for foreign in [false,true]{
  let mut vm=base.clone();let descriptor=if class==1&&kind==4{0x0faab7e0}else{0x0faab770+(vm.memory.read(0x0faab7d0+kind,1).unwrap() as u32&3)*0x1e};vm.memory.write(descriptor+0x1d,1,df as u128).unwrap();
  let mut ast=Ast::default();let target=ast.push(bound_leaf(&mut vm,0x20000200,8,7));let args=ast.push(bound_leaf(&mut vm,0x20000300,6,3));let root=ast.push(Node{kind:0x61,type_id:ty,flags,left:Some(target),right:Some(args),symbol:Some(1),count:0x8899,..Node::default()});
  vm.memory.write_u32(0x20000400,(ty as u32)<<16|0x61).unwrap();vm.memory.write_u32(0x20000404,flags).unwrap();vm.memory.write_u32(0x20000410,0x20000600).unwrap();vm.memory.write_u32(0x20000614,0x20000500).unwrap();vm.memory.write_u32(0x20000414,0x20000300).unwrap();vm.memory.write_u32(0x20000418,0x20000200).unwrap();vm.memory.write_u32(0x2000041c,0x8899).unwrap();vm.memory.write_u32(0x20000420,0x20000500).unwrap();vm.memory.write_u32(0x20000424,0x20000700).unwrap();
  let mut b=Bindings::default();b.slots.insert(1,0x84);
  b.member_metadata.insert(root,vba6_ast_codegen::member_calls::ResolvedMemberMetadata{symbol_kind:Some(kind),class:Some(class),descriptor_flags:Some(df),foreign_context:Some(foreign),result_type_index:None,symbol_slot:None,indirect_descriptor:Some(0xabcd1234),target_bytes:None,argument_bytes:None,call_descriptor:Some(0x3344),result_descriptor:Some(0x5678)});
  let output=emit_ast(&ast,root,1,&b).unwrap();let(reference,ret)=run(&mut vm,kind,class,flags,df,foreign);assert_eq!(output.bytes,reference,"member kind={kind} class={class} flags={flags:X} df={df:X} ty={ty} foreign={foreign}");assert_eq!(output.native_return,ret);total+=1;
 }}}}}}
 println!("{total} member complete-stream checks pass with explicit helper-result fixtures against original lifted IR; helper implementations and Microsoft runtime not validated.");
}
