#!/usr/bin/env python3
"""Craft the Cloud Glass shaped recipes in a real crafting table.

Called by tools/e2e.sh with the open game's control socket. Uses the harness's
crafting grid (`ctl screen`: slots 100-108 row-major, result 200), gives the
ingredients, and asserts:

1. four `minecraft:orange_wool` around one `minecraft:glass` -> four
   `cloudglass:orange_cloud_glass` (the upstream ` W `/`WGW`/` W ` recipe),
   and the crafted stack reaches the inventory;
2. four `minecraft:stick` around one glass -> an empty result (the wool
   recipe's control);
3. eight `cloudglass:white_cloud_glass` around one `minecraft:light_blue_dye`
   -> eight `cloudglass:light_blue_cloud_glass` (the upstream dye recipe);
4. eight white cloud glass around a stick -> an empty result.

The table is replaced before every case, so a failed/untaken result cannot
leak into the next grid. Exits 1 with the observed result on the first
mismatch.
"""
import json
import subprocess
import sys

CROSS = [101, 103, 105, 107]
CENTRE = 104
RING = [100, 101, 102, 103, 105, 106, 107, 108]


def main():
    sock, ctl, x, y, z = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4], sys.argv[5]

    def run(*args):
        out = subprocess.run([ctl, "--socket", sock, *args], text=True, capture_output=True)
        try:
            return json.loads(out.stdout)
        except json.JSONDecodeError:
            raise SystemExit(f"ctl {' '.join(args)}: {out.stdout}{out.stderr}")

    def wait(ticks=4):
        run("wait", str(ticks))

    def slots():
        screen = run("screen")
        if "slots" not in screen:
            raise SystemExit(f"no menu is open: {screen}")
        return screen["slots"]

    def find(item):
        for slot in slots():
            if slot.get("item") == item:
                return slot["slot"]
        raise SystemExit(f"missing ingredient {item} in the inventory")

    def open_table():
        wait()
        print("debug interact:", run("interact", x, y, z))
        wait(5)

    def result():
        """The result slot's stack, or None: an empty slot has `item: null`."""
        stack = [s for s in slots() if s["slot"] == 200]
        if not stack or not stack[0].get("item"):
            return None
        return stack[0]

    def place(item, mode, targets):
        slot = find(item)
        run("click-slot", str(slot))
        wait(1)
        for target in targets:
            if mode == "right":
                run("click-slot", str(target), "right")
            else:
                run("click-slot", str(target))
            wait(1)
        run("click-slot", str(slot))
        wait(2)

    def setup(items):
        try:
            run("close")
        except SystemExit:
            pass
        wait(5)
        # Replace the table so its block entity, and therefore the crafting
        # grid, is fresh for this case.
        run("setblock", x, y, z, "minecraft:air")
        run("setblock", x, y, z, "minecraft:crafting_table")
        wait(2)
        print("debug tp:", run("command", f"tp @s {float(x) - 0.5} {y} {float(z) + 0.5}"))
        wait(3)
        run("command", "clear @s")
        for item, count in items:
            run("give", item, str(count))
        wait(3)
        open_table()

    failures = []

    def check(label, items, placements, want_item, want_count):
        setup(items)
        for item, mode, targets in placements:
            place(item, mode, targets)
        got = result()
        print(f"{label} ->", json.dumps(got))
        if not got or got.get("item") != want_item or got.get("count") != want_count:
            failures.append(f"{label}: expected {want_count} {want_item}, got {got}")
            return None
        return got

    def empty_control(label, items, placements):
        setup(items)
        for item, mode, targets in placements:
            place(item, mode, targets)
        got = result()
        print(f"{label} ->", json.dumps(got))
        if got is not None:
            failures.append(f"{label}: control crafted {got['item']}")

    # 1. Upstream's wool + glass recipe, with an inventory hand-off.
    setup([("minecraft:orange_wool", 4), ("minecraft:glass", 1)])
    place("minecraft:orange_wool", "right", [str(slot) for slot in CROSS])
    place("minecraft:glass", "right", [str(CENTRE)])
    got = result()
    print("4 orange wool + glass ->", json.dumps(got))
    if not got or got.get("item") != "cloudglass:orange_cloud_glass" or got.get("count") != 4:
        failures.append("the orange wool + glass recipe did not make 4 blocks")
    else:
        run("click-slot", "200")
        wait(5)
        run("click-slot", "9")
        wait(3)
        made = run("inventory")["counts"].get("cloudglass:orange_cloud_glass", 0)
        if made < 4:
            failures.append(f"the crafted glass reached the inventory as {made}, not 4")

    # 2. The wool recipe's deliberately unmatched control.
    empty_control(
        "4 sticks + glass",
        [("minecraft:stick", 4), ("minecraft:glass", 1)],
        [
            ("minecraft:stick", "right", [str(slot) for slot in CROSS]),
            ("minecraft:glass", "right", [str(CENTRE)]),
        ],
    )

    # 3. The dye recipe: eight white cloud glass around one dye.
    check(
        "8 white cloud glass + light_blue dye",
        [("cloudglass:white_cloud_glass", 8), ("minecraft:light_blue_dye", 1)],
        [
            ("cloudglass:white_cloud_glass", "right", [str(slot) for slot in RING]),
            ("minecraft:light_blue_dye", "right", [str(CENTRE)]),
        ],
        "cloudglass:light_blue_cloud_glass",
        8,
    )

    # 4. The dye recipe's control.
    empty_control(
        "8 white cloud glass + stick",
        [("cloudglass:white_cloud_glass", 8), ("minecraft:stick", 1)],
        [
            ("cloudglass:white_cloud_glass", "right", [str(slot) for slot in RING]),
            ("minecraft:stick", "right", [str(CENTRE)]),
        ],
    )

    run("close")
    wait(3)
    if failures:
        for failure in failures:
            print(f"FAIL: {failure}", file=sys.stderr)
        raise SystemExit(1)
    print(
        "recipe PASS: wool + glass makes 4, white cloud glass + dye makes 8, "
        "and both stick controls leave the result empty"
    )


if __name__ == "__main__":
    main()
