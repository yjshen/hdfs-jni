use std::env;

fn main() {
    match env::var("JAVA_HOME") {
        Ok(val) => {
            println!("cargo:rustc-link-search=all={}/jre/lib/amd64/server", val);
        }
        Err(e) => {
            panic!("HADOOP_HOME shell environment must be set: {}", e);
        }
    }

    // for libhdfs.a
    match env::var("HADOOP_HOME") {
        Ok(val) => {
            println!("cargo:rustc-link-search=all={}/lib/native", val);
        }
        Err(e) => {
            panic!("HADOOP_HOME shell environment must be set: {}", e);
        }
    }

    // for jvm.h and linking to jni libraries
    let mut minidfs_config = cc::Build::new();
    minidfs_config
        .file("src/libhdfs/native_mini_dfs.c")
        .include("src/libhdfs");

    match env::var("JAVA_HOME") {
        Ok(val) => {
            minidfs_config.include(format!("{}/include/", val));
            if cfg!(target_os = "linux") {
                minidfs_config.include(format!("{}/include/linux", val));
            } else if cfg!(target_os = "macos") {
                minidfs_config.include(format!("{}/include/darwin", val));
            }
            // TODO - to be changed to consider a dependent platform.
        }
        Err(e) => {
            panic!("JAVA_HOME shell environment must be set: {}", e);
        }
    }

    minidfs_config.compile("minidfs");
}
