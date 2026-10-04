const std = @import("std");

extern "c" fn init() void;
extern "c" fn start() void;
extern "c" fn stop() void;
extern "c" fn print() void;
extern "c" fn reset() void;

const InputType = enum { String, Integer };

fn detectType(data: []const u8) InputType {
    if (data[8] == 1) return .Integer;
    return .String;
}

fn readStrings(allocator: std.mem.Allocator, data: []const u8) ![][]u8 {
    const n = std.mem.readInt(u64, data[0..8], .little);
    var result = try allocator.alloc([]u8, n);
    var pos: usize = 9;
    for (0..n) |idx| {
        const len = std.mem.readInt(u32, data[pos..][0..4], .little);
        pos += 4;
        const s = try allocator.dupe(u8, data[pos .. pos + len]);
        pos += len;
        result[idx] = s;
    }
    return result;
}

fn readIntegers(allocator: std.mem.Allocator, data: []const u8) ![]u64 {
    const n = std.mem.readInt(u64, data[0..8], .little);
    var result = try allocator.alloc(u64, n);
    var pos: usize = 9;
    for (0..n) |idx| {
        result[idx] = std.mem.readInt(u64, data[pos..][0..8], .little);
        pos += 8;
    }
    return result;
}

fn reverseString(allocator: std.mem.Allocator, s: []const u8) ![]u8 {
    const result = try allocator.dupe(u8, s);
    std.mem.reverse(u8, result);
    return result;
}

fn runBenchmarksString(allocator: std.mem.Allocator, keys: [][]u8) !void {
    const n = keys.len;
    var get: i64 = 0;
    var get2: i64 = 0;
    var m = std.StringHashMap(i64).init(allocator);
    defer m.deinit();

    std.debug.print("\n=== BENCHMARK: INSERT ===", .{});
    for (keys, 0..) |k, i| {
        start();
        try m.put(k, @intCast(i));
        stop();
    }
    print();
    reset();

    std.debug.print("\n=== BENCHMARK: GET HIT ===", .{});
    for (keys) |k| {
        start();
        const val = m.get(k);
        stop();
        if (val) |v| get += v;
    }
    print();
    reset();

    std.debug.print("\n=== BENCHMARK: GET MISS ===", .{});
    for (keys) |k| {
        const miss = try reverseString(allocator, k);
        defer allocator.free(miss);
        start();
        const val = m.get(miss);
        stop();
        if (val) |v| get2 += v;
    }
    print();
    reset();

    std.debug.print("\n=== BENCHMARK: RE-INSERT ===", .{});
    for (keys, 0..) |k, i| {
        const v: i64 = @as(i64, @intCast(i)) * 2;
        start();
        try m.put(k, v);
        stop();
    }
    print();
    reset();

    std.debug.print("\n=== BENCHMARK: REMOVE ===", .{});
    for (keys) |k| {
        start();
        _ = m.remove(k);
        stop();
    }
    print();
    reset();

    std.debug.print("\nN: {d}\nGet: {d}\n", .{ n, get - get2 });
}

fn runBenchmarksInteger(allocator: std.mem.Allocator, keys: []u64) !void {
    const n = keys.len;
    var get: i64 = 0;
    var get2: i64 = 0;
    var m = std.AutoHashMap(u64, i64).init(allocator);
    defer m.deinit();

    std.debug.print("\n=== BENCHMARK: INSERT ===", .{});
    for (keys, 0..) |k, i| {
        start();
        try m.put(k, @intCast(i));
        stop();
    }
    print();
    reset();

    std.debug.print("\n=== BENCHMARK: GET HIT ===", .{});
    for (keys) |k| {
        start();
        const val = m.get(k);
        stop();
        if (val) |v| get += v;
    }
    print();
    reset();

    std.debug.print("\n=== BENCHMARK: GET MISS ===", .{});
    for (keys) |k| {
        const miss = k + 1;
        start();
        const val = m.get(miss);
        stop();
        if (val) |v| get2 += v;
    }
    print();
    reset();

    std.debug.print("\n=== BENCHMARK: RE-INSERT ===", .{});
    for (keys, 0..) |k, i| {
        const v: i64 = @as(i64, @intCast(i)) * 2;
        start();
        try m.put(k, v);
        stop();
    }
    print();
    reset();

    std.debug.print("\n=== BENCHMARK: REMOVE ===", .{});
    for (keys) |k| {
        start();
        _ = m.remove(k);
        stop();
    }
    print();
    reset();

    std.debug.print("\nN: {d}\nGet: {d}\n", .{ n, get - get2 });
}

pub fn main(process_init: std.process.Init) !void {
    const io = process_init.io;
    const allocator = std.heap.smp_allocator;

    var read_buffer: [65536]u8 = undefined;
    const file = try std.Io.Dir.cwd().openFile(io, "../input.bin", .{});
    defer file.close(io);
    var fr = file.reader(io, &read_buffer);
    var buf = std.ArrayListUnmanaged(u8).empty;
    defer buf.deinit(allocator);
    try fr.interface.appendRemaining(allocator, &buf, .unlimited);
    const data = buf.items;

    init();

    switch (detectType(data)) {
        .Integer => {
            std.debug.print("[i] Detected: integer", .{});
            const keys = try readIntegers(allocator, data);
            defer allocator.free(keys);
            try runBenchmarksInteger(allocator, keys);
        },
        .String => {
            std.debug.print("[i] Detected: string", .{});
            const keys = try readStrings(allocator, data);
            defer {
                for (keys) |k| allocator.free(k);
                allocator.free(keys);
            }
            try runBenchmarksString(allocator, keys);
        },
    }
}