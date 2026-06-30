//Write startup code Part -1

//1. define the vector table for the mcu 
 static VECTOR_TABLE  : [Option<unsafe fn()>; 99 ] = [
   Some(reset_handler), //You dont have to create this vector table manually insted use svd-vector-gen tool which will be explained shortly
   Some(nmi_handler),
   Some(hardfault_handler),
   Some(mem_manage_handler),
   Some(busfault_handler),
   Some(usagefault_handler),
   None,
   None,
   None,
   None
 ];
#[unsafe(no_mangle)]
 fn default_handler(){
  loop {
      
  }
 }
  #[unsafe(no_mangle)]
 fn hardfault_handler(){
 loop {
     
 }
 }
 #[unsafe(no_mangle)]
fn nmi_handler(){
  loop {
      
  }
}
//2. define the reset handler 
#[unsafe(no_mangle)]
 fn reset_handler(){
   //1 Copy the .data section to falsh and RAM 
   //2 Zero out the .bss section in to the RAM 
   //3 call main()
   crate::main()
   }
//3. define the exceptions and interuputs 