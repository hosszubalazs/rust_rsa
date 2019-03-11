// KID-RSA
// based on the ideas of Neal Koblitz
// Cryptography as a teaching tool
// https://sites.math.washington.edu/~koblitz/crlogia.html

// rand use to generate keys for the agents
use rand::Rng;

// FIXME A struct might be a stupid choice, the fields are readable by default.
// For now let's take this as a temporary solution.
pub struct KidRsaUser {
    private_key: u64,
    public_key: u64,
    n: u64,
}

impl KidRsaUser {
    fn transform_with_private_key(&self, data: u64) -> u64 {
        (data * self.private_key) % self.n
    }
    fn transform_with_public_key_from(&self, encrypted_data: u64, sender: &KidRsaUser) -> u64 {
        // (data * public_key) % n
        // c % m = (a*b) % m = ((a%m) * (b%m)) % m
        let a = encrypted_data % sender.n;
        let b = sender.public_key % sender.n;
        let c = a * b;
        c % sender.n
    }
}

fn new_kid_rsa_user() -> KidRsaUser {
    let mut rng = rand::thread_rng();
    let range_lower_bound = 2;
    let range_upper_bound = 230;
    // Memory requirements
    // m = n^2 == (a^4)^2 = range_upper_bound^8
    // keys = a ^3 = range_upper_bound^3
    // encrypted data = data * key = data *a^3 --> data can go up to range_upper_bound^5
    // in u64 our upper boundary is approximately: \sqrt[8]{9.22337\dots E18}=234.75303

    let a: u64 = rng.gen_range(range_lower_bound, range_upper_bound);
    // The current range_upper_bound is very low. To avoid conflicting numbers,
    // lets use this workaround
    let ax: u64 = a + 1; 
    // Mutable since we might need to regenerate them
    let mut b: u64;
    let mut bx: u64;
    let mut m: u64;

    let mut public_key_exponent: u64;
    let mut private_key_exponent: u64;

    // This is a sneaky do-while loop
    while {
        b = ax + 1; 
        bx = b + 1; 
        m = a * b - 1;
        public_key_exponent = ax * m + a;
        private_key_exponent = bx * m + b;

        // FIXME: If the keys are equal, the algorithm breaks. Why?
        // This condition was not described in the original paper.
        public_key_exponent == private_key_exponent
    } {}
    println!("a,b={},{}", a, b);
    println!("ax,bx={},{}", ax, bx);
    println!("m={}", m);

    let n: u64 = (ax * bx * m) + (a * bx) + (ax * b) + 1; //(public_key_exponent * private_key_exponent -1 ) / m;

    KidRsaUser {
        private_key: private_key_exponent,
        public_key: public_key_exponent,
        n,
    }
}

// The tests are not super stable
// Please check the FIXMEs in the code for pointers on where it can possibly fail
#[cfg(test)]
mod tests {
    // The logic is organized into private functions outside of the test module.
    // To be able to access them, this use is needed.
    use super::*;

    #[test]
    fn directed_message() {
        for _x in 0..10 {
            // Case 1.A : Alice sends a message using Bob's public key
            // + only Bob can read it
            // - Bob cannot be sure that Alice sent the message

            let alice = new_kid_rsa_user();
            let bob = new_kid_rsa_user();
            let eve = new_kid_rsa_user();

            // Must be smaller then n
            let data = 123;

            // Alice sings the data with Bob's public key, and sends it as a public message
            let data_signed_with_public_key = alice.transform_with_public_key_from(data, &bob);

            println!(
                "Alice e,d,n={},{},{}",
                alice.public_key, alice.private_key, alice.n
            );
            println!("Bob e,d,n={},{},{}", bob.public_key, bob.private_key, bob.n);
            println!("Eve e,d,n={},{},{}", eve.public_key, eve.private_key, eve.n);

            // Bob can safely read the message
            let unwrapped_data = bob.transform_with_private_key(data_signed_with_public_key);
            assert_eq!(data, unwrapped_data);

            // Eve cannot read the message
            let corrupt_data = eve.transform_with_private_key(data_signed_with_public_key);
            assert_ne!(data, corrupt_data);

            // Case 1.B : Bob does not know who sent the message
            // Eve sends Bob a message
            // Bob can read the message, others cant
            // Bob cannot be sure who sent the message

            let evil_data = 99;

            // Eve creates a message that Bob can read, but the writer cannot be verified
            let evil_data_for_bob = eve.transform_with_public_key_from(evil_data, &bob);
            let naively_read_message = bob.transform_with_private_key(evil_data_for_bob);
            assert_eq!(evil_data, naively_read_message);
        }
    }

    #[test]
    fn digital_signature() {
        for _x in 0..10 {
            // Case 2.A : Digital Signature, Alice sends Bob a message, using her private key
            // + Bob can be sure the message came from Alice
            // - Anyone snooping the network can also read the message (open for man-in-the-middle attack)

            let alice = new_kid_rsa_user();
            let bob = new_kid_rsa_user();
            let eve = new_kid_rsa_user();

            // Must be smaller then n
            let data = 100;
            let data_signed_by_alice = alice.transform_with_private_key(data);

            println!(
                "Alice e,d,n={},{},{}",
                alice.public_key, alice.private_key, alice.n
            );
            println!("Bob e,d,n={},{},{}", bob.public_key, bob.private_key, bob.n);
            println!("Eve e,d,n={},{},{}", eve.public_key, eve.private_key, eve.n);

            let unwrapped_data = bob.transform_with_public_key_from(data_signed_by_alice, &alice);
            assert_eq!(data, unwrapped_data);

            // Only Alice's public key works to recover the data
            let corrupt_data = bob.transform_with_public_key_from(data_signed_by_alice, &eve);
            assert_ne!(data, corrupt_data);

            // Case 2.B
            // Everyone can read the data, not just Bob
            let snooped_data = eve.transform_with_public_key_from(data_signed_by_alice, &alice);
            assert_eq!(data, snooped_data);
        }
    }

    #[test]
    fn complex_encryption() {
        // Case 3 : Alice sends Bob a message, encrypting both with her private and his public key
        // + Bob can be sure the message came from Alice
        // + No one else can read the message

        let alice = new_kid_rsa_user();
        let bob = new_kid_rsa_user();
        let eve = new_kid_rsa_user();

        // Must be smaller then n
        let data = 100;

        // Encryption by Alice
        let data_signed_by_alice = alice.transform_with_private_key(data);
        let double_signed_data = alice.transform_with_public_key_from(data_signed_by_alice, &bob);

        println!(
            "Alice e,d,n={},{},{}",
            alice.public_key, alice.private_key, alice.n
        );
        println!("Bob e,d,n={},{},{}", bob.public_key, bob.private_key, bob.n);
        println!("Eve e,d,n={},{},{}", eve.public_key, eve.private_key, eve.n);

        // Decryption by Bob
        let only_bob_can_read_this = bob.transform_with_private_key(double_signed_data);
        let alice_sent_this_and_only_bob_can_read_this =
            bob.transform_with_public_key_from(only_bob_can_read_this, &alice);
        assert_eq!(data, alice_sent_this_and_only_bob_can_read_this);
    }
}
