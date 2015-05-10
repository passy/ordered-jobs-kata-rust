#![feature(str_char)]
#![feature(collections)]
//! A simple, stupid, unoptimized version of the Ordered Jobs Kata.
//! (Re)learning Rust at the same time. Back in my days we had ~["vectors", "like", "this"].

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Job {
    name: char,
    dependency: Option<char>
}

impl Job {
    // XXX: Is this useful?
    pub fn new(name: char, dependency: Option<char>) -> Job {
        Job {
            name: name,
            dependency: dependency
        }
    }

    pub fn from_spec(spec: &str) -> Job {
        let splits: Vec<char> = spec
            .splitn(2, "=>")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.char_at(0))  // Generally unsafe but logically guarded by above stmt
            .collect();
        match splits.len() {
            1 => Job::new(splits[0], None),
            2 => Job::new(splits[0], Some(splits[1])),
            _ => panic!(format!("Invalid Job spec format: {}", spec))
        }
    }
}

pub struct JobList {
    jobs: Vec<Job>
}

fn add_job(job: &Job, mut jobs: Vec<Job>) -> Vec<Job> {
    if (!jobs.contains(job)) {
        jobs.push(job.clone());
    }
    jobs
}

fn add_job_before(new_job: &Job, other_job: &Job, mut jobs: Vec<Job>) -> Vec<Job> {
    if let Some(i) = jobs.position_elem(other_job) {
        jobs.insert(i, new_job.clone());
    }
    jobs
}

impl JobList {
    pub fn from_jobs(input: Vec<Job>) -> JobList {
        let mut jobs: Vec<Job> = Vec::with_capacity(input.len());

        // Boo... Let's refactor this, okay?
        for job in input.iter() {
            if (job.dependency.is_none()) {
                jobs = add_job(&(*job), jobs);
            }
        }

        JobList { jobs: input }
    }

    fn add_dep(job: &Job, dep: &Job, mut jobs: Vec<Job>) -> Result<Vec<Job>, &'static str> {
        if (job.name == dep.name) {
            Err("Dependency on self")
        } else if (jobs.contains(job) && jobs.contains(dep)) {
            Err("Circular job dependency")
        } else if (jobs.contains(job)) {
            Ok(add_job_before(dep, job, jobs))
        } else {
            // Hmmm, composition anyone?
            jobs = add_job(dep, jobs);
            jobs = add_job(job, jobs);
            Ok(jobs)
        }
    }
}

fn main() {
    println!("Hello, world!");
}

fn run(input: &str) -> Option<Vec<char>> {
    let jobs: Vec<Job> = input.lines().map(Job::from_spec).collect();
    println!("Jobs: {:?}", jobs);
    let jl: JobList = JobList::from_jobs(jobs);

    // To make it compile for now
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
