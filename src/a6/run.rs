use std::cmp::min;

fn main() {


    //let races = [(42,284), (68,1005), (69, 1122), (85,1341)];

    let races = [(42686985u64,284100511221341u64)];

    //let races = [(7,9), (15,40), (30, 200)];


    let mut tot_mul = 1;
    for (time, distance) in races {
        let mut tot_ways = 0;
        // for t in 0..=time {
        //     let mut tot_dist = 0;
        //     let speed = t;
        //     for t2 in (t+1)..=time {
        //        tot_dist += speed;
        //     }
        //     //tot_dist = speed * (time - t );
        //
        //     if(tot_dist > distance) {
        //         tot_ways += 1;
        //         println!("speed {speed} tot_dist {tot_dist}");
        //     }
        // }
        //dist = t * (time - t);
        //dist = t* time - t*t;
        //t*t + t*time - dist = 0
        let a = 1.0;
        let b = -(time as i64) as f64;
        let c = (distance+1) as f64;
        let aa = -b/(2.0*a);
        let a2 = aa + (aa*aa - (c/a)).sqrt();
        let a1 = aa - (aa*aa - (c/a)).sqrt();
        println!("aa {a1}   a2 {a2}");

        let mut aa1: u64 = (a1-0.0000001) as u64;
        let mut aa2: u64 = a2 as u64;
        aa1 = min(aa1, time);
        aa2 = min(aa2, time);

        let diff = aa2 - aa1;


        println!("{tot_ways} {diff}");
        tot_mul *= tot_ways;
    }
    println!("part 1: {tot_mul}");
    //687960
}

