// rand use to generate keys for the agents
use rand::Rng;

pub struct KidRsaUser {
    private_key: u64,
    pub public_key: u64,
    pub n: u64,
}

impl KidRsaUser {
    fn transform_data_with_key(&self, data: u64, n: u64, key: u64) -> u64 {
        if data > n {
            panic!("Data({}) must be lower than n({}) of key", data, n);
        }
        // Optimising for low memory usage by distributing the mods
        // required result: (data * public_key) % n
        // used equality:   c % m = (a*b) % m = ((a%m) * (b%m)) % m
        let a = data % n;
        let b = key % n;
        //let transformed_data = a * b;
        (a * b) % n
    }
    pub fn transform_with_private_key(&self, data: u64) -> u64 {
        self.transform_data_with_key(data, self.n, self.private_key)
    }
    pub fn transform_with_public_key_from(&self, data: u64, sender: &KidRsaUser) -> u64 {
        self.transform_data_with_key(data, sender.n, sender.public_key)
    }
}

pub fn new_kid_rsa_user() -> KidRsaUser {
    let mut rng = rand::thread_rng();

    // These ranges have two major cnosequences:
    // The higher the lower bound, the bigger n can be --> Encoded data can be bigger --> messages can be bigger
    // The wider the range inside the boundaries, the more agents can be safely generated, without conflicts
    let range_lower_bound = 500;
    let range_upper_bound = 550;

    // Memory requirements ??
    // m = n^2 == (a^4)^2 = range_upper_bound^8
    // keys = a ^3 = range_upper_bound^3
    // encrypted data = data * key = data *a^3 --> data can go up to range_upper_bound^5
    // in u64 our upper boundary is approximately: \sqrt[8]{9.22337\dots E18}=234.75303 ?? No!
    // The calculations above are incorrect. As experienced, choosing numbers up to 550 is well stable.

    // Mutable since we might need to regenerate them
    let mut a: u64;
    let mut ax: u64;
    let mut b: u64;
    let mut bx: u64;
    let mut m: u64;

    let mut public_key_exponent: u64;
    let mut private_key_exponent: u64;
    let n: u64;

    // This is a sneaky do-while loop
    while {
        // Previously I seemed to have issues where any numbers of a,ax,b,bx were equal.
        // That issue does not reproduce anymore, so I can just generate the numbers.
        a = rng.gen_range(range_lower_bound, range_upper_bound);
        ax = rng.gen_range(range_lower_bound, range_upper_bound);
        b = rng.gen_range(range_lower_bound, range_upper_bound);
        bx = rng.gen_range(range_lower_bound, range_upper_bound);

        m = a * b - 1;
        public_key_exponent = ax * m + a;
        private_key_exponent = bx * m + b;

        // This condition was not described in the original paper.
        public_key_exponent == private_key_exponent
    } {}

    // n = (public_key_exponent * private_key_exponent -1 ) / m
    n = (ax * bx * m) + (a * bx) + (ax * b) + 1;

    KidRsaUser {
        private_key: private_key_exponent,
        public_key: public_key_exponent,
        n,
    }
}
