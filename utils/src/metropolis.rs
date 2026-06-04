use rand::Rng;

pub trait Metropolis {
    fn p(&self) -> f64;
    fn prop(&self) -> Self;
}

pub fn metropolis_pass<T: Metropolis>(s_zero: T, mut rng: impl Rng) -> T {
    // Propõe um novo estado `s_prop`
    let s_prop = s_zero.prop();

    // Calcula as probabilidades do atual e do novo
    let p_self = s_zero.p();
    let p_prop = s_prop.p();

    // Calcula a taxa de aceitação
    let alpha = f64::min(1.0, p_prop / p_self);
    let p_sort = rng.next_u32() as f64 / u32::MAX as f64;

    if p_sort <= alpha {
        return s_prop;
    }

    return s_zero;
}
