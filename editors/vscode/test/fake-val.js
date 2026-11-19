// Stands in for VAL's `Validate -v domain problem plan` in the desktop suite:
// prints what VAL prints when the second step's precondition fails.
const [, , flag, domain, problem, plan] = process.argv;
if (flag !== "-v" || !domain.endsWith("domain.pddl") || !problem.endsWith("deliver.pddl") || !plan.endsWith("deliver.plan")) {
  console.error(`unexpected arguments: ${process.argv.slice(2).join(" ")}`);
  process.exit(2);
}
console.log(`Checking plan: ${plan}
Plan to validate:

Plan size: 2
1:
(drive t1 home home)
2:
(drive t1 home shop)

Plan Validation details
-----------------------

Checking next happening (time 1)
Checking next happening (time 2)
Plan failed because of unsatisfied precondition in:
(drive t1 home shop)
Plan failed to execute

Plan Repair Advice:

(drive t1 home shop) has an unsatisfied precondition at time 2
(Set (at t1 home) to true)

Failed plans:
 ${plan}`);
process.exit(1);
