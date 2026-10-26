#[path="../../research/sources/backend/tip_compile_module.rs"] mod original;
fn main() {
 let vm=original::Vm::new().unwrap();
 for (id, expected) in [(9u32,"__vbaAryConstruct2"),(12,"__vbaAryDestruct"),(0x9b,"__vbaFixstrConstruct"),(0x140,"__vbaGosubFree")] {
  let record=0x0faf0df8+id*20;
  let pointer=vm.memory.read(record,4).unwrap() as u32;
  let mut bytes=Vec::new();
  for i in 0..128 {let b=vm.memory.read(pointer+i,1).unwrap() as u8;if b==0 {break;}bytes.push(b);}
  let name=String::from_utf8(bytes).unwrap();assert_eq!(name,expected);assert_eq!(vm.memory.read(record+16,4).unwrap(),2);
  println!("builtin={id:X} record={record:08X} pointer={pointer:08X} name={name} result_type_template={}",vm.memory.read(record+16,4).unwrap());
 }
}
