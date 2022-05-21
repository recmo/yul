use criterion::{black_box, Criterion};
use logos::Logos;
use yul_parser::{parse, Lexer, Token};

fn bench_source(c: &mut Criterion, name: &str, input: &str) {
    c.bench_function(&format!("logos {}", name), |b| {
        b.iter(|| {
            for token in Token::lexer(black_box(input)) {
                black_box(token);
            }
        })
    });
    c.bench_function(&format!("lex {}", name), |b| {
        b.iter(|| {
            for token in Lexer::new(black_box(input)) {
                black_box(token);
            }
        })
    });
    c.bench_function(&format!("parse {}", name), |b| {
        b.iter(|| black_box(parse(black_box(input))))
    });
}

fn main() {
    let mut criterion = criterion::Criterion::default().configure_from_args();

    bench_source(
        &mut criterion,
        "example",
        include_str!("../test/example.yul"),
    );
    bench_source(&mut criterion, "erc20", include_str!("../test/erc20.yul"));

    criterion.final_summary();
}
