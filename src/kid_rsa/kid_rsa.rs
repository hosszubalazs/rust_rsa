// KID-RSA
// based on the ideas of Neal Koblitz
// Cryptography as a teaching tool
// https://sites.math.washington.edu/~koblitz/crlogia.html

use super::kid_rsa_user::KidRsaUser;
use super::kid_rsa_user::new_kid_rsa_user;

use rand::Rng;

#[cfg(test)]
mod tests {
    // The logic is organized into private functions outside of the test module.
    // To be able to access them, this use is needed.
    use super::*;

    // Currently we are very limited on the total possible number of agents.
    // for now this is accepted, and will be addressed.
    // For testing purposes we ignore this limitationg by forcing unique users trough generation
    fn generate_three_users() -> (KidRsaUser, KidRsaUser, KidRsaUser) {
        let alice = new_kid_rsa_user();
        let mut bob: KidRsaUser;
        let mut eve: KidRsaUser;

        while {
            bob = new_kid_rsa_user();
            bob.public_key == alice.public_key
        } {}
        while {
            eve = new_kid_rsa_user();
            eve.public_key == alice.public_key || eve.public_key == bob.public_key
        } {}
        (alice, bob, eve)
    }

    #[test]
    fn directed_message() {
        let mut rng = rand::thread_rng();

        for _x in 0..100 {
            // Case 1.A : Alice sends a message using Bob's public key
            // + only Bob can read it
            // - Bob cannot be sure that Alice sent the message
            let (alice, bob, eve) = generate_three_users();

            // raw data, and all encoded data must be smaller than n.
            let data = rng.gen_range(10, 400);

            // Alice sings the data with Bob's public key, and sends it as a public message
            //alice.validate_decoded_data_lenght(data);
            let data_signed_with_public_key = alice.transform_with_public_key_from(data, &bob);

            // Bob can safely read the message
            let unwrapped_data = bob.transform_with_private_key(data_signed_with_public_key);
            assert_eq!(data, unwrapped_data);

            // Eve cannot read the message
            let corrupt_data = eve.transform_with_private_key(data_signed_with_public_key);
            assert_ne!(
                data, corrupt_data,
                "Eve was successful at reading the secret data, please use bigger random numbers."
            );

            // Case 1.B : Bob does not know who sent the message
            // Eve sends Bob a message
            // Bob can read the message, others cant
            // Bob cannot be sure who sent the message

            // raw data, and all encoded data must be smaller than n.
            let evil_data = 11;

            // Eve creates a message that Bob can read, but the writer cannot be verified
            let evil_data_for_bob = eve.transform_with_public_key_from(evil_data, &bob);
            let naively_read_message = bob.transform_with_private_key(evil_data_for_bob);
            assert_eq!(evil_data, naively_read_message);
        }
    }

    #[test]
    fn digital_signature() {
        let mut rng = rand::thread_rng();

        for _x in 0..100 {
            // Case 2.A : Digital Signature, Alice sends Bob a message, using her private key
            // + Bob can be sure the message came from Alice
            // - Anyone snooping the network can also read the message (open for man-in-the-middle attack)

            let (alice, bob, eve) = generate_three_users();

            assert!( (alice.public_key != bob.public_key)
            && (bob.public_key != eve.public_key)
            && (eve.public_key != alice.public_key), "Public keys of all agents are not unique, please use bigger numbers for random generation!" );

            // raw data, and all encoded data must be smaller than n.
            let data = rng.gen_range(10, 400);

            let data_signed_by_alice = alice.transform_with_private_key(data);

            let unwrapped_data = bob.transform_with_public_key_from(data_signed_by_alice, &alice);
            assert_eq!(data, unwrapped_data);

            // Only Alice's public key works to recover the data
            let corrupt_data = bob.transform_with_public_key_from(data_signed_by_alice, &eve);
            assert_ne!(
                data, corrupt_data,
                "Eve was successful at reading the secret data, please use bigger random numbers."
            );

            // Case 2.B
            // Everyone can read the data, not just Bob
            let snooped_data = eve.transform_with_public_key_from(data_signed_by_alice, &alice);
            assert_eq!(data, snooped_data);
        }
    }

    #[test]
    fn complex_encryption() {
        let mut rng = rand::thread_rng();

        for _x in 0..100 {
            // Case 3 : Alice sends Bob a message, encrypting both with her private and his public key
            // + Bob can be sure the message came from Alice
            // + No one else can read the message

            let (alice, bob, eve) = generate_three_users();

            // raw data, and all encoded data must be smaller than n.
            let data = rng.gen_range(10, 200);

            // Encryption by Alice
            let data_signed_by_alice = alice.transform_with_private_key(data);
            let double_signed_data =
                alice.transform_with_public_key_from(data_signed_by_alice, &bob);

            // Decryption by Bob

            let only_bob_can_read_this = bob.transform_with_private_key(double_signed_data);
            println!("only_bob_can_read_this,{}", only_bob_can_read_this);

            let alice_sent_this_and_only_bob_can_read_this =
                bob.transform_with_public_key_from(only_bob_can_read_this, &alice);
            assert_eq!(data, alice_sent_this_and_only_bob_can_read_this);
        }
    }
}
