use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::rc::Rc;

use nock_interpreter::nock_4k_rc::nock_4k_rc::Noun;

fn create_decrement_formula_rc() -> Rc<Noun> {
    Noun::cell(
        Noun::atom(8),
        Noun::cell(
            Noun::cell(Noun::atom(1), Noun::atom(0)),
            Noun::cell(
                Noun::atom(8),
                Noun::cell(
                    Noun::cell(
                        Noun::atom(1),
                        Noun::cell(
                            Noun::atom(6),
                            Noun::cell(
                                Noun::cell(
                                    Noun::atom(5),
                                    Noun::cell(
                                        Noun::cell(Noun::atom(0), Noun::atom(7)),
                                        Noun::cell(
                                            Noun::atom(4),
                                            Noun::cell(Noun::atom(0), Noun::atom(6)),
                                        ),
                                    ),
                                ),
                                Noun::cell(
                                    Noun::cell(Noun::atom(0), Noun::atom(6)),
                                    Noun::cell(
                                        Noun::atom(9),
                                        Noun::cell(
                                            Noun::atom(2),
                                            Noun::cell(
                                                Noun::cell(Noun::atom(0), Noun::atom(2)),
                                                Noun::cell(
                                                    Noun::cell(
                                                        Noun::atom(4),
                                                        Noun::cell(
                                                            Noun::atom(0),
                                                            Noun::atom(6),
                                                        ),
                                                    ),
                                                    Noun::cell(Noun::atom(0), Noun::atom(7)),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                    Noun::cell(
                        Noun::atom(9),
                        Noun::cell(Noun::atom(2), Noun::cell(Noun::atom(0), Noun::atom(1))),
                    ),
                ),
            ),
        ),
    )
}

fn bench_nock_4k_rc_decrement(c: &mut Criterion) {
    let formula = create_decrement_formula_rc();
    c.bench_function("nock_4k_rc decrement", |b| {
        b.iter(|| {
            let subject = Noun::atom(black_box(100));
            let input = Noun::cell(subject, formula.clone());
            black_box(Noun::tar(input))
        })
    });
}

criterion_group!(benches, bench_nock_4k_rc_decrement);
criterion_main!(benches);