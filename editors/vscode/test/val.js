// Checks how src/val.ts reads VAL's output, in plain Node: `node test/val.js`.
// run.js runs it first, so `yarn run test` covers it too.

const assert = require("assert");
const fs = require("fs");
const path = require("path");
const esbuild = require("esbuild");

// val.ts imports nothing, so a bundle of it loads without VS Code.
const [bundle] = esbuild.buildSync({
  entryPoints: [path.join(__dirname, "..", "src", "val.ts")],
  bundle: true,
  format: "cjs",
  platform: "node",
  write: false,
}).outputFiles;
const val = { exports: {} };
new Function("module", "exports", bundle.text)(val, val.exports);
const { parseVal } = val.exports;

// The recorded output of the real VAL: the second step of deliver.plan fails.
const recorded = fs.readFileSync(path.join(__dirname, "val-output.txt"), "utf8");
const deliver = fs.readFileSync(path.join(__dirname, "workspace", "problems", "deliver.plan"), "utf8");
const failed = parseVal(recorded, deliver);
assert.strictEqual(failed.valid, false);
assert.deepStrictEqual(failed.problems.map((p) => p.line), [2]);
assert.ok(failed.problems[0].message.startsWith("VAL: unsatisfied precondition in (drive t1 shop home)"));

/** VAL's output for a plan whose step at `time` fails on `step`. */
const failing = (step, time) => [
  "Plan Validation details",
  "-----------------------",
  "",
  ...Array.from({ length: time - 1 }, (_, i) => [`Checking next happening (time ${i + 1})`, ""]).flat(),
  `Checking next happening (time ${time})`,
  "Plan failed because of unsatisfied precondition in:",
  step,
  "",
  "Plan failed to execute",
].join("\n");

// A step that repeats an earlier one is marked, not the earlier one.
const repeated = "(move a b)\n(move b a)\n(move a b)\n";
assert.deepStrictEqual(parseVal(failing("(move a b)", 3), repeated).problems.map((p) => p.line), [2]);
assert.deepStrictEqual(parseVal(failing("(move a b)", 1), repeated).problems.map((p) => p.line), [0]);

// A timed plan: the step that started last at or before the failing happening.
const timed = "0.000: (move a b) [1]\n1.001: (move b a) [1]\n2.002: (move a b) [1]\n";
const late = failing("(move a b)", 1).replace("(time 1)", "(time 2.002)");
assert.deepStrictEqual(parseVal(late, timed).problems.map((p) => p.line), [2]);

assert.deepStrictEqual(parseVal("Plan valid\n", repeated), { valid: true, problems: [] });
console.log("val.ts: all checks passed");
