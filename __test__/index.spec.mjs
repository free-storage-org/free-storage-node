import test from "ava";
import { FileId } from "../index.js";

test("`new` error", (t) => {
    t.is(
        t.throws(() => new FileId()).message,
        "You cannot `new` a `FileId`. Use the `upload` function instead."
    );
});

test("invalid token", async (t) => {
    t.is(
        await t
            .throwsAsync(() =>
                FileId.upload("file", [], "test/test", "invalid")
            )
            .then((e) => e.message),
        "Invalid Repository OR The Token is Invalid"
    );
});

test("getting a file", async (t) => {
    const file = await FileId.fromRaw(
        [86099156],
        "free-storage-org/test-storage"
    ).get();
    t.is(file.name, "hello.txt");
    t.is(file.data.toString(), "Hello Tests!\n");
});
