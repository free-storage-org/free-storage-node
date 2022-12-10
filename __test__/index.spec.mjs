import test from "ava";
import { FileId } from "../index.js";

test("`new` error", (t) => {
    let m = t.throws(() => new FileId()).message;

    t.is(m, "You cannot `new` a `FileId`. Use the `upload` function instead.");
});

test("invalid token", async (t) => {
    let m = await t
        .throwsAsync(() => FileId.upload("file", [], "test/test", "invalid"))
        .then((e) => e.message);

    t.is(m, "Invalid Repository OR The Token is Invalid");
});

test("getting a file", async (t) => {
    const file = await FileId.fromRaw(
        [86099156],
        "free-storage-org/test-storage"
    ).get();

    if (file.name.includes("API limit exeeded")) t.pass();

    t.is(file.name, "hello.txt");
    t.is(file.data.toString(), "Hello Tests!\n");
});
