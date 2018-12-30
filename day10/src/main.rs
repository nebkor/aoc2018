use day10::*;

fn main() {
    let beacons = get_input();

    let sky = Sky { beacons: beacons };

    let mut ymin = MAX;
    for t in 0.. {
        let (_, r) = &sky.t(t);
        let ny = r.yspan;
        if ny > ymin {
            for nt in (t - 5)..(t + 5) {
                println!("{}:\n{}", nt, sky.print(nt));
            }
            break;
        }
        ymin = ny;
    }
}
