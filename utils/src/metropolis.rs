use rand::Rng;

pub trait Metropolis {
    fn delta_e(&self, o: &Self) -> f64;
    fn prop(&self) -> Self;
}

pub fn metropolis_pass<T: Metropolis>(s_zero: T, mut rng: impl Rng) -> T {
    // Propõe um novo estado `s_prop`
    let s_prop = s_zero.prop();

    // Calcula as probabilidades do atual e do novo
    let delta_e = s_zero.delta_e(&s_prop);
    // Calcula a taxa de aceitação
    let alpha = if delta_e < 0.0 {
        1.0
    } else {
        f64::exp(-delta_e)
    };

    let p_sort = rng.next_u32() as f64 / u32::MAX as f64;

    if p_sort <= alpha {
        return s_prop;
    }

    return s_zero;
}
