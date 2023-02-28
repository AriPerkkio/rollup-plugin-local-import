import { startVitest } from "vitest/node";

for (const [round] of Array(10_000).fill(null).entries()) {
  console.log("Round", round);

  await startVitest("test", ["test/"], {
    run: true,
    reporters: ["hanging-process", "verbose"],
  });

  if (process.exitCode) process.exit();
}
