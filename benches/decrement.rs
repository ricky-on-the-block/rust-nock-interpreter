use criterion::{black_box, criterion_group, criterion_main, Criterion};

use nock_interpreter::nock_4k_box::nock_4k_box::Noun as NounBox;
use nock_interpreter::nock_4k_rc::nock_4k_rc::Noun as NounRc;

fn create_decrement_formula_box() -> NounBox {
    NounBox::cell(
        NounBox::atom(8),
        NounBox::cell(
            NounBox::cell(NounBox::atom(1), NounBox::atom(0)),
            NounBox::cell(
                NounBox::atom(8),
                NounBox::cell(
                    NounBox::cell(
                        NounBox::atom(1),
                        NounBox::cell(
                            NounBox::atom(6),
                            NounBox::cell(
                                NounBox::cell(
                                    NounBox::atom(5),
                                    NounBox::cell(
                                        NounBox::cell(NounBox::atom(0), NounBox::atom(7)),
                                        NounBox::cell(
                                            NounBox::atom(4),
                                            NounBox::cell(NounBox::atom(0), NounBox::atom(6)),
                                        ),
                                    ),
                                ),
                                NounBox::cell(
                                    NounBox::cell(NounBox::atom(0), NounBox::atom(6)),
                                    NounBox::cell(
                                        NounBox::atom(9),
                                        NounBox::cell(
                                            NounBox::atom(2),
                                            NounBox::cell(
                                                NounBox::cell(NounBox::atom(0), NounBox::atom(2)),
                                                NounBox::cell(
                                                    NounBox::cell(
                                                        NounBox::atom(4),
                                                        NounBox::cell(
                                                            NounBox::atom(0),
                                                            NounBox::atom(6),
                                                        ),
                                                    ),
                                                    NounBox::cell(
                                                        NounBox::atom(0),
                                                        NounBox::atom(7),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                    NounBox::cell(
                        NounBox::atom(9),
                        NounBox::cell(
                            NounBox::atom(2),
                            NounBox::cell(NounBox::atom(0), NounBox::atom(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}

fn create_decrement_formula_rc() -> NounRc {
    NounRc::cell(
        NounRc::atom(8),
        NounRc::cell(
            NounRc::cell(NounRc::atom(1), NounRc::atom(0)),
            NounRc::cell(
                NounRc::atom(8),
                NounRc::cell(
                    NounRc::cell(
                        NounRc::atom(1),
                        NounRc::cell(
                            NounRc::atom(6),
                            NounRc::cell(
                                NounRc::cell(
                                    NounRc::atom(5),
                                    NounRc::cell(
                                        NounRc::cell(NounRc::atom(0), NounRc::atom(7)),
                                        NounRc::cell(
                                            NounRc::atom(4),
                                            NounRc::cell(NounRc::atom(0), NounRc::atom(6)),
                                        ),
                                    ),
                                ),
                                NounRc::cell(
                                    NounRc::cell(NounRc::atom(0), NounRc::atom(6)),
                                    NounRc::cell(
                                        NounRc::atom(9),
                                        NounRc::cell(
                                            NounRc::atom(2),
                                            NounRc::cell(
                                                NounRc::cell(NounRc::atom(0), NounRc::atom(2)),
                                                NounRc::cell(
                                                    NounRc::cell(
                                                        NounRc::atom(4),
                                                        NounRc::cell(
                                                            NounRc::atom(0),
                                                            NounRc::atom(6),
                                                        ),
                                                    ),
                                                    NounRc::cell(NounRc::atom(0), NounRc::atom(7)),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                    NounRc::cell(
                        NounRc::atom(9),
                        NounRc::cell(
                            NounRc::atom(2),
                            NounRc::cell(NounRc::atom(0), NounRc::atom(1)),
                        ),
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
            let subject = NounRc::atom(black_box(100));
            black_box(NounRc::tar(NounRc::cell(subject, formula.clone())))
        })
    });
}

fn bench_nock_4k_box_decrement(c: &mut Criterion) {
    let formula = create_decrement_formula_box();
    c.bench_function("nock_4k_box decrement", |b| {
        b.iter(|| {
            let subject = NounBox::atom(black_box(100));
            black_box(NounBox::tar(&subject, &formula))
        })
    });
}

criterion_group!(
    benches,
    bench_nock_4k_rc_decrement,
    bench_nock_4k_box_decrement
);
criterion_main!(benches);
