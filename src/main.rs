use num_integer::gcd;
use std::io;

fn main() {
    println!("Welcome to my small RSA coder / decoder");
    println!("Please check the tests below to see how they work");
    println!("This interactive way of running the program is for debug purposes.");
    println!("---");
    println!("Please input the following integers:p, q, k, message");

    let p = read_number_from_user();

    let q = read_number_from_user();
    let k = read_number_from_user();
    let data = read_number_from_user();

    let phi = calculate_phi(p, q);
    let e = calculate_e(phi);
    let d = calculate_d(k, phi, e);
    let n = p * q;

    let encrypted_data :u64 = encrypt_data(data, e, n).into();
    let decrypted_data = decrypt_data(encrypted_data, d, n.into());

    println!("Encrypted data:{}", encrypted_data);
    println!("Decrypted data:{}", decrypted_data);
}

fn read_number_from_user() -> u32 {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: u32 = n.trim().parse().expect("invalid input");
    n
}
fn calculate_e(phi: u32) -> u32 {
    let mut e = 2;
    while e < phi {
        if gcd(e, phi) == 1 {
            break;
        } else {
            e += 1;
        }
    }
    e
}

fn calculate_phi(p: u32, q: u32) -> u32 {
    (p - 1) * (q - 1)
}

fn calculate_d(k: u32, phi: u32, e: u32) -> u32 {
    (1 + (k * phi)) / e
}

fn encrypt_data(data: u32, encrypt: u32, n: u32) -> u32 {
    data.pow(encrypt) % n
}

fn decrypt_data(encrypted_data: u64, decrypt: u32, n: u64) -> u64 {
    // optimizing the modular exponentiation for low memory usage
    // we need as an endresult (encrypted_data^decrypt)%n
    // c % m = (a*b) % m = ((a%m) * (b%m)) % m
    //let pow: u64 = encrypted_data.pow(decrypt).into();
    //pow % n
    let mut decrypted_data: u64 = 1;
    let mut decrypt_index = 0;
    while decrypt_index < decrypt {
        decrypted_data = (decrypted_data * encrypted_data) % n;
        decrypt_index += 1;
    }

    decrypted_data
}

#[cfg(test)]
mod tests {
    // The logic is organized into private functions outside of the test module.
    // To be able to access them, this use is needed.
    use super::*;

    #[test]
    fn test_encrypt_decrypt_0() {
        // Select two primes, P and Q
        // For now they have to be very little, otherwise decryption fails. This is demonstrated by tests.
        let p = 3;
        let q = 5;
        // FIXME what is a good strategy to choose k?
        let k = 4;
        // The data we want to send
        let data = 12;

        // Calculate modulus 'n'
        let n = p * q;
        // Calculate the totient:
        // the number of positive integers smaller than n which are coprime to n
        let phi = calculate_phi(p, q);
        //Calculate the public and private key exponents
        let e = calculate_e(phi);
        let d = calculate_d(k, phi, e);

        let encrypted_data :u64 = encrypt_data(data, e, n).into();
        let decrypted_data = decrypt_data(encrypted_data, d, n.into());

        assert_eq!(3, encrypted_data);
        assert_eq!(12, decrypted_data);
    }

    #[test]
    fn test_encrypt_decrypt_1() {
        let p = 3;
        let q = 7;
        let k = 7;
        let data = 12;

        let phi = calculate_phi(p, q);
        let e = calculate_e(phi);
        let d = calculate_d(k, phi, e);
        let n = p * q;

        let encrypted_data :u64 = encrypt_data(data, e, n).into();
        let decrypted_data = decrypt_data(encrypted_data, d, n.into());

        assert_eq!(3, encrypted_data);
        assert_eq!(12, decrypted_data);
    }

    #[test]
    fn test_calculate_e() {
        assert_eq!(3, calculate_e(8));
    }

    #[test]
    fn test_calculate_phi_0() {
        assert_eq!(8, calculate_phi(3, 5));
    }
    #[test]
    fn test_calculate_phi_1() {
        assert_eq!(12, calculate_phi(3, 7));
    }
    #[test]
    fn test_calculate_phi_2() {
        assert_eq!(24, calculate_phi(5, 7));
    }

    #[test]
    fn test_calculate_e_d() {
        let phi = 8;
        let e = calculate_e(phi);
        assert_eq!(3, e);
        let d = calculate_d(2, phi, e);
        assert_eq!(5, d);
    }

    #[test]
    fn test_calculate_d() {
        assert_eq!(5, calculate_d(2, 8, 3));
    }

    #[test]
    fn test_encrypt() {
        assert_eq!(3, encrypt_data(12, 5, 21));
    }

    #[test]
    fn test_decrypt_0() {
        assert_eq!(12, decrypt_data(3, 5, 21));
    }

    #[test]
    fn test_decrypt_1() {
        //p,q = 53, 59
        assert_eq!(12, decrypt_data(1728, 2011, 3127));
    }

    #[test]
    #[should_panic]
    fn decrypter_dies_on_notsolarge_numbers() {
        //p, q = 5, 7
        assert_eq!(12, decrypt_data(17, 9, 35));
    }
}
