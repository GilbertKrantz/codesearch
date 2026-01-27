use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use codesearch::parser::{CodeParser, RustParser, PythonParser, JavaScriptParser, GoParser, JavaParser};

const RUST_CODE: &str = r#"
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    pub fn distance(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

pub fn main() {
    let p = Point::new(3.0, 4.0);
    println!("Distance: {}", p.distance());
}
"#;

const PYTHON_CODE: &str = r#"
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
    
    def distance(self):
        return (self.x ** 2 + self.y ** 2) ** 0.5

def main():
    p = Point(3.0, 4.0)
    print(f"Distance: {p.distance()}")

if __name__ == "__main__":
    main()
"#;

const JAVASCRIPT_CODE: &str = r#"
class Point {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
    
    distance() {
        return Math.sqrt(this.x * this.x + this.y * this.y);
    }
}

function main() {
    const p = new Point(3.0, 4.0);
    console.log(`Distance: ${p.distance()}`);
}

main();
"#;

const GO_CODE: &str = r#"
package main

import (
    "fmt"
    "math"
)

type Point struct {
    x float64
    y float64
}

func (p *Point) Distance() float64 {
    return math.Sqrt(p.x*p.x + p.y*p.y)
}

func main() {
    p := &Point{x: 3.0, y: 4.0}
    fmt.Printf("Distance: %f\n", p.Distance())
}
"#;

const JAVA_CODE: &str = r#"
public class Point {
    private double x;
    private double y;
    
    public Point(double x, double y) {
        this.x = x;
        this.y = y;
    }
    
    public double distance() {
        return Math.sqrt(x * x + y * y);
    }
    
    public static void main(String[] args) {
        Point p = new Point(3.0, 4.0);
        System.out.println("Distance: " + p.distance());
    }
}
"#;

fn bench_rust_parser(c: &mut Criterion) {
    let parser = RustParser;
    c.bench_function("rust_parser_parse_content", |b| {
        b.iter(|| parser.parse_content(black_box(RUST_CODE)))
    });
    
    c.bench_function("rust_parser_extract_functions", |b| {
        b.iter(|| parser.extract_functions(black_box(RUST_CODE)))
    });
    
    c.bench_function("rust_parser_extract_classes", |b| {
        b.iter(|| parser.extract_classes(black_box(RUST_CODE)))
    });
}

fn bench_python_parser(c: &mut Criterion) {
    let parser = PythonParser;
    c.bench_function("python_parser_parse_content", |b| {
        b.iter(|| parser.parse_content(black_box(PYTHON_CODE)))
    });
    
    c.bench_function("python_parser_extract_functions", |b| {
        b.iter(|| parser.extract_functions(black_box(PYTHON_CODE)))
    });
    
    c.bench_function("python_parser_extract_classes", |b| {
        b.iter(|| parser.extract_classes(black_box(PYTHON_CODE)))
    });
}

fn bench_javascript_parser(c: &mut Criterion) {
    let parser = JavaScriptParser;
    c.bench_function("javascript_parser_parse_content", |b| {
        b.iter(|| parser.parse_content(black_box(JAVASCRIPT_CODE)))
    });
    
    c.bench_function("javascript_parser_extract_functions", |b| {
        b.iter(|| parser.extract_functions(black_box(JAVASCRIPT_CODE)))
    });
    
    c.bench_function("javascript_parser_extract_classes", |b| {
        b.iter(|| parser.extract_classes(black_box(JAVASCRIPT_CODE)))
    });
}

fn bench_go_parser(c: &mut Criterion) {
    let parser = GoParser;
    c.bench_function("go_parser_parse_content", |b| {
        b.iter(|| parser.parse_content(black_box(GO_CODE)))
    });
    
    c.bench_function("go_parser_extract_functions", |b| {
        b.iter(|| parser.extract_functions(black_box(GO_CODE)))
    });
    
    c.bench_function("go_parser_extract_classes", |b| {
        b.iter(|| parser.extract_classes(black_box(GO_CODE)))
    });
}

fn bench_java_parser(c: &mut Criterion) {
    let parser = JavaParser;
    c.bench_function("java_parser_parse_content", |b| {
        b.iter(|| parser.parse_content(black_box(JAVA_CODE)))
    });
    
    c.bench_function("java_parser_extract_functions", |b| {
        b.iter(|| parser.extract_functions(black_box(JAVA_CODE)))
    });
    
    c.bench_function("java_parser_extract_classes", |b| {
        b.iter(|| parser.extract_classes(black_box(JAVA_CODE)))
    });
}

fn bench_all_parsers_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_comparison");
    
    let parsers: Vec<(&str, Box<dyn CodeParser>, &str)> = vec![
        ("rust", Box::new(RustParser), RUST_CODE),
        ("python", Box::new(PythonParser), PYTHON_CODE),
        ("javascript", Box::new(JavaScriptParser), JAVASCRIPT_CODE),
        ("go", Box::new(GoParser), GO_CODE),
        ("java", Box::new(JavaParser), JAVA_CODE),
    ];
    
    for (name, parser, code) in parsers {
        group.bench_with_input(BenchmarkId::new("parse_content", name), &code, |b, code| {
            b.iter(|| parser.parse_content(black_box(code)))
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_rust_parser,
    bench_python_parser,
    bench_javascript_parser,
    bench_go_parser,
    bench_java_parser,
    bench_all_parsers_comparison
);
criterion_main!(benches);
