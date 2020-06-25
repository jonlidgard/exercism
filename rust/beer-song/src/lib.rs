// n >= 2
// m = n - 1
// s = n > 2 s : ''
// <n> bottle<s> of beer on the wall, <n> bottle<s> of beer.\nTake one down and pass it around, <m> bottle<s> of beer on the wall.\n

// 1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n

// No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n


//    When n > 1, m = n - 1, capitalize - 0

//    n m c
//      2 1 ?
//      1 0 0
//      0 99 0 then 1

pub fn format_bottle(n: u32) -> (String, String) {
    const NO_MORE: &'static str = "o more bottles";
    const X: &'static [&'static str] = &["Go to the store and buy some more","Take it down and pass it around","Take one down and pass it around"];
    
    let static mut capitalize = false;
    let bottles = n.to_string();
    let mut m = n;
    if m > 2 {
        m = 2;
    }

    match n {
        0 => unsafe {
            if capitalize {
                format!("N{}", X[m as usize])
            } else {
                format!("n{}", NO_MORE)                
            }
        },
        1 => { 
            (format!("{} bottle", bottles), TAKE_IT_DOWN)
        }
        _ => (format!("{} bottles", bottles), TAKE_ONE_DOWN)
    }
}


pub fn verse(n: u32) -> String {
    //unimplemented!("emit verse {}", n)
    

    let bottles1 = format_bottle(n, true);
    let part1 = format!("{} of beer", bottles1);
    let mut part2 = String::new(); //from("no more bottles");
    println!("*** {}", bottles1);
    let mut bottles2 = format_bottle(99, false);
    println!("*** {}", bottles2);
    let mut part3 = BUY_MORE;
    if n > 0 {
        bottles2 = format_bottle(n - 1, false);
        part2 = part1.clone();
        if n > 1 {
            part3 = TAKE_ONE_DOWN;
        } else {
            part3 = "";
        }
    } else {
    }

    //let mut part1x = part1;
    // if n == 0 {
    //     part1x = String::from("no more bottles");
    // }
    let part4 = format!("{} of beer", bottles2);

    let line = format!(
        "{} on the wall, {}.\n{}, {} on the wall.\n",
        part1, part2, part3, part4
    );
    println!("{}", line);
    line
}

pub fn sing(start: u32, end: u32) -> String {
    //  unimplemented!("sing verses {} to {}, inclusive", start, end)
    let mut song = verse(start);
    let mut v = start;
    loop {
        v = v - 1;
        song = format!("{}\n{}", song, verse(v));
        if v == end {
            break;
        };
    }
    song
}
