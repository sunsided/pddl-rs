// Stands in for VAL's `Validate -v domain problem plan` in the desktop suite.
// It prints test/val-output.txt, the output of the real VAL (planutils'
// `val Validate -v`) on problems/deliver.plan, whose second step fails, and
// exits 1 as VAL does for a failed plan.
const fs = require("fs");
const path = require("path");
const [, , flag, domain, problem, plan] = process.argv;
if (flag !== "-v" || !domain.endsWith("domain.pddl") || !problem.endsWith("deliver.pddl") || !plan.endsWith("deliver.plan")) {
  console.error(`unexpected arguments: ${process.argv.slice(2).join(" ")}`);
  process.exit(2);
}
process.stdout.write(fs.readFileSync(path.join(__dirname, "val-output.txt"), "utf8"));
process.exit(1);
