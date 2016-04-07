use time::SteadyTime;

pub struct Stats {
    announces: u64,
    torrents: u64,
    peers: u64,
    start_time: SteadyTime,
    clear_time: SteadyTime,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            announces: 0,
            torrents: 0,
            peers: 0,
            start_time: SteadyTime::now(),
            clear_time: SteadyTime::now(),
        }
    }
}
