
This is a very simple Rust program for the [TI MSP432 Launchpad](http://www.ti.com/tool/MSP-EXP432P401R) 
which demonstrates use of the [msp432p401r crate](https://crates.io/crates/msp432p401r). 

Embedded systems code written in Rust can fail to work depending on changes in the
version of the nightly compiler used. You can check [this article](http://pramode.in/2018/01/31/ti-launchpad-with-rust-new-io/)
which explains how to use the [tm4c123x crate](https://crates.io/crates/tm4c123x) for programming the 
Texas Instruments Tiva/Stellaris launchpads - the same toolchain setup should be used for
building the code in this repository also. You will have to use [UniFlash](http://processors.wiki.ti.com/index.php/Category:CCS_UniFlash)
for flash programming (rather than lm4flash).

Once the toolchain is installed, you can build the code by running:

```
make release
```

The binary is copied to the *upload* folder and its name is *msp432.release.bin*. You can
use UniFlash to flash this code and you will see the RGB LED on the MSP432 Launchpad lighting
up!


