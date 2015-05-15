#![feature(str_char)]
#![feature(collections)]
//! A simple, stupid, unoptimized version of the Ordered Jobs Kata.
//! (Re)learning Rust at the same time. Back in my days we had ~["vectors", "like", "this"].

// TODO: The split between dep and job is iffy. #unify
// TODO: Organize in either "objects" (nah) or modules (yay)
// TODO: There's a lot of cloning going on. While "function", a lot of it is unnecessary.

#[derive(Debug, Clone)]
pub struct Job {
    name: char,
    dependency: Option<char>
}

impl PartialEq for Job {
    fn eq(&self, other: &Job) -> bool {
        self.name == other.name
    }
}

impl Job {
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
            _ => panic!(format!("Invalid Job spec format: {:?}", spec))
        }
    }
}

fn add_job(mut jobs: Vec<Job>, job: &Job) -> Vec<Job> {
    if !jobs.contains(job) {
        jobs.push(job.clone());
    }
    jobs.clone()
}

fn add_job_before(mut jobs: Vec<Job>, new_job: &Job, other_job: &Job) -> Vec<Job> {
    // This depends on our PartialEq checking only names, not deps - which doesn't feel quite
    // right.
    if let Some(i) = jobs.position_elem(other_job) {
        jobs.insert(i, new_job.clone());
    }
    jobs.clone()
}

fn add_dep(mut jobs: Vec<Job>, job: &Job, dep: &char) -> Result<Vec<Job>, &'static str> {
    if job.name == *dep {
        Err("Dependency on self")
    } else if job_name_exists(&jobs, &job.name) && job_name_exists(&jobs, dep) {
        Err("Circular job dependency")
    } else if job_name_exists(&jobs, &job.name) {
        Ok(add_job_before(jobs, &Job::new(*dep, None), job))
    } else {
        // Hmmm, composition anyone?
        jobs = add_job(jobs, &Job::new(*dep, None));
        jobs = add_job(jobs, job);
        Ok(jobs)
    }
}

fn job_name_exists(jobs: &Vec<Job>, name: &char) -> bool {
    jobs.iter().any(|s| s.name == *name)
}

pub struct JobList {
    jobs: Vec<Job>
}

impl JobList {
    pub fn from_jobs(input: Vec<Job>) -> Result<JobList, &'static str> {
        let jobs: Vec<Job> = Vec::with_capacity(input.len());

        let res = input.iter().fold(Ok(jobs), |acc, ref job| {
            acc.and_then(|jobs| {
                match job.dependency {
                    Some(dep) => add_dep(jobs, &job, &dep),
                    None      => Ok(add_job(jobs, &job))
                }
            })
        });

        res.map(|r| JobList { jobs: r })
    }
}

fn run(input: &str) -> Result<Vec<char>, &'static str> {
    let jobs: Vec<Job> = input.lines().map(Job::from_spec).collect();
    let jl: Result<JobList, &'static str> = JobList::from_jobs(jobs);

    jl.map(|j| j.jobs.iter().map(|j| j.name).collect())
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
    assert_eq!(res.unwrap(), vec!['a', 'c', 'b'])
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
    assert!(res.is_err())
}

#[test]
fn test_jobs_cannot_have_simple_circular_dependencies() {
    let res = run("a => b\
                 \nb => a");
    assert!(res.is_err())
}

#[test]
fn test_jobs_cannot_have_complex_circular_dependencies() {
    let res = run("a =>\
                 \nb => c\
                 \nc => f\
                 \nd => a\
                 \ne =>\
                 \nf => b");
    assert!(res.is_err())
}
