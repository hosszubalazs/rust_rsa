use num_integer::gcd;

fn main() {
    println!("Welcome to my small RSA coder / decoder");
    println!("The functionality is implemented as a heap of functions.");
    println!("Please check the tests below to see how they work");
}

fn calculate_e(_phi: u32) -> u32 {
    let mut _e = 2;
    while _e < _phi {
        if gcd(_e, _phi) == 1 {
            break;
        } else {
            _e += 1;
        }
    }
    _e
}

fn calculate_phi(_p: u32, _q: u32) -> u32 {
    (_p - 1) * (_q - 1)
}

fn calculate_d(_k: u32, _phi: u32, _e: u32) -> u32 {
    (1 + (_k * _phi)) / _e
}

fn encrypt_data(_data: u32, _encrypt: u32, _n: u32) -> u32 {
    _data.pow(_encrypt) % _n
}

fn decrypt_data(_encrypted_data: u32, _decrypt: u32, _n: u64) -> u64 {
    let _pow: u64 = _encrypted_data.pow(_decrypt).into();
    _pow % _n
}

#[test]
fn test_encrypt_decrypt_0() {
    // Select two primes, P and Q
    // For now they have to be very little, otherwise decryption fails. This is demonstrated by tests.
    let _p = 3;
    let _q = 5;
    // FIXME what is a good strategy to choose k?
    let _k = 4;
    // The data we want to send
    let _data = 12;

    // Calculate modulus 'n'
    let _n = _p * _q;
    // Calculate the totient:
    // the number of positive integers smaller than n which are coprime to n
    let _phi = calculate_phi(_p, _q);
    //Calculate the public and private key exponents
    let _e = calculate_e(_phi);
    let _d = calculate_d(_k, _phi, _e);

    let _encrypted_data = encrypt_data(_data, _e, _n);
    let _decrypted_data = decrypt_data(_encrypted_data, _d, _n.into());

    assert_eq!(3, _encrypted_data);
    assert_eq!(12, _decrypted_data);
}

#[test]
fn test_encrypt_decrypt_1() {
    let _p = 3;
    let _q = 7;
    let _k = 7;
    let _data = 12;

    let _phi = calculate_phi(_p, _q);
    let _e = calculate_e(_phi);
    let _d = calculate_d(_k, _phi, _e);
    let _n = _p * _q;

    let _encrypted_data = encrypt_data(_data, _e, _n);
    let _decrypted_data = decrypt_data(_encrypted_data, _d, _n.into());

    assert_eq!(3, _encrypted_data);
    assert_eq!(12, _decrypted_data);
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
    let _phi = 8;
    let _e = calculate_e(_phi);
    assert_eq!(3, _e);
    let _d = calculate_d(2, _phi, _e);
    assert_eq!(5, _d);
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
fn test_decrypt() {
    assert_eq!(12, decrypt_data(3, 5, 21));
}

#[test]
#[should_panic]
fn decrypter_dies_on_large_numbers() {
    //thread '...' panicked at 'attempt to multiply with overflow'
    //p,q = 53, 59
    assert_eq!(12, decrypt_data(1728, 2011, 3127));
}

#[test]
#[should_panic]
fn decrypter_dies_on_notsolarge_numbers() {
    //p, q = 5, 7
    assert_eq!(12, decrypt_data(17, 9, 35));
}
