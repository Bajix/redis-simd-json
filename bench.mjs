import { Benchmark } from 'buffalo-bench';
import { get, set } from "./index.js";
import data from "./random.json" assert { type: "json" };
import Redis from "ioredis";

const client = new Redis();

let suite = new Benchmark.Suite("GET", {
  before: async () => {
    await set("random.json", JSON.stringify(data));

  },
  minSamples: 10
});

suite.add("redis-simd-json", async () => {
  await get("random.json")
});

suite.add("ioredis", async () => {
  JSON.parse(await client.get("random.json"));
});

await suite.run();

let result = suite.compareFastestWithSlowest('percent');

console.log(result.fastest.name + " is faster than " + result.slowest.name + " by " + result.by + "%"); 
