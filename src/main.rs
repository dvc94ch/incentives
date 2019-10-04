use arr_macro::arr;

const PEERS: usize = 20;
const ROUNDS: u64 = 20;

// #[derive(Clone, Copy, Debug)]
// enum Request {
//     Dispatched,
//     None,
// }

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
type History = Vec<Vec<Interaction>>;
// type Acquaintances = Vec<&'static Peer<'a>>;

#[derive(Clone)]
struct Strategy<'a> {
    decide: fn(&Peer, &History) -> Interaction,
    stranger: fn(&Peer, &History) -> Interaction,
    // TODO
    server:  fn(&'a [Peer], &History) -> &'a Peer<'a>,
}

#[derive(Clone)]
struct Peer<'a> {
    strat: Strategy<'a>,
    acq: Vec<&'a Peer<'a>>,
    req: Vec<&'a Peer<'a>>,
    status: Status,
    private: Vec<Interaction>,
}

fn coop(_me: &Peer, _other: &Peer, _hist: &History) -> Interaction {
    Interaction::Coop
}

fn defect(_me: &Peer, _other: &Peer, _hist: &History) -> Interaction {
    Interaction::Defect
}

fn server<'a>(peers: &'a [Peer<'a>], _hist: &History) -> &'a Peer<'a> {
    &peers[0]
}

fn stranger(_me: &Peer, _other: &Peer, _hist: &History) -> Interaction {
    Interaction::Coop
}

fn main() {
    let mut requests: Vec<Vec<&Peer>> = vec![];
    // Integer in arr![;_] should match the number PEERS
    let mut peers: Vec<Peer> = vec![];
}
