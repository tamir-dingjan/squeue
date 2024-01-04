# Squeue
A simple software queueing tool for Schrodinger Suite components.

## The Problem
The [Schrodinger, Inc. software suite]("https://newsite.schrodinger.com/") uses a job submission system to enqueue and then run jobs. However, it does not include a mechanism to control the maximum number of running jobs on the localhost - this is a feature that can only be specified for remote hosts. Running jobs will also check for available license/s up to a fixed timeout of 30 seconds, after which they will crash. There is also no feature to prevent a job launching in the absence of sufficient available licenses. This causes problems when working with a limited number of licenses on the localhost: the job controller will simply launch all submitted jobs, and once all the available licenses have been checked out, subsequently submitted jobs will simply fail.

## Possible solutions
- One solution is to enqueue jobs with the "-WAIT" flag, but this will force jobs to run one after another, which is definitely the slowest possible way to do it.
- Another way is to encode more complex job dependencies to allow as many jobs as can be licensed to be run in parallel, each of which is followed by dependent jobs, but this setup is not fault-tolerant: a failed job will cause a whole dependency chain to fail.

## This solution
A way to solve this while maintaining optimal performance is to enqueue the jobs via a program that can read the number of currently running jobs and pause the job submission until there are few enough running jobs. This approach prevents over-submission, while maintaining optimal throughput.

## Usage
Prepend the job command with the name of the binary, and mark with a "--" the desired maximum number of jobs to run:

`$ squeue $SCHRODINGER/component-to-run --5`

> [!NOTE]
> Because the program uses the double-dash flag to identify the desired maximum number of jobs to run simultaneously, it cannot be used to run programs which also rely on the double-dash flag to enter arguments. This will simply raise an error.
