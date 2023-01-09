use rayon::prelude::{
    FromParallelIterator, IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator,
    IntoParallelRefMutIterator, ParallelIterator,
};

/// Parallel naiive genetic algorithm that uses crossover and mutation operators.
/// Cross overs mutate the first arg to cross over with the read-only reference arg.
pub fn evolve<T: Send + Sync, U: PartialOrd + Send + Sync>(
    population: &mut [T],
    score: impl Fn(&T) -> U + Send + Sync,
    cross_over: impl Fn(&mut T, &T) + Send + Sync,
    mutate: impl Fn(&mut T) + Send + Sync,
) {
    let len = population.len();
    if len == 0 {
        return;
    }

    let scores = Vec::from_par_iter(population.par_iter().map(score));

    // Culled if equal to a neighbor and under another,
    // under both neighbors, or equal both neighbors.
    let (culled, survive): (Vec<_>, Vec<_>) =
        population.par_iter_mut().enumerate().partition(|(i, _)| {
            let (prev, next) = neighbors(*i, len);
            scores[prev] >= scores[*i] && scores[*i] <= scores[next]
        });

    culled.into_par_iter().for_each(|(i, change)| {
        let (prev, next) = neighbors(i, len);

        let mut compare = next;
        if scores[prev] > scores[next] {
            compare = prev;
        }

        if let Ok(index) = survive.binary_search_by(|(x, _)| x.cmp(&compare)) {
            cross_over(change, survive[index].1);
        }

        mutate(change);
    });
}

/// Wraps around borders, gets "left" and "right" of `index`.
/// Panics in debug if `len` is `0`.
fn neighbors(index: usize, len: usize) -> (usize, usize) {
    let prev = (index + len - 1) % len;
    let next = (index + 1) % len;
    (prev, next)
}

// Referring to test.rs for separate tests file.
#[cfg(test)]
mod test;
