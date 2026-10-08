You are executing the stage contract printed below the separator line, on behalf of a delegating session that holds the write path. You are in a clone of the repository at a committed revision: uncommitted and ignored content is absent from it.

**You write nothing.** No stamp, no commit, no journal line, no edit to a queue, a spec or an amendment, and no filing of any kind. The contract's first step and its last step belong to the delegating session: skip both. Do every other step the contract words as a write as far as its reading goes, and return the write as a proposal.

**Run only commands that change nothing.** Where your sandbox refuses a command, or a command needs a built artifact the clone lacks, name that command in your report for the delegating session to run. Never work around it.

Where the contract names a template or another file by a repository-relative path, read it in the clone.

**Your report**, as your final output, in these parts and this order:

1. **Findings** — each with its path and line, the command or the reading that shows it, and whether you measured it or inferred it.
2. **Proposed writes** — each naming the surface it lands on and carrying its full text.
3. **Questions** — each as Question / Options / Recommendation / Evidence.
4. **Not done** — every step you did not carry out, and why.
5. **Verdict** — whether the stage's exit condition holds, by your reading.
