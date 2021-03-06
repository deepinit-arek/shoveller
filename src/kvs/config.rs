use toml;
use std::fs::File;

// If block or segment size > huge page, it must be a multiple of huge page.
pub const BLOCK_SHIFT:      usize = 16;
pub const BLOCK_SIZE:       usize = 1 << BLOCK_SHIFT;
pub const BLOCK_OFF_MASK:   usize = BLOCK_SIZE - 1;
pub const SEGMENT_SHIFT:    usize = 25;
pub const SEGMENT_SIZE:     usize = 1 << SEGMENT_SHIFT;
pub const BLOCKS_PER_SEG:   usize = SEGMENT_SIZE / BLOCK_SIZE;

/// Actual config struct created from the toml struct (with defaults).
#[derive(Copy,Clone,Debug)]
pub struct Config {
    comp_ratio: f64,
    comp_threads: usize,
    comp_reserve_segs: usize,
    comp_reserve_pct: f64,
    ht_virtual: usize,
    ht_per_socket: usize,
    log_alloc_extra_blks: usize,
    log_heads_per_socket: usize,
    log_head_select_cpuid: bool,
}

/// Used to read in the toml configuration file.
#[derive(Copy,Clone,Debug)]
pub struct ConfigToml {

    /// Ratio of available memory to total capacity, below which
    /// compaction threads will aggressively try to compress memory.
    /// If None, we default to 20%.
    comp_ratio: Option<f64>,

    /// Number of worker threads per instance. If None, then
    /// we allocate as many threads as half the number of cores
    /// as exist on one socket by default.
    comp_threads: Option<usize>,

    /// Specific number of segments devoted to reserve memory per socket.
    /// If None, reserve_pct is used.
    comp_reserve_segs: Option<usize>,

    /// Percent of overall capacity devoted to reserve memory.
    /// If None, then reserve_segs defines the quantity. If also None,
    /// we default to 2% of overall capacity.
    comp_reserve_pct: Option<f64>,

    /// Fixed size in virtual memory each hashtable occupies.
    /// Physical usage varies with occupancy.
    /// If None, we default to 1<<36 bytes.
    ht_virtual: Option<usize>,

    /// Number of hashtable partitions to use per socket. If None,
    /// we default to 8.
    ht_per_socket: Option<usize>,

    /// Vary new head allocations by at most this many blocks.
    /// If None, default to half segment size.
    log_alloc_extra_blks: Option<usize>,

    /// Number of log heads to use per socket.
    log_heads_per_socket: Option<usize>,

    /// Select head to append based on thread's current core ID (=true)
    /// or randomly (=false). If based on core ID, then
    /// log_heads_per_socket == cores per socket
    log_head_select_cpuid: bool,
}

impl Config {
    pub fn load(path: &str) -> Self {
        let file = File::open(path);
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        let config_toml: ConfigToml = toml::de::Deserializer::new(contents);
        // TODO verify that if based on cpuid, there are exactly as many
        // heads as there are cores.

        // do other sanity checking

        // generate the Config from the ConfigToml using defaults
        // when no values are specified
        
    }
}