use arr_macro::arr;

const PEERS: usize = 20;
const ROUNDS: u64 = 20;

#[derive(Clone, Copy, Debug)]
enum Request {
    Dispatched,
    None,
}

#[derive(Clone, Copy, Debug)]
enum Interaction {
    Coop,
    Defect,
    Helpless,
    None,
}

#[derive(Clone, Copy, Debug)]
enum Status {
    Online,
    Offline,
}

// The outer vector tracks the destination of the requests, the inner the source
// type Requests = [[Request; PEERS]; PEERS];
type History = Vec<[Interaction; PEERS]>;
// type Acquaintances = Vec<&'static Peer<'a>>;

#[derive(Clone)]
struct Strategy {
    decide: fn(&Peer, &Peer, &History) -> Interaction,
    stranger: fn(&Peer, &Peer, &History) -> Interaction,
    server: fn(&'static [Peer], &History) -> &'static Peer<'static>,
}

#[derive(Clone)]
struct Peer<'a> {
    strat: Strategy,
    acq: Vec<&'static Peer<'static>>,
    req: &'a [Request; PEERS],
    status: Status,
    private: Vec<Interaction>,
}

fn coop(_me: &Peer, _other: &Peer, _hist: &History) -> Interaction {
    Interaction::Coop
}

fn defect(_me: &Peer, _other: &Peer, _hist: &History) -> Interaction {
    Interaction::Defect
}

fn server(peers: &'static [Peer], _hist: &History) -> &'static Peer<'static> {
    &peers[0]
}

fn stranger(_me: &Peer, _other: &Peer, _hist: &History) -> Interaction {
    Interaction::Coop
}

fn main() {
    let mut requests = [[Request::None; PEERS]; PEERS];
    // Integer in arr![;_] should match the number PEERS
    let mut peers = arr![Peer { strat: Strategy { decide: coop, stranger: coop, server }, acq: vec![], req: &requests[0], status: Status::Offline, private: vec![]}; 20];
}
