# Performance engineering

Performance changes often trade straightforward code for a specialized implementation. A faster
result in one benchmark does not show that the trade is worthwhile.

## Why performance work requires evidence

Performance problems are difficult to identify from source code alone. Operations that look
expensive may be insignificant in a complete application, while costs caused by allocation,
conversion, cache behavior, terminal output, or repeated work may be difficult to see during
review.

Unmeasured optimization tends to produce recurring problems:

- **The wrong problem is optimized.** A locally expensive operation may contribute little to frame
  time or application latency.
- **The workload is not representative.** A microbenchmark, worst-case example, or synthetic input
  may behave differently from ordinary Ratatui applications.
- **Noise is mistaken for improvement.** Short runs, different environments, and single samples can
  make normal variance look like a useful result.
- **Relative numbers hide insignificant changes.** A large percentage improvement to a tiny
  operation may save too little time to matter to users.
- **Costs move rather than disappear.** A change may improve one input size, platform, rendering
  pattern, or memory metric while making another worse.
- **Correctness changes accidentally.** Data structure, caching, and unsafe-code optimizations can
  alter behavior in ways that a timing benchmark does not detect.
- **Complexity remains after the benefit disappears.** Specialized representations, additional
  branches, unsafe code, and new abstractions must still be maintained if later changes make the
  original optimization irrelevant.
- **Implementation choices become public constraints.** An optimization can affect public types,
  trait implementations, feature flags, serialization, or downstream integration.

These costs are borne by every future contributor. The performance benefit is received only by
users whose workloads exercise the improved path. The evidence therefore needs to show that the
benefit is real, relevant, and large enough to pay for the lasting cost of the change.

A result that disproves an optimization hypothesis is still useful. In
[PR #231](https://github.com/ratatui/ratatui/pull/231), a string representation that appeared likely
to improve locality instead produced a measured 20-40% regression. Closing the proposal was the
right outcome because the measurements changed the decision.

## Performance tuning process

Use the following process for performance changes:

1. **Define the problem.** Identify the user-visible workload, the affected operation, how often it
   occurs, and what improvement would be meaningful. Be specific about whether the concern is
   latency, throughput, allocations, memory use, startup time, or binary size.
1. **Choose representative cases.** Include ordinary usage and any important boundary or stress
   cases. Record the data, dimensions, iteration counts, target, and other inputs.
1. **Establish a baseline.** Measure the current implementation and determine its normal variance
   before making changes.
1. **Locate the cost.** Use counters, tracing, phase timing, allocation instrumentation, or a
   profiler to confirm where the relevant resources are being spent.
1. **State the hypothesis.** Explain the suspected cause, the mechanism by which the proposed change
   should help, and which measurement should change if the hypothesis is correct.
1. **Test one idea at a time.** Keep the experiment separate from refactoring, dependency updates,
   and unrelated behavioral changes so the result can be attributed to that idea.
1. **Verify correctness.** Characterize existing behavior and run the relevant tests before treating
   any timing result as useful.
1. **Compare equivalent measurements.** Use the same workload, profile, features, toolchain, and
   environment. Run timing benchmarks sequentially and retain the raw samples.
1. **Check for displaced costs.** Compare relevant input sizes, rendering patterns, platforms, and
   memory or compilation behavior rather than reporting only the best case.
1. **Evaluate the tradeoff.** Decide whether the absolute benefit is meaningful and whether it pays
   for the added complexity, API constraints, maintenance burden, and portability risk.
1. **Keep or revert the experiment.** Do not keep speculative complexity when the evidence is
   inconclusive. Record useful negative results so later contributors do not repeat the work.

Do not decide whether to keep a change from one short benchmark run. Repeat measurements, inspect
their variance, and investigate results that change significantly between runs.

## Choosing measurement techniques

Different tools answer different questions:

- **Is the path exercised often enough to matter?** Use counters, tracing, or instrumentation.
  Retain call counts and workload parameters.
- **Which phase is expensive?** Use phase timers or application telemetry. Retain the absolute time
  spent in each phase.
- **Where is CPU time spent?** Use a sampling profiler such as `samply`, `perf`, or
  `cargo flamegraph`. Retain the profile or flamegraph and identify the important symbols.
- **Is a small operation faster?** Use Criterion or another statistical benchmark harness. Retain
  the samples, distributions, variance, and absolute times.
- **Is the complete workload faster?** Use an application benchmark or a tool such as `hyperfine`.
  Retain repeated wall, user, and system times.
- **Did allocation behavior improve?** Use allocator instrumentation, DHAT, or a heap profiler.
  Retain allocation counts, allocated bytes, peak memory, and retained memory.
- **Did generated code change as expected?** Use `cargo asm`, LLVM output, or disassembly. Retain the
  relevant code-generation difference and explain why it matters.

Start with the cheapest technique that can answer the current question. Move to more detailed tools
only when the existing evidence leaves a specific uncertainty.

Profilers locate costs. Benchmarks compare alternatives. Tests verify behavior. None substitutes for
the others.

## Recording the experiment

A performance pull request should record:

- the user-visible problem and representative workload;
- the optimization hypothesis and expected mechanism;
- the baseline and candidate revisions;
- exact commands, build profile, enabled features, and tool versions;
- hardware, operating system, Rust toolchain, target, and relevant environmental conditions;
- corpus, terminal dimensions, iteration counts, and other workload parameters;
- raw samples or exported reports;
- variance and repeated-run behavior;
- absolute measurements as well as relative changes;
- correctness checks;
- cases that improved, regressed, or did not change;
- failed approaches that affected the final design;
- effects on readability, complexity, public APIs, portability, compilation, and maintenance; and
- why the measured benefit is worth those costs.

Do not reduce the result to a single percentage. For example, report both the percentage change and
the time saved per frame, then relate that saving to how frequently the operation occurs in the
representative workload.

## Making the decision

A performance change is worth keeping when:

- the affected workload matters to Ratatui users;
- the result is repeatable and larger than normal measurement noise;
- the absolute improvement is meaningful at the frequency with which the operation occurs;
- important workloads or targets do not regress unacceptably;
- behavior and compatibility remain correct;
- the implementation remains understandable and testable; and
- the benefit justifies the complexity and maintenance burden it introduces.

Following this process avoids asking the project and its users to carry permanent costs in exchange
for a benefit that has not been demonstrated.
