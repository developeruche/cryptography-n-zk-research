import fs from "fs";
import * as testUtils from "./testUtils.js";
import * as fastFile from "../src/fastfile.js";

import assert from "assert";
import * as chai from "chai";
import chaiAsPromised from "chai-as-promised";

chai.use(chaiAsPromised);
const expect = chai.expect;

describe("fastfile testing suite for osfile", function () {
    let fileName = "test_osfile.bin";

    this.timeout(100000);

    let str1 = "0123456789";
    let str2 = "Hi_there";
    let str3 = "/!!--::**";

    it("should read valid strings from file", async () => {
        let fd = await fastFile.createOverride(fileName);

        await testUtils.writeFakeStringToFile(fd, fd.pageSize - 11);

        await testUtils.writeStringToFile(fd, str1);
        await testUtils.writeStringToFile(fd, str2);
        await testUtils.writeStringToFile(fd, str3);

        let str = await fd.readString(fd.pageSize - 11);
        assert.strictEqual(str, str1);

        str = await fd.readString();
        assert.strictEqual(str, str2);

        str = await fd.readString();
        assert.strictEqual(str, str3);

        await fd.close();

        assert(fs.existsSync(fileName));

        await fs.promises.unlink(fileName);
    });

    it("should throw error when try to read a closed file", async () => {
        let fd = await fastFile.createOverride(fileName);
        await fd.close();
        expect(fd.readString(0)).to.be.rejectedWith("Reading a closing file");
        await fs.promises.unlink(fileName);
    });
});




