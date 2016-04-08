use time::SteadyTime;

#[derive(Debug)]
pub struct Stats {
    pub announces: u64,
    pub scrapes: u64,
    torrents: u64,
    peers: u64,
    start_time: SteadyTime,
    clear_time: SteadyTime,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            announces: 0,
            scrapes: 0,
            torrents: 0,
            peers: 0,
            start_time: SteadyTime::now(),
            clear_time: SteadyTime::now(),
        }
    }

    pub fn update(&mut self) {
        self.clear_time = SteadyTime::now();
        self.announces = 0;
        self.scrapes = 0;
    }
}

#[derive(Debug)]
pub struct StatsResponse {
    pub announce_rate: u64,
    pub scrape_rate: u64,
    pub torrents: u64,
    pub peers: u64,
    pub uptime: u64,
}

impl StatsResponse {
    pub fn new(stats: &Stats) -> StatsResponse {
        let secs = (SteadyTime::now() - stats.clear_time).num_seconds();
        let uptime = (SteadyTime::now() - stats.start_time).num_seconds();
        StatsResponse {
            announce_rate: stats.announces/secs as u64,
            scrape_rate: stats.scrapes/secs as u64,
            torrents: stats.torrents,
            peers: stats.peers,
            uptime: uptime as u64,
        }
    }
}
