use rand::Rng;
use zxcvbn::zxcvbn;

pub fn generate_random_password(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut charset = String::new();
    if uppercase {
        //should remove uppercase letter 'O' to avoid confusion with number '0', remove uppercase letter 'I' to avoid confusion with lowercase letter 'l'  and '1'
        charset.push_str("ABCDEFGHJKLMNOPQRSTUVWXYZ");
    }
    if lowercase {
        //remove lowercase letter 'l' and 'o' to avoid confusion with number '1' and '0'
        charset.push_str("abcdefghijkmnpqrstuvwxyz");
    }
    if number {
        //should remove number '0' to avoid confusion with uppercase letter 'O', remove number '1' to avoid confusion with lowercase letter 'l' and uppercase letter 'I'
        charset.push_str("23456789");
    }
    if symbol {
        //should remove symbols which escape the shell, such as $ & ` " ' \ /
        charset.push_str("!@#$%^*()-_=+[]{}|;:,.<>?");
    }

    let mut rng = rand::rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();
    let estimate = zxcvbn(&password, &[]);
    eprintln!(
        "Score: {}, Crack times: {:?}",
        estimate.score(),
        estimate.crack_times()
    );
    Ok(password)
}
