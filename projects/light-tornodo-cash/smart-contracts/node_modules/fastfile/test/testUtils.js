export async function writeStringToFile(fd, str) {
    let buff = new Uint8Array(str.length + 1);
    for (let i = 0; i < str.length; i++) {
        buff[i] = str.charCodeAt(i);
    }
    buff[str.length] = 0;

    fd.write(buff);
}

export async function writeFakeStringToFile(fd, length) {
    let buff = new Uint8Array(length);
    for (let i = 0; i < length; i++) {
        buff[i] = 1;
    }
    await fd.write(buff);
}