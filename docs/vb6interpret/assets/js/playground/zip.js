/// Minimal ZIP archive writer (store method, no compression) for building a
/// downloadable snapshot of the playground's in-memory files without pulling
/// in an external dependency.

const CRC_TABLE = buildCrcTable();

function buildCrcTable() {
    const table = new Uint32Array(256);
    for (let n = 0; n < 256; n += 1) {
        let c = n;
        for (let k = 0; k < 8; k += 1) {
            c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
        }
        table[n] = c >>> 0;
    }
    return table;
}

function crc32(bytes) {
    let crc = 0xffffffff;
    for (let i = 0; i < bytes.length; i += 1) {
        crc = CRC_TABLE[(crc ^ bytes[i]) & 0xff] ^ (crc >>> 8);
    }
    return (crc ^ 0xffffffff) >>> 0;
}

function dosDateTime(date) {
    const time =
        ((date.getHours() & 0x1f) << 11) |
        ((date.getMinutes() & 0x3f) << 5) |
        ((date.getSeconds() >> 1) & 0x1f);
    const dosDate =
        (((date.getFullYear() - 1980) & 0x7f) << 9) |
        (((date.getMonth() + 1) & 0xf) << 5) |
        (date.getDate() & 0x1f);
    return { time, date: dosDate };
}

function writeUint16(view, offset, value) {
    view.setUint16(offset, value, true);
}

function writeUint32(view, offset, value) {
    view.setUint32(offset, value, true);
}

/// Build a ZIP archive (as a `Blob`) from `entries`, each `{ name, data }`
/// where `data` is a `Uint8Array` of the file's raw content.
export function createZip(entries) {
    const { time, date } = dosDateTime(new Date());
    const encoder = new TextEncoder();
    const localParts = [];
    const centralParts = [];
    let offset = 0;

    entries.forEach((entry) => {
        const nameBytes = encoder.encode(entry.name);
        const data = entry.data;
        const crc = crc32(data);

        const localHeader = new DataView(new ArrayBuffer(30));
        writeUint32(localHeader, 0, 0x04034b50);
        writeUint16(localHeader, 4, 20);
        writeUint16(localHeader, 6, 0);
        writeUint16(localHeader, 8, 0);
        writeUint16(localHeader, 10, time);
        writeUint16(localHeader, 12, date);
        writeUint32(localHeader, 14, crc);
        writeUint32(localHeader, 18, data.length);
        writeUint32(localHeader, 22, data.length);
        writeUint16(localHeader, 26, nameBytes.length);
        writeUint16(localHeader, 28, 0);

        localParts.push(new Uint8Array(localHeader.buffer), nameBytes, data);

        const centralHeader = new DataView(new ArrayBuffer(46));
        writeUint32(centralHeader, 0, 0x02014b50);
        writeUint16(centralHeader, 4, 20);
        writeUint16(centralHeader, 6, 20);
        writeUint16(centralHeader, 8, 0);
        writeUint16(centralHeader, 10, 0);
        writeUint16(centralHeader, 12, time);
        writeUint16(centralHeader, 14, date);
        writeUint32(centralHeader, 16, crc);
        writeUint32(centralHeader, 20, data.length);
        writeUint32(centralHeader, 24, data.length);
        writeUint16(centralHeader, 28, nameBytes.length);
        writeUint16(centralHeader, 30, 0);
        writeUint16(centralHeader, 32, 0);
        writeUint16(centralHeader, 34, 0);
        writeUint16(centralHeader, 36, 0);
        writeUint32(centralHeader, 38, 0);
        writeUint32(centralHeader, 42, offset);

        centralParts.push(new Uint8Array(centralHeader.buffer), nameBytes);

        offset += localHeader.byteLength + nameBytes.length + data.length;
    });

    const centralDirSize = centralParts.reduce((sum, part) => sum + part.length, 0);
    const centralDirOffset = offset;

    const endRecord = new DataView(new ArrayBuffer(22));
    writeUint32(endRecord, 0, 0x06054b50);
    writeUint16(endRecord, 4, 0);
    writeUint16(endRecord, 6, 0);
    writeUint16(endRecord, 8, entries.length);
    writeUint16(endRecord, 10, entries.length);
    writeUint32(endRecord, 12, centralDirSize);
    writeUint32(endRecord, 16, centralDirOffset);
    writeUint16(endRecord, 20, 0);

    return new Blob([...localParts, ...centralParts, new Uint8Array(endRecord.buffer)], {
        type: "application/zip",
    });
}
