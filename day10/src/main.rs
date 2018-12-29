use day10::*;

fn main() {
    let beacons = get_input();

    let sky = Sky { beacons: beacons };

    let mut ymin = MAX;
    for t in 0..MAX {
        let (_, r) = &sky.t(t);
        let ny = r.yspan;
        if ny > ymin {
            println!("{}:\n{}", t - 1, sky.print(t - 1));
            break;
        }
        ymin = ny;
    }
}
