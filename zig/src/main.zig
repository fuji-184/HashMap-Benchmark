const std = @import("std");

inline fn rdtscp() u64 {
    return asm volatile ("rdtscp"
        : [ret] "={ax}" (-> u64)
        :
        : "dx", "cx" 
    );
}

fn printFmt(label: []const u8, val: u64) void {
    var buf: [32]u8 = undefined;
    var temp = val;
    
    if (temp == 0) {
        std.debug.print("{s}: 0 cycles\n", .{label});
        return;
    }

    var i: usize = buf.len;
    var count: usize = 0;

    while (temp > 0) {
        if (count > 0 and count % 3 == 0) {
            i -= 1;
            buf[i] = '.';
        }
        i -= 1;
        buf[i] = @as(u8, @intCast(temp % 10)) + '0';
        temp /= 10;
        count += 1;
    }

    std.debug.print("{s}: {s} cycles\n", .{label, buf[i..]});
}

pub fn main() !void {
    var N: i32 = 0;

    const fd = std.posix.open("input.txt", .{}, 0) catch |err| {
        std.debug.print("Gagal membuka file input.txt: {}\n", .{err});
        return err;
    };
    defer std.posix.close(fd);

    var buf: [16]u8 = undefined;
    const bytes_read = try std.posix.read(fd, &buf);
    const trimmed = std.mem.trim(u8, buf[0..bytes_read], &std.ascii.whitespace);
    N = try std.fmt.parseInt(i32, trimmed, 10);

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const gpa_allocator = gpa.allocator();

    var get: i64 = 0;

    var m = std.AutoHashMap(i64, i64).init(gpa_allocator);
    defer m.deinit();

    const start_insert = rdtscp();
    var i: i64 = 0;
    while (i < N) : (i += 1) {
        try m.put(i, i);
    }
    const end_insert = rdtscp();

    const start_hit = rdtscp();
    i = 0;
    while (i < N) : (i += 1) {
        if (m.get(i)) |val| {
            get += val;
        }
    }
    const end_hit = rdtscp();

    var get2: i64 = 0;

    const start_miss = rdtscp();
    i = N;
    while (i < N * 2) : (i += 1) {
        if (m.get(i)) |val| {
            get2 += val;
        }
    }
    const end_miss = rdtscp();

    const start_update = rdtscp();
    i = 0;
    while (i < N) : (i += 1) {
        try m.put(i, i * 2);
    }
    const end_update = rdtscp();

    const start_delete = rdtscp();
    i = 0;
    while (i < N) : (i += 1) {
        _ = m.remove(i);
    }
    const end_delete = rdtscp();

    std.debug.print("N: {d}\n", .{N});
    std.debug.print("Get: {d}\n", .{get + get2});
    printFmt("Insert", end_insert - start_insert);
    printFmt("Get Hit", end_hit - start_hit);
    printFmt("Get Miss", end_miss - start_miss);
    printFmt("Update", end_update - start_update);
    printFmt("Delete", end_delete - start_delete);
}
