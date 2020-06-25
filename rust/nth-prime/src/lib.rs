pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2u32];
    let mut notprime: bool = false;

    while (n as usize) >= primes.len() {
        let mut guess = primes.last().unwrap() + 1;
        loop {
            for p in &primes {
                notprime = guess % p == 0;
                if notprime {
                    guess = guess + 1;
                    break;
                }
            }
            if notprime == false {
                primes.push(guess);
                break;
            }
        }
    }
    *primes.last().unwrap()
}
