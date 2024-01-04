
use std::{thread, time};
use std::{env, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    // let args = dbg!(args);

    // Get the job submission command and max jobs from the command line
    let (command, max_jobs) = parse_args(&args);
    // dbg!(max_jobs);

    let mut job_submitted: bool = false;

    while !job_submitted {
        // Get the currently running number of jobs according to Schrodinger's jobcontrol script

        let running_jobs = get_running_jobs();

        if running_jobs < max_jobs {
            // If the number of currently running jobs is lower than the threshold, submit the job
            let job = Command::new(&command[0])
            .args(&command[1..command.len()])
            .output()
            .expect("the user-provided command should be available and executable by the user");
        
        println!("{}", String::from_utf8(job.stdout).expect("should be able to encode the job stdout as text"));
        
        job_submitted = true
    } else {
            // If the number of currently running jobs is higher than the threshold, sleep
            thread::sleep(time::Duration::from_millis(10000))

        }

    }

    // println!("All done!");
}

fn parse_args(args: &Vec<String>) -> (Vec<String>, i32) {
    let command: Vec<String> = args[1..]
        .iter()
        .filter(|&x| !x.contains("--"))
        .cloned()
        .collect();

    // let command = dbg!(command);

    let job_spec: Vec<String> = args.iter().filter(|&x| x.contains("--")).cloned().collect();

    //let job_spec = dbg!(job_spec);

    if job_spec.len() > 1 {
        panic!("found more than a single specifier for the maximum number of jobs to run")
    } else if job_spec.len() == 0 {
        panic!("maximum number of jobs to run not specified")
    }

    let max_jobs: i32 = job_spec
        .concat()
        .trim_matches('-')
        .to_string()
        .parse()
        .unwrap_or(1);

    //let max_jobs = dbg!(max_jobs);

    // Catch case where the user entered "--0" for the job flag - this would run indefinitely
    if max_jobs == 0 {
        panic!("cannot run with a maximum number of jobs set to 0")
    }

    return (command, max_jobs);
}

fn get_running_jobs() -> i32 {

    let schrodinger_path = env::var("SCHRODINGER").expect("the SCHRODINGER environment variable should be set to the Schrodinger base directory");
    let jobcontrol_output = Command::new(&format!("{}/jobcontrol", schrodinger_path))
        .arg("-list")
        .arg("running")
        .output()
        .expect("the $SCHRODINGER/jobcontrol command should return the current running jobs");

    let output = String::from_utf8(jobcontrol_output.stdout).expect("should be able to parse the Schrodinger jobcontrol output as text");
    let lines: Vec<_> = output.split("\n").collect();

    // for line in &lines {
    //     println!("{}", line)
    // }
    
    let count = lines[0]
    .split(" ")
    .collect::<Vec<_>>()
    [1];

    // Need to handle the case where there are no currently running jobs
    let njobs:i32 = match count {
        "no" => 0,
        _ => count.parse().expect("should be able to parse the job count as a number"),
    };

    return njobs;
}
