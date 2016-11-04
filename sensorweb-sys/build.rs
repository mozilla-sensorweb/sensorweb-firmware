extern crate gcc;
fn main() {
  // See: http://alexcrichton.com/gcc-rs/gcc/index.html for docs on the gcc crate.
  gcc::Config::new()
    .compiler("arm-none-eabi-gcc")
    .define("gcc", None)
    .define("USE_FREERTOS", None)
    .flag("-std=c99")
    .include("../cc3200-rs/cc3200-sys")

  // Uncomment the following if you need to get access to the FreeRTOS headers
  /*
    .include("../cc3200-rs/cc3200-sys/sdk")
    .include("../cc3200-rs/cc3200-sys/sdk/inc")
    .include("../cc3200-rs/cc3200-sys/sdk/driverlib")
    .include("../cc3200-rs/cc3200-sys/sdk/example/common")
    .include("../cc3200-rs/cc3200-sys/sdk/oslib")
    .include("../cc3200-rs/cc3200-sys/sdk/third_party/FreeRTOS/source/include")
    .include("../cc3200-rs/cc3200-sys/sdk/third_party/FreeRTOS/source/portable/GCC/ARM_CM4")
   */
    .file("sensorweb.c")
    .compile("libsensorweb.a");

  println!("cargo:rustc-link-lib=sensorweb");
}
