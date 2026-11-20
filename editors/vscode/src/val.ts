// Reads VAL's verbose output (`Validate -v domain problem plan`) into
// problems on the plan's steps. No `vscode` import, so it is plain logic.

/** A problem VAL found, on a 0-based line of the plan file. */
export interface ValProblem {
  line: number;
  message: string;
}

/** What VAL said about a plan. */
export interface ValResult {
  valid: boolean;
  problems: ValProblem[];
}

/** A plan step without case, spacing or a leading `time:`, e.g. `(drive t1 home shop)`. */
export function normalizeStep(text: string): string | undefined {
  const match = /\(([^()]*)\)/.exec(text.replace(/;.*$/, ""));
  return match ? `(${match[1].trim().split(/\s+/).join(" ").toLowerCase()})` : undefined;
}

/** The lines after `heading` up to the first line that is blank after some text. */
function section(lines: string[], heading: string): string[] {
  const start = lines.findIndex((line) => line.trim() === heading);
  if (start < 0) {
    return [];
  }
  const body: string[] = [];
  for (const line of lines.slice(start + 1)) {
    if (line.trim() === "") {
      if (body.length > 0) {
        break;
      }
      continue;
    }
    body.push(line.trim());
  }
  return body;
}

/**
 * Turns VAL's output into problems on `plan`'s lines. A failed precondition
 * marks its step, an unreached goal the last step, and output VAL gives no
 * step for marks the first line.
 */
export function parseVal(output: string, plan: string): ValResult {
  const lines = output.split(/\r?\n/);
  if (lines.some((line) => line.trim() === "Plan valid")) {
    return { valid: true, problems: [] };
  }
  const planLines = plan.split(/\r?\n/);
  const steps = planLines
    .map((text, line) => ({ line, step: normalizeStep(text) }))
    .filter((s): s is { line: number; step: string } => s.step !== undefined);
  const advice = section(lines, "Plan Repair Advice:");
  const withAdvice = (message: string) => (advice.length ? `${message}\n${advice.join("\n")}` : message);

  const failed = section(lines, "Plan failed because of unsatisfied precondition in:")[0];
  if (failed !== undefined) {
    const step = normalizeStep(failed);
    const at = steps.find((s) => s.step === step);
    return {
      valid: false,
      problems: [{ line: at?.line ?? 0, message: withAdvice(`VAL: unsatisfied precondition in ${failed}`) }],
    };
  }
  if (lines.some((line) => line.trim() === "Goal not satisfied")) {
    const last = steps[steps.length - 1];
    return {
      valid: false,
      problems: [{ line: last?.line ?? 0, message: withAdvice("VAL: the plan ends without reaching the goal") }],
    };
  }
  const text = lines.filter((line) => line.trim() !== "").slice(0, 20).join("\n");
  return { valid: false, problems: [{ line: 0, message: `VAL rejected the plan:\n${text}` }] };
}
