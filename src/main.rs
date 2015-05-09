fn main() {
    println!("Hello, world!");
}

fn run(input: &str) -> Option<Vec<char>> {
    let mut vec = Vec::new();
    vec.push('a');
    Some(vec)
}

#[test]
fn test_empty() {
    let res = run("");
    assert_eq!(res.unwrap(), vec![])
}

#[test]
fn test_single_job() {
    let res = run("a =>");
    assert_eq!(res.unwrap(), vec!['a'])
}

#[test]
fn test_multiple_jobs() {
    let res = run("a =>\
                 \nb =>\
                 \nc =>");
    assert_eq!(res.unwrap(), vec!['a', 'b', 'c'])
}

#[test]
fn test_multiple_jobs_single_dep() {
    let res = run("a =>\
                 \nb => c\
                 \nc =>");
    assert_eq!(res.unwrap(), vec!['a', 'b', 'c'])
}

#[test]
fn test_multiple_jobs_multiple_deps() {
    let res = run("a =>\
                 \nb => c\
                 \nc => f\
                 \nd => a\
                 \ne => b\
                 \nf =>");
    assert_eq!(res.unwrap(), vec!['a', 'f', 'c', 'b', 'd', 'e'])
}

#[test]
fn test_jobs_cannot_depend_on_themselves() {
    let res = run("a =>\
                 \nb =>\
                 \nc => c");
    assert!(res.is_none())
}

#[test]
fn test_jobs_cannot_have_circular_dependencies() {
    let res = run("a =>\
                 \nb => c\
                 \nc => f\
                 \nd => a\
                 \ne =>
                 \nf => b");
    assert!(res.is_none())
}
