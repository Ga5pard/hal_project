# hal_project
HAL Project done during my 4th year in my engineering school.

[CORRECTION GPIO] (Don't hesitate to remove this part)
Consider subdividing your project into separate modules. 
You can only use the I/O registers of port B here (and not the C port for example).
It would be nice to have something to prevent modifying the register in an incoherent way. For example, if I do ``` config_pin(40, true);```, it won't bug during the compilation, but it may generate a problem on your hardware.

[CORRECTION USART] (Don't hesitate to remove this part)
You could try implementing the different USART mode (asynchrone double speed for example).
You didn't implement USART for your second target.