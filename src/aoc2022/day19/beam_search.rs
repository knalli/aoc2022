use itertools::Itertools;

// https://github.com/mfornet/advent-of-code-2022/blob/6aef157f13002ff7bab6390e853c420dfcd0dab5/day19b/src/beam_search.rs
pub trait Node: {
    fn children(&self) -> Vec<Self> where Self: Sized;

    fn score(&self) -> usize;

    fn real_score(&self) -> usize;
}

pub fn beam_search(source: impl Node, width: usize) -> usize {
    let mut beam = vec![source];
    let mut best = 0;

    while !beam.is_empty() {
        let mut next_beam = beam
            .into_iter()
            .flat_map(|n| {
                n.children()
                    .into_iter()
                    .map(|n| (n.score(), n))
            })
            .collect_vec();

        for (_, node) in &next_beam {
            let score = node.real_score();
            if score > best {
                best = score;
            }
        }

        // sort by $score, trunc to first $width
        next_beam.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        next_beam.truncate(width);

        beam = next_beam
            .into_iter()
            .map(|(_, node)| node)
            .collect_vec();
    }

    best
}