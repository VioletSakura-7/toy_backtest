use std::{
    ops::Deref,
    sync::{
        atomic::{
            AtomicBool,
            AtomicU64,
            Ordering,
        },
        Arc,
        OnceLock,
    },
    thread,
    time::{
        Duration,
        SystemTime,
        UNIX_EPOCH,
    },
};

static GLOBAL_TIME: OnceLock<Arc<AtomicTime>> = OnceLock::new();

pub struct AtomicTime
{
    timestamp_ns: AtomicU64,
    realtime: AtomicBool,
}
impl AtomicTime
{
    pub fn new() -> Arc<Self>
    {
        GLOBAL_TIME
            .get_or_init(|| {
                let clock = Arc::new(AtomicTime::default());
                let clock_clone = clock.clone();
                thread::spawn(move || {
                    clock_clone.run();
                });
                clock
            })
            .clone()
    }

    pub fn now(&self) -> u64
    {
        self.load(Ordering::Relaxed)
    }

    pub fn stop(&self)
    {
        self.realtime.store(false, Ordering::Relaxed);
    }

    pub fn set_time(&self, timestamp_ns: u64)
    {
        self.store(timestamp_ns, Ordering::Relaxed);
    }

    pub fn is_realtime(&self) -> bool
    {
        self.realtime.load(Ordering::Relaxed)
    }

    pub fn increment(&self, nanoseconds: u64)
    {
        self.fetch_add(nanoseconds, Ordering::Relaxed);
    }

    fn run(&self)
    {
        while self.is_realtime() {
            self.increment(1_000_000);
        }
        thread::sleep(Duration::from_millis(1));
    }
}

impl Deref for AtomicTime
{
    type Target = AtomicU64;

    fn deref(&self) -> &Self::Target
    {
        &self.timestamp_ns
    }
}

impl Default for AtomicTime
{
    /// Creates a new default [`AtomicTime`] instance.
    fn default() -> Self
    {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self {
            timestamp_ns: AtomicU64::new(now),
            realtime: AtomicBool::new(true),
        }
    }
}

impl AtomicTime {}
