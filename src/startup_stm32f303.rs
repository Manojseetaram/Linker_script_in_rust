//Write startup code Part -1

//1. define the vector table for the mcu 

//2. define the reset handler 
 fn reset_handler(){
   //1 Copy the .data section to falsh and RAM 
   //2 Zero out the .bss section in to the RAM 
   //3 call main()
   crate::main()
   }
//3. define the exceptions and interuputs 