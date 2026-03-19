use rand::{
    SeedableRng,
    distr::{Distribution, Uniform},
    rngs::SmallRng,
};

#[derive(Clone)]
pub struct Signal<S: Iterator> {
    pub source: S,
    pub points: Vec<S::Item>,
    pub tick_rate: usize,
}

impl<S> Signal<S>
where
    S: Iterator,
{
    pub fn on_tick(&mut self) {
        let drain_count = self.points.len().min(self.tick_rate);
        self.points.drain(0..drain_count);
        self.points
            .extend(self.source.by_ref().take(self.tick_rate));
    }
}

pub struct Signals {
    pub sin1: Signal<SinSignal>,
    pub sin2: Signal<SinSignal>,
    pub window: [f64; 2],
}

impl Signals {
    fn on_tick(&mut self) {
        self.sin1.on_tick();
        self.sin2.on_tick();
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

#[derive(Clone)]
pub struct RandomSignal {
    distribution: Uniform<u64>,
    rng: SmallRng,
}

impl RandomSignal {
    pub fn new(lower: u64, upper: u64) -> Self {
        Self {
            distribution: Uniform::new(lower, upper).unwrap(),
            rng: SmallRng::seed_from_u64(0),
        }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.distribution.sample(&mut self.rng))
    }
}

#[derive(Clone)]
pub struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    pub const fn new(interval: f64, period: f64, scale: f64) -> Self {
        Self {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

#[derive(Clone)]
pub struct PieChartData {
    pub val1: f64,
    pub val2: f64,
    pub val3: f64,
}

impl Default for PieChartData {
    fn default() -> Self {
        Self {
            val1: 33.3,
            val2: 33.3,
            val3: 33.3,
        }
    }
}

#[derive(Clone)]
pub struct BarChartData {
    pub cpu: u64,
    pub gpu: u64,
    pub ram: u64,
    pub io: u64,
    pub network: u64,
}

impl Default for BarChartData {
    fn default() -> Self {
        Self {
            cpu: 30,
            gpu: 25,
            ram: 20,
            io: 15,
            network: 10,
        }
    }
}

impl BarChartData {
    pub fn on_tick(&mut self) {
        self.cpu = (self.cpu + 1) % 100;
        self.gpu = (self.gpu + 2) % 100;
        self.ram = (self.ram + 3) % 100;
        self.io = (self.io + 4) % 100;
        self.network = (self.network + 5) % 100;
    }
}
