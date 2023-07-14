unsafe fn solve() {
    input!(n: usize);
    let mut vector: Vec<usize> = n.to_string().chars().map(|c| (c as u8 -b'0') as usize).collect();
    vector.reverse();
    let m = vector.len();
    let mut set = [0; 10];
    for i in 0..m {
        set[vector[i]] += 1;
    }
    // 543212345 -> 543212340 -> 543212300 -> 543212000 -> ...
    for i in 0..m {
        if vector[i] == 0 { continue }
        for j in i + 1..m {
            set[vector[j]] += vector[i] * 10usize.pow(i as u32);
        }
        // 300 -> 299 -> 000
        let floor = if i < m - 1 { 0 } else { 1 };
        for k in floor..vector[i] {
            set[k] += 10usize.pow(i as u32);
        }
        if i == 0 { continue }
        for k in 0..10 {
            set[k] += vector[i] * 10usize.pow(i as u32 - 1) * i;
        }
        // 1000 / 0999 / 0099 / 0009 / 0000
        if i == m - 1 {
            for k in 0..i {
                set[0] -= 10usize.pow(k as u32);
            }
        }
    }
    for i in 0..10 {
        print!("{} ", set[i]);
    }
}
