Look at issues under [https://github.com/neu-lang/neu/issues](https://github.com/neu-lang/neu/issues), and start working on them one at a time. Once an issue is picked up, mark it as in progress.

For each independent issue, create a dedicated branch from `main`. Commit and push that issue's work only to its own branch, then raise a pull request back to `main`, ensuring the issue number is tagged. Once the pull request has all checks passing, let me know. Do not merge pull requests yourself. After the pull request is merged by someone else, remove its branch.

If the next issue depends on work from a previous issue, check out the previous issue's branch and add the dependent work there. Do not create disjoint branches for related work; keeping dependent changes together avoids merge conflicts.

Once a pull request is raised, don't wait for the status. Go ahead with the next issue.
