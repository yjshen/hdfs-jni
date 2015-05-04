use libc::{c_char, c_int, c_uchar, int32_t, int64_t, uint16_t, c_void};

// Opaque Pointer of hdfsFS
pub enum HdfsFS {}

// Opaque Pointer of hdfsFile
pub enum HdfsFile {}

// Opaque Pointer of hdfsBuilder
pub enum HdfsBuilder {}

#[repr(C)]
#[allow(non_snake_case)]
pub struct HdfsReadStatistics {
  pub totalBytesRead : u64,
  pub totalLocalBytesRead : u64,
  pub totalShortCircuitBytesRead : u64,
  pub totalZeroCopyBytesRead : u64
}

#[link(name="hdfs")]
extern "C" {
  
  /// Determine if a file is open for read.
  ///
  /// Return 1 if the file is open for read; 0 otherwise
  pub fn hdfsFileIsOpenForRead(fs: *mut HdfsFile) -> c_int;
  
  /// Determine if a file is open for write.
  /// 
  /// Return 1 if the file is open for write; 0 otherwise.
  pub fn hdfsFileIsOpenForWrite(file: *mut HdfsFile) -> c_int;
  
  /// Get read statistics about a file.  This is only applicable to files 
  /// opened for reading.
  ///
  /// # Return Value
  /// 
  /// * 0 if the statistics were successfully returned,
  /// * -1 otherwise.  On a failure, please check errno against
  /// * ENOTSUP. webhdfs, LocalFilesystem, and so forth may 
  /// not support read statistics.
  pub fn hdfsFileGetReadStatistics(file: *mut HdfsFile, 
                   stats: &mut *mut HdfsReadStatistics) -> c_int;
  
  /// HDFS read statistics for a file,
  /// 
  /// Return the number of remote bytes read.
  pub fn hdfsReadStatisticsGetRemoteBytesRead(
    stats: *const HdfsReadStatistics) -> int64_t;
  
  /// Free some HDFS read statistics.
  pub fn hdfsFileFreeReadStatistics(stats: *mut HdfsReadStatistics);
  
  /// hdfsConnectAsUser - Connect to a hdfs file system as a specific user.
  ///
  /// Returns a handle to the filesystem or NULL on error.  
  pub fn hdfsConnectAsUser(host: *const c_char, 
                       uint16_t: u16, user: 
                       *const c_char) -> *mut HdfsFS;
  
  /// hdfsConnect - Connect to a hdfs file system.
  ///
  /// Returns a handle to the filesystem or NULL on error.  
  pub fn hdfsConnect(host: *const c_char, uint16_t: u16) -> *mut HdfsFS;
  
  /// hdfsConnect - Connect to an hdfs file system.
  /// 
  /// Forces a new instance to be created
  ///
  /// Returns a handle to the filesystem or NULL on error.
  pub fn hdfsConnectAsUserNewInstance(host: *const c_char, 
                    uint16_t: u16,
                    user: *const c_char) -> *mut HdfsFS;
  
  pub fn hdfsConnectNewInstance(host: *const c_char, 
                            uint16_t: u16) -> *mut HdfsFS;
  
  pub fn hdfsNewBuilder() -> *mut HdfsBuilder;
  
  pub fn hdfsFreeBuilder(bld: *mut HdfsBuilder);
  
  pub fn hdfsBuilderSetNameNode(bld: *mut HdfsBuilder, host: *const c_char);
  
  pub fn hdfsBuilderSetNameNodePort(bld: *mut HdfsBuilder, port : uint16_t);
  
  pub fn hdfsBuilderSetUserName(bld: *mut HdfsBuilder, userName: *const c_char);
  
  pub fn hdfsBuilderSetKerbTicketCachePath(bld: *mut HdfsBuilder, 
                       kerbTicketCachePath: *const c_char);
  
  pub fn hdfsBuilderConfSetStr(bld: *mut HdfsBuilder, 
                           key: *const c_char , value: *const c_char) -> c_int;
  
//  fn hdfsConfGetStr(value : *const c_char, **c_char val) -> c_int

  pub fn hdfsConfGetInt(key: *const c_char, val: *mut int32_t) -> c_int;
  
  pub fn hdfsConfStrFree(val: *const c_char);
  
  /// 
  /// Try to connect a specific HDFS namenode.
  ///
  /// 
  pub fn hdfsBuilderConnect(bld : *mut HdfsBuilder) -> *mut HdfsFS;
  
  pub fn hdfsDisconnect(fs: *mut HdfsFS) -> c_int;
  
  pub fn hdfsExists(fs: *mut HdfsFS, path: *const c_char) -> c_int;
}

pub enum NativeMiniDfsCluster {}

#[repr(C)]
#[allow(non_snake_case)]
pub struct MiniDfsConf {
  do_format: c_uchar,
  webhdfs_enabled: c_uchar,
  namenode_http_port: c_int,
  short_circuit_enabled: c_uchar
}

impl MiniDfsConf {
  pub fn new() -> MiniDfsConf {
    MiniDfsConf {
      do_format: 1,
      webhdfs_enabled: 0,
      namenode_http_port: 0,
      short_circuit_enabled: 0
    }
  }

  pub fn set_do_format(&mut self, on: bool) -> &mut MiniDfsConf {
    self.do_format = if on { 1 } else { 0 };
    self
  }

  pub fn do_format(&self) -> bool {
    if self.do_format != 0 { true } else { false }
  }

  pub fn set_web_hdfs(&mut self, enable: bool) -> &mut MiniDfsConf {
    self.webhdfs_enabled = if enable { 1 } else { 0 };
    self
  }

  pub fn web_hdfs_enabled(&self) -> bool {
    if self.webhdfs_enabled != 0 { true } else { false } 
  }

  pub fn set_http_port(&mut self, port: i32) -> &mut MiniDfsConf {
    self.namenode_http_port = port as c_int;
    self
  }

  pub fn http_port(&self) -> i32 {
    self.namenode_http_port
  }

  pub fn set_short_circuit(&mut self, enable: bool) -> &mut MiniDfsConf {
    self.short_circuit_enabled = if enable { 1 } else { 0 };
    self
  }

  pub fn short_circuit_enabled(&self) -> bool {
    if self.short_circuit_enabled != 0 { true } else { false } 
  }
}

#[link(name="hdfs")]
extern "C" {
  pub fn nmdCreate(conf: *const MiniDfsConf) -> *mut NativeMiniDfsCluster;

  pub fn nmdWaitClusterUp(cl: *mut NativeMiniDfsCluster) -> c_int;

  pub fn nmdShutdown(cl: *mut NativeMiniDfsCluster) -> c_int;

  pub fn nmdFree(cl: *mut NativeMiniDfsCluster) -> c_void;

  pub fn nmdGetNameNodePort(cl: *const NativeMiniDfsCluster) -> c_int;

  pub fn nmdGetNameNodeHttpAddress(cl: *const NativeMiniDfsCluster,
                               port: *mut c_int, hostName: *mut *mut c_char) -> c_int;

  pub fn nmdConfigureHdfsBuilder(cl: *mut NativeMiniDfsCluster, bld: *mut HdfsBuilder) -> c_int;
}