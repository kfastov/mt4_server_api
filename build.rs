extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_type("PluginInfo")
        .allowlist_type("PluginCfg")
        .allowlist_type("ConAccess")
        .allowlist_type("ConCommon")
        .allowlist_type("ConTime")
        .allowlist_type("ConBackup")
        .allowlist_type("ConFeeder")
        .allowlist_type("ConGroupSec")
        .allowlist_type("ConGroupMargin")
        .allowlist_type("ConGroup")
        .allowlist_type("ConHoliday")
        .allowlist_type("LiveInfoFile")
        .allowlist_type("ConLiveUpdate")
        .allowlist_type("ConManagerSec")
        .allowlist_type("ConManager")
        .allowlist_type("ConSession")
        .allowlist_type("ConSessions")
        .allowlist_type("ConSymbol")
        .allowlist_type("ConSymbolGroup")
        .allowlist_type("ConGatewayAccount")
        .allowlist_type("ConGatewayMarkup")
        .allowlist_type("ConGatewayRule")
        .allowlist_type("FeedTick")
        .allowlist_type("FeedData")
        .allowlist_type("FeedNews")
        .allowlist_type("LogInfo")
        .allowlist_type("LogRequest")
        .allowlist_type("PerformanceInfo")
        .allowlist_type("UserRecord")
        .allowlist_type("UserInfo")
        .allowlist_type("RateInfo")
        .allowlist_type("TickAPI")
        .allowlist_type("MailBoxHeader")
        .allowlist_type("TradeTransInfo")
        .allowlist_type("RequestInfo")
        .allowlist_type("TradeRecord")
        .allowlist_type("MarginLevel")
        .allowlist_type("OverNightData")
        .allowlist_type("DailyReport")
        .allowlist_type("StateReport")
        .allowlist_type("GroupState")
        .allowlist_type("CServerInterface")
        .allowlist_type("EnGatewayAccountFlags")
        .allowlist_type("EnGatewayRuleFlags")
        .allowlist_type("EnTickSources")
        .allowlist_type("constants")
        .allowlist_type("EnNewsFlags")
        .allowlist_type("EnLogType")
        .allowlist_type("EnReqFlags")
        .opaque_type("std::.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}