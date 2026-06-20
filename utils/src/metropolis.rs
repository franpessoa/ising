use rand::Rng;

pub trait Metropolis {
    fn p_acc(&self, o: &Self) -> f64;
    fn prop(&self) -> Self;
}

pub fn metropolis_pass<T: Metropolis>(s_zero: T, mut rng: impl Rng) -> T {
    // Propõe um novo estado `s_prop`
    let s_prop = s_zero.prop();

    let alpha = s_zero.p_acc(&s_prop);

    let p_sort = rng.next_u32() as f64 / u32::MAX as f64;

    if p_sort <= alpha {
        return s_prop;
    }

    return s_zero;
}
