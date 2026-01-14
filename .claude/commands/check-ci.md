# Check CI Failures

Debug CI failures for the current branch using the GitHub CLI.

## Instructions

1. First, check the CI status for the current PR:

   ```
   gh pr checks
   ```

2. For any failed checks, get the detailed logs:

   ```
   gh run view <run-id> --log-failed
   ```

3. Analyze the failure logs to identify:
   - The specific test(s) or step(s) that failed
   - The error messages and stack traces
   - Any patterns that suggest the root cause

4. Based on your analysis:
   - Explain what went wrong in clear terms
   - Suggest specific fixes for the failures
   - If the fix is straightforward, offer to implement it

5. If the CI is still running, report the current status and offer to check back later.

## Notes

- Focus on actionable insights rather than just reporting the error
- If multiple checks failed, prioritize them by importance (tests > lints > other)
- Consider if the failure might be flaky or infrastructure-related vs. a real code issue
